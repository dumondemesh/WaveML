// Re-export all public items from the v1 WFR module without spelling them out
// to avoid name drift between this facade and the canonical definitions.
// This keeps the API stable for dependents (wavectl, runners, etc.).
#![allow(clippy::module_inception)]

pub mod wfr_v1;
pub use wfr_v1::*;
