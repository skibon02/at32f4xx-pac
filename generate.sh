#!/usr/bin/env bash
set -euo pipefail

# Bash port of generate.ps1.
# Inputs:
#   PACKs/Keil5/*.pack  - vendor SVDs (newer/authoritative; firmware library has no SVDs)
#   patches/*.yaml      - per-chip svdtools patches, plus shared patches/common/**
# Tools required on PATH: svd2rust, svdtools, form, cargo, rustfmt, unzip, sed
# Provided by the flake devShell (`nix develop`).

export RUST_LOG=warn
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

JOBS="${JOBS:-$(nproc)}"

mkdir -p svd

extract_pack() {
    local pack="$1"
    local base
    base="$(basename "$pack" .pack)"
    local tmp="$ROOT/.gen-tmp/$base"
    rm -rf "$tmp"
    mkdir -p "$tmp"
    unzip -q -o "$pack" -d "$tmp"
    if [ -d "$tmp/SVD" ]; then
        local svd
        for svd in "$tmp/SVD"/*.svd; do
            [ -e "$svd" ] || continue
            local name
            name="$(basename "$svd")"
            name="${name//xx_v2/}"
            name="${name,,}"
            cp -f "$svd" "$ROOT/svd/$name"
        done
    fi
    rm -rf "$tmp"
}

export -f extract_pack
export ROOT

echo "[1/5] Extracting SVDs from PACKs..."
find PACKs/Keil5 -maxdepth 1 -name '*.pack' -print0 |
    xargs -0 -P"$JOBS" -I{} bash -c 'extract_pack "$@"' _ {}

echo "[2/5] Patching SVDs (svdtools)..."
find patches -maxdepth 1 -name '*.yaml' -print0 |
    xargs -0 -P"$JOBS" -I{} svdtools patch {}

# exit # early exit if we are testing svd patching only

generate_chip() {
    set -euo pipefail
    local svd_patched="$1"
    local fname
    fname="$(basename "$svd_patched")"          # at32f435.svd.patched
    local stem="${fname%.svd.patched}"           # at32f435
    local dir_name="${stem,,}"                   # at32f435
    local svd_name="${dir_name}.svd"             # at32f435.svd

    if [ -z "$dir_name" ] || [ -z "$stem" ]; then
        echo "FATAL: empty dir_name from '$svd_patched'" >&2
        return 1
    fi

    local work="$ROOT/.gen-tmp/$dir_name"
    rm -rf "$work"
    mkdir -p "$work"
    cp "$svd_patched" "$work/$svd_name"
    pushd "$work" >/dev/null

    cargo init -q
    if ! svd2rust -m -g -s \
            --edition 2024 \
            --reexport-core-peripherals --reexport-interrupt \
            -f peripheral::c: -f peripheral_singleton::c: -f register::c: -f register_spec::c:_SPEC -f field_reader::c:_R -f field_writer::c:_W -f interrupt::c: -f enum_name::c:_A -f enum_value::p: \
            --max_cluster_size \
            --atomics --atomics_feature atomics \
            --impl_debug --impl_defmt defmt\
            -l warn -i "$svd_name"; then
        popd >/dev/null
        echo "svd2rust failed for $dir_name" >&2
        return 1
    fi

    form -i mod.rs -o src/
    rm -f mod.rs
    cargo fmt
    cp "$svd_name" src/
    cp device.x src/
    mv src/lib.rs src/mod.rs

    popd >/dev/null

    local target="$ROOT/src/$dir_name"
    case "$target" in
        "$ROOT/src/"|"$ROOT/src"|"$ROOT/"|"$ROOT") echo "FATAL: refusing to rm '$target'" >&2; return 1 ;;
    esac
    rm -rf "$target"
    cp "$work/generic.rs" "$ROOT/src/"
    rm -f "$work/generic.rs"
    cp -r "$work/src" "$target"
    rm -rf "$work"
}

export -f generate_chip

echo "[3/5] Running svd2rust + form per chip..."
mkdir -p src
find svd -maxdepth 1 -name '*.svd.patched' -print0 |
    xargs -0 -P"$JOBS" -I{} bash -c 'generate_chip "$@"' _ {}

echo "[4/5] Generating HTML docs..."
mapfile -t patched_svds < <(find svd -maxdepth 1 -name '*.svd.patched')
mapfile -t orig_svds    < <(find svd -maxdepth 1 -name '*.svd' ! -name '*.patched')
mkdir -p html html/original
svdtools html ./html "${patched_svds[@]}" >/dev/null
svdtools html ./html/original "${orig_svds[@]}" >/dev/null
svdtools htmlcompare ./html/comparison "${patched_svds[@]}" >/dev/null
svdtools htmlcompare ./html/original/comparison "${orig_svds[@]}" >/dev/null
sed -i 's|comparisons.html|comparison/index.html|g' \
    ./html/index.html ./html/original/index.html

echo "[5/5] Cleaning up..."
rm -rf .gen-tmp

echo "Done."
