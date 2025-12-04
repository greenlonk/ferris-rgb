#!/usr/bin/env bash
set -e

echo "=== Rust Analyzer Environment Check ==="

echo ""
echo "1) Checking rustc location:"
if command -v rustc >/dev/null 2>&1; then
    RUSTC=$(command -v rustc)
    echo "   ✓ rustc found at: $RUSTC"
else
    echo "   ✗ rustc NOT found in PATH — rust-analyzer cannot function!"
    exit 1
fi

echo ""
echo "2) Checking cargo:"
if command -v cargo >/dev/null 2>&1; then
    echo "   ✓ cargo found"
else
    echo "   ✗ cargo missing — RA will fail to load workspace metadata"
fi

echo ""
echo "3) Checking rustc sysroot:"
SYSROOT=$(rustc --print sysroot 2>/dev/null || echo "ERR")
echo "   sysroot: $SYSROOT"

if [ "$SYSROOT" = "ERR" ]; then
    echo "   ✗ Cannot read sysroot — RA cannot find std library"
fi

echo ""
echo "4) Checking for rust-src inside sysroot:"
if [ -d "$SYSROOT/lib/rustlib/src/rust/library" ]; then
    echo "   ✓ rust-src present (std available)"
else
    echo "   ✗ rust-src NOT found — Go-to-definition on std & many symbols will NOT work"
    echo "     (solution: add rust-src via rust-overlay or Nix flake)"
fi

echo ""
echo "5) Checking Nix toolchain visibility:"
if [[ "$RUSTC" == /nix/store/* ]]; then
    echo "   ✓ rustc comes from Nix store"
else
    echo "   ✗ rustc NOT from Nix — VS Code may be using wrong toolchain"
fi

echo ""
echo "6) Checking if direnv has applied env:"
if [[ -n "$DIRENV_WATCHES" ]]; then
    echo "   ✓ direnv active"
else
    echo "   ✗ direnv NOT active — VS Code likely started RA too early"
fi

echo ""
echo "=== Summary ==="

IFS=$'\n'
issues=()

if ! command -v rustc >/dev/null 2>&1; then issues+=("rustc missing"); fi
if ! command -v cargo >/dev/null 2>&1; then issues+=("cargo missing"); fi
if [ ! -d "$SYSROOT/lib/rustlib/src/rust/library" ]; then issues+=("rust-src missing"); fi
if [[ -z "$DIRENV_WATCHES" ]]; then issues+=("direnv not loaded at RA startup"); fi

if [ ${#issues[@]} -eq 0 ]; then
    echo "   ✓ No issues detected — rust-analyzer SHOULD work"
else
    echo "   ✗ Potential problems:"
    for x in "${issues[@]}"; do echo "      - $x"; done
fi