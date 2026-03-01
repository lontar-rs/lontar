#!/usr/bin/env bash
set -euo pipefail

# Generate PDF fixtures from docs/latex_examples/*.tex
# Outputs PDFs to tests/fixtures/reference_docs/

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
EXAMPLES_DIR="$ROOT_DIR/docs/latex_examples"
OUT_DIR="$ROOT_DIR/tests/fixtures/reference_docs"

mkdir -p "$OUT_DIR"

# Use xelatex by default
LATEX="xelatex"
BIBER="biber"

compile_tex() {
  local texfile=$1
  local basename
  basename=$(basename "$texfile" .tex)
  echo "==> Building $basename.pdf"
  pushd "$EXAMPLES_DIR" >/dev/null
  # First pass
  $LATEX -interaction=nonstopmode "$basename.tex" >/dev/null
  # Run biber if .bcf exists (citations)
  if [[ -f "$basename.bcf" ]]; then
    $BIBER "$basename" >/dev/null || true
  fi
  # Second and third passes for refs/TOC
  $LATEX -interaction=nonstopmode "$basename.tex" >/dev/null
  $LATEX -interaction=nonstopmode "$basename.tex" >/dev/null
  popd >/dev/null
  mv "$EXAMPLES_DIR/$basename.pdf" "$OUT_DIR/$basename.pdf"
}

for tex in "$EXAMPLES_DIR"/*.tex; do
  compile_tex "$tex"
done

echo "PDF fixtures generated in $OUT_DIR"
