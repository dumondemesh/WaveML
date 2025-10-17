use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

const MAGIC: [u8; 4] = *b"WMLB";
const ALIGN: u64 = 16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest(pub serde_json::Value);

impl Manifest {
    /// Create a minimal manifest with header fields required by v0.3
    pub fn minimal(domain: &str) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        let mut map = BTreeMap::<String, serde_json::Value>::new();
        map.insert("version".into(), "0.3.0".into());
        map.insert("schema_semver".into(), "0.3.0".into());
        map.insert("domain".into(), domain.into());
        map.insert("endianness".into(), "little".into());
        map.insert("timebase".into(), serde_json::json!({"unit":"s", "tick":1}));
        map.insert("profiles".into(), serde_json::json!(["WF-Core"]));
        map.insert("created_at".into(), now.into());
        Manifest(serde_json::Value::Object(map.into_iter().collect()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub name: String,
    pub offset: u64,
    pub size: u64,
    pub sha256: [u8; 32],
    pub codec_id: u16, // 0=raw
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Index {
    pub entries: Vec<IndexEntry>,
}

fn align16(pos: u64) -> u64 {
    pos.div_ceil(ALIGN) * ALIGN
}

/// JCS-like canonicalization: recursively sort object keys lexicographically.
pub fn to_canonical_json(value: &serde_json::Value) -> String {
    fn sort_value(v: &serde_json::Value) -> serde_json::Value {
        match v {
            serde_json::Value::Object(map) => {
                let mut bmap = BTreeMap::<String, serde_json::Value>::new();
                for (k, vv) in map {
                    bmap.insert(k.clone(), sort_value(vv));
                }
                serde_json::Value::Object(bmap.into_iter().collect())
            }
            serde_json::Value::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(sort_value).collect())
            }
            _ => v.clone(),
        }
    }
    let v = sort_value(value);
    serde_json::to_string(&v).expect("canonical json")
}

/// Pack a WMLB file per v0.3 layout: HEADER | MANIFEST | INDEX | BLOBS
pub fn pack_wmlb<P: AsRef<Path>>(
    out_path: P,
    manifest: &Manifest,
    mut blobs: Vec<(String, Vec<u8>, u16)>, // (name, data, codec_id)
) -> Result<()> {
    blobs.shrink_to_fit();
    let mut f = File::create(&out_path)?;

    // 1) HEADER
    f.write_all(&MAGIC)?;                 // magic[4]
    f.write_all(&0u16.to_le_bytes())?;    // major=0
    f.write_all(&3u16.to_le_bytes())?;    // minor=3
    f.write_all(&0u32.to_le_bytes())?;    // flags
    f.write_all(&[0u8])?;                 // endianness: little
    f.write_all(&[0u8; 7])?;              // reserved

    // 2) MANIFEST (len:u64 + bytes)
    let manifest_str = to_canonical_json(&manifest.0);
    let manifest_bytes = manifest_str.as_bytes();
    f.write_all(&(manifest_bytes.len() as u64).to_le_bytes())?;
    f.write_all(manifest_bytes)?;

    // 3) Compute INDEX size to know where BLOBS start
    let mut index_size: u64 = 4; // blob_count:u32
    for (name, data, _codec) in &blobs {
        let _ = data;
        index_size += 2; // name_len
        index_size += name.len() as u64;
        index_size += 8 + 8; // offset + size
        index_size += 32;    // sha256
        index_size += 2;     // codec_id
    }
    let index_start = f.stream_position()?;
    let blobs_start = align16(index_start + index_size);

    // Precompute entries
    let mut entries: Vec<IndexEntry> = Vec::with_capacity(blobs.len());
    let mut cur_offset = blobs_start;
    for (name, data, codec_id) in &blobs {
        let aligned = align16(cur_offset);
        if aligned != cur_offset {
            cur_offset = aligned;
        }
        let sha: [u8; 32] = Sha256::digest(data).into();
        let entry = IndexEntry {
            name: name.clone(),
            offset: cur_offset,
            size: data.len() as u64,
            sha256: sha,
            codec_id: *codec_id,
        };
        entries.push(entry);
        cur_offset += data.len() as u64;
    }

    // 3) Write INDEX
    f.write_all(&(entries.len() as u32).to_le_bytes())?;
    for e in &entries {
        let name_bytes = e.name.as_bytes();
        let name_len = u16::try_from(name_bytes.len()).map_err(|_| anyhow!("blob name too long"))?;
        f.write_all(&name_len.to_le_bytes())?;
        f.write_all(name_bytes)?;
        f.write_all(&e.offset.to_le_bytes())?;
        f.write_all(&e.size.to_le_bytes())?;
        f.write_all(&e.sha256)?;
        f.write_all(&e.codec_id.to_le_bytes())?;
    }

    // Pad to blobs_start
    let after_index = f.stream_position()?;
    if after_index > blobs_start {
        return Err(anyhow!("INDEX overrun; accounting error"));
    }
    let pad = (blobs_start - after_index) as usize;
    if pad > 0 {
        f.write_all(&vec![0u8; pad])?;
    }

    // 4) Write BLOBS aligned to 16
    for (i, (_name, data, _codec)) in blobs.into_iter().enumerate() {
        let want = entries[i].offset;
        let cur = f.stream_position()?;
        if cur < want {
            let pad = (want - cur) as usize;
            f.write_all(&vec![0u8; pad])?;
        }
        f.write_all(&data)?;
    }

    Ok(())
}

/// Read and parse a WMLB, returning manifest and index; blobs are lazy via read_blob.
#[derive(Debug)]
pub struct WmlbReader<R: Read + Seek> {
    inner: R,
    pub manifest: Manifest,
    pub index: Index,
    _blobs_base: u64,
}

impl<R: Read + Seek> WmlbReader<R> {
    pub fn open(mut inner: R) -> Result<Self> {
        let mut magic = [0u8; 4];
        inner.read_exact(&mut magic)?;
        if magic != MAGIC {
            return Err(anyhow!("bad magic"));
        }
        let mut u16buf = [0u8; 2];
        let mut u32buf = [0u8; 4];
        let mut u8buf = [0u8; 1];
        inner.read_exact(&mut u16buf)?; // major
        inner.read_exact(&mut u16buf)?; // minor
        inner.read_exact(&mut u32buf)?; // flags
        inner.read_exact(&mut u8buf)?;  // endianness
        let mut reserved = [0u8; 7];
        inner.read_exact(&mut reserved)?;

        // manifest
        let mut u64buf = [0u8; 8];
        inner.read_exact(&mut u64buf)?;
        let manifest_len = u64::from_le_bytes(u64buf) as usize;
        let mut mbytes = vec![0u8; manifest_len];
        inner.read_exact(&mut mbytes)?;
        let manifest_json: serde_json::Value = serde_json::from_slice(&mbytes)?;
        let manifest = Manifest(manifest_json);

        // index
        inner.read_exact(&mut u32buf)?;
        let count = u32::from_le_bytes(u32buf) as usize;
        let mut entries = Vec::with_capacity(count);
        for _ in 0..count {
            let mut nbuf = [0u8; 2];
            inner.read_exact(&mut nbuf)?;
            let name_len = u16::from_le_bytes(nbuf) as usize;
            let mut name_bytes = vec![0u8; name_len];
            inner.read_exact(&mut name_bytes)?;
            let name = String::from_utf8(name_bytes)?;
            inner.read_exact(&mut u64buf)?;
            let offset = u64::from_le_bytes(u64buf);
            inner.read_exact(&mut u64buf)?;
            let size = u64::from_le_bytes(u64buf);
            let mut sha = [0u8; 32];
            inner.read_exact(&mut sha)?;
            inner.read_exact(&mut nbuf)?;
            let codec_id = u16::from_le_bytes(nbuf);
            entries.push(IndexEntry { name, offset, size, sha256: sha, codec_id });
        }
        let pos = inner.stream_position()?;
        let blobs_base = pos; // after index
        Ok(Self { inner, manifest, index: Index { entries }, _blobs_base: blobs_base })
    }

    pub fn read_blob(&mut self, name: &str) -> Result<Vec<u8>> {
        let e = self.index.entries.iter().find(|e| e.name == name)
            .ok_or_else(|| anyhow!("no such blob: {}", name))?;
        self.inner.seek(SeekFrom::Start(e.offset))?;
        let mut buf = vec![0u8; e.size as usize];
        self.inner.read_exact(&mut buf)?;
        // verify sha256
        let sha: [u8; 32] = Sha256::digest(&buf).into();
        if sha != e.sha256 {
            return Err(anyhow!("sha256 mismatch for blob {}", name));
        }
        Ok(buf)
    }
}
