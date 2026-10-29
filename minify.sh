#!/usr/bin/env bash
#
# Usage:
#   ./minify.sh [options] <source_dir>
#
# Options:
#   -o, --out <dir>   Output directory (default: dist)
#   -i, --install     Install any missing npm packages globally
#   -f, --force       Overwrite the output directory if it already exists
#   -h, --help        Show this help

set -euo pipefail

if [[ -t 1 ]]; then
  RED=$'\e[31m'; GREEN=$'\e[32m'; YELLOW=$'\e[33m'; BLUE=$'\e[34m'; BOLD=$'\e[1m'; RESET=$'\e[0m'
else
  RED=""; GREEN=""; YELLOW=""; BLUE=""; BOLD=""; RESET=""
fi

info()  { echo "${BLUE}==>${RESET} $*"; }
ok()    { echo "${GREEN}  ✔${RESET} $*"; }
warn()  { echo "${YELLOW}  !${RESET} $*" >&2; }
error() { echo "${RED}  ✘${RESET} $*" >&2; }
die()   { error "$*"; exit 1; }

usage() { sed -n '3,17p' "$0" | sed 's/^# \{0,1\}//'; }

SRC=""
OUT=""
INSTALL=false
FORCE=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    -o|--out)     [[ $# -ge 2 ]] || die "$1 requires a directory"; OUT="$2"; shift 2 ;;
    -i|--install) INSTALL=true; shift ;;
    -f|--force)   FORCE=true; shift ;;
    -h|--help)    usage; exit 0 ;;
    -*)           die "Unknown option: $1 (see --help)" ;;
    *)            [[ -z "$SRC" ]] || die "Only one source directory allowed"; SRC="$1"; shift ;;
  esac
done

[[ -n "$SRC" ]] || { usage; exit 1; }
[[ -d "$SRC" ]] || die "Source directory not found: $SRC"
SRC="${SRC%/}"
OUT="${OUT:-dist}"
OUT="${OUT%/}"

info "Checking for npm..."
if ! command -v npm >/dev/null 2>&1; then
  error "npm is not installed or not on your PATH."
  echo "    Install Node.js (which includes npm) from https://nodejs.org" >&2
  exit 1
fi
ok "npm $(npm --version) found"

PACKAGES=(
  "html-minifier-terser:html-minifier-terser"
  "terser:terser"
  "lightningcss-cli:lightningcss"
)

info "Checking required tools..."
MISSING=()
for entry in "${PACKAGES[@]}"; do
  pkg="${entry%%:*}"
  bin="${entry##*:}"
  if command -v "$bin" >/dev/null 2>&1; then
    ok "$pkg"
  else
    warn "$pkg is not installed (command '$bin' not found)"
    MISSING+=("$pkg")
  fi
done

if [[ ${#MISSING[@]} -gt 0 ]]; then
  if $INSTALL; then
    info "Installing missing packages: ${MISSING[*]}"
    if ! npm install -g "${MISSING[@]}"; then
      error "npm install failed."
      echo "    If this is a permissions error, see:" >&2
      echo "    https://docs.npmjs.com/resolving-eacces-permissions-errors-when-installing-packages-globally" >&2
      exit 1
    fi

    for entry in "${PACKAGES[@]}"; do
      bin="${entry##*:}"
      command -v "$bin" >/dev/null 2>&1 \
        || die "'$bin' still not found after install. Is npm's global bin dir ($(npm prefix -g)/bin) on your PATH?"
    done
    ok "All packages installed"
  else
    echo >&2
    error "Missing ${#MISSING[@]} required package(s)."
    echo "    Install them with:" >&2
    echo "      ${BOLD}npm install -g ${MISSING[*]}${RESET}" >&2
    echo "    or re-run this script with --install." >&2
    exit 1
  fi
fi

SRC_ABS="$(cd "$SRC" && pwd)"
if [[ -e "$OUT" ]]; then
  OUT_ABS="$(cd "$OUT" && pwd)"
  [[ "$OUT_ABS" != "$SRC_ABS" ]] || die "Output directory must differ from source directory"
  $FORCE || die "Output directory '$OUT' already exists (use --force to overwrite)"
  info "Removing existing output directory '$OUT'"
  rm -rf -- "$OUT"
fi

info "Copying '$SRC' to '$OUT'..."
cp -R -- "$SRC" "$OUT"
find "$OUT" -name '.DS_Store' -delete

# ---------- helpers ----------
dir_bytes() { find "$1" -type f -exec cat {} + 2>/dev/null | wc -c | tr -d ' '; }

human() {
  awk -v b="$1" 'BEGIN {
    split("B KB MB GB", u); i = 1
    while (b >= 1024 && i < 4) { b /= 1024; i++ }
    printf (i == 1 ? "%d %s" : "%.1f %s"), b, u[i]
  }'
}

FAILED=0
COUNT=0

minify_file() {
  local file="$1"; shift
  local tmp="${file}.minify-tmp"
  local cmd=("${@//__IN__/$file}")
  cmd=("${cmd[@]//__OUT__/$tmp}")

  if "${cmd[@]}" >/dev/null 2>"${tmp}.err" && [[ -s "$tmp" ]]; then
    mv -- "$tmp" "$file"
    COUNT=$((COUNT + 1))
  else
    warn "Failed, kept original: ${file#"$OUT"/}"
    sed 's/^/      /' "${tmp}.err" | head -n 5 >&2 || true
    rm -f -- "$tmp"
    FAILED=$((FAILED + 1))
  fi
  rm -f -- "${tmp}.err"
}

BEFORE=$(dir_bytes "$OUT")

info "Minifying HTML..."
while IFS= read -r -d '' f; do
  minify_file "$f" html-minifier-terser \
    --collapse-whitespace \
    --remove-comments \
    --remove-script-type-attributes \
    --remove-style-link-type-attributes \
    --minify-js true --minify-css true \
    -o __OUT__ __IN__
done < <(find "$OUT" -type f \( -name '*.html' -o -name '*.htm' \) -print0)

info "Minifying JavaScript..."
while IFS= read -r -d '' f; do
  minify_file "$f" terser __IN__ --compress --mangle -o __OUT__
done < <(find "$OUT" -type f \( -name '*.js' -o -name '*.mjs' \) ! -name '*.min.js' -print0)

info "Minifying CSS..."
while IFS= read -r -d '' f; do
  minify_file "$f" lightningcss --minify __IN__ -o __OUT__
done < <(find "$OUT" -type f -name '*.css' ! -name '*.min.css' -print0)

AFTER=$(dir_bytes "$OUT")

echo
info "${BOLD}Done.${RESET}"
ok "Minified $COUNT file(s) into '$OUT'"
if [[ "$BEFORE" -gt 0 ]]; then
  SAVED=$((BEFORE - AFTER))
  PCT=$(awk -v s="$SAVED" -v b="$BEFORE" 'BEGIN { printf "%.1f", 100 * s / b }')
  ok "Total size: $(human "$BEFORE") → $(human "$AFTER") (saved $(human "$SAVED"), ${PCT}%)"
fi
if [[ $FAILED -gt 0 ]]; then
  warn "$FAILED file(s) could not be minified and were left unchanged"
  exit 2
fi