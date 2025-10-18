#!/usr/bin/env zsh
set -euo pipefail

# === УКАЖИ СВОИ ФАЙЛЫ/ПАПКИ ТУТ (относительно директории скрипта) ===
# Если при запуске переданы аргументы — они ПЕРЕПИШУТ этот список.
INCLUDE=(
"acceptance"
"crates"
"Makefile"
"build"
"docs"
"modules"
"tests"
"examples"
"README.md"       
"tools"
"Cargo.toml"
"fix"
"rust-toolchain.toml"
"ci"          
"make_waveml_zip.zsh" 
"scripts"
)

# Папка, где лежит скрипт
SCRIPT_DIR="$(cd -- "$(dirname -- "$0")" && pwd)"
cd "$SCRIPT_DIR"

# Имя архива с датой/временем (без двоеточий)
ts="$(date +"%Y-%m-%d_%H-%M-%S")"
OUT="WaveML_${ts}.zip"

# Что архивировать: аргументы -> иначе INCLUDE
typeset -a SELECTED TO_ZIP
if (( $# > 0 )); then
  SELECTED=("$@")
else
  SELECTED=("${INCLUDE[@]}")
fi

if (( ${#SELECTED[@]} == 0 )); then
  echo "Нет путей для архивации. Укажи их в массиве INCLUDE или передай аргументами:"
  echo "  $0 file1 dir2 'path with spaces/file.txt'"
  exit 1
fi

# Проверка существования путей
for p in "${SELECTED[@]}"; do
  if [[ -e "$p" ]]; then
    TO_ZIP+=("$p")
  else
    echo "Предупреждение: '$p' не найдено в ${SCRIPT_DIR} — пропускаю." >&2
  fi
done

if (( ${#TO_ZIP[@]} == 0 )); then
  echo "Нечего архивировать — все пути отсутствуют."
  exit 1
fi

# Создание zip:
# -r рекурсивно, -9 максимум сжатия, -X без лишних атрибутов macOS,
# -y хранить симлинки как симлинки, -x исключения-мусор
zip -r -9 -X -y "$OUT" "${TO_ZIP[@]}" \
  -x "*/.DS_Store" "*/__pycache__/*" "*/node_modules/*" "*/.git/*"

echo "Готово: ${SCRIPT_DIR}/${OUT}"

