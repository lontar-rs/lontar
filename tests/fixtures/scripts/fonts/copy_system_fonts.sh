#!/usr/bin/env bash
# Copy Noto fonts from system to test fixtures directory

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FONTS_DIR="$SCRIPT_DIR"

echo "Copying Noto fonts to $FONTS_DIR"

# Noto Sans Balinese
if [ -f "/usr/share/fonts/truetype/noto/NotoSansBalinese-Regular.ttf" ]; then
    cp /usr/share/fonts/truetype/noto/NotoSansBalinese-Regular.ttf "$FONTS_DIR/"
    echo "✓ Copied NotoSansBalinese-Regular.ttf"
else
    echo "✗ NotoSansBalinese-Regular.ttf not found in system fonts"
fi

# Noto Sans Devanagari
if [ -f "/usr/share/fonts/truetype/noto/NotoSansDevanagari-Regular.ttf" ]; then
    cp /usr/share/fonts/truetype/noto/NotoSansDevanagari-Regular.ttf "$FONTS_DIR/"
    echo "✓ Copied NotoSansDevanagari-Regular.ttf"
else
    echo "✗ NotoSansDevanagari-Regular.ttf not found in system fonts"
fi

# Noto Sans Arabic
if [ -f "/usr/share/fonts/truetype/noto/NotoSansArabic-Regular.ttf" ]; then
    cp /usr/share/fonts/truetype/noto/NotoSansArabic-Regular.ttf "$FONTS_DIR/"
    echo "✓ Copied NotoSansArabic-Regular.ttf"
else
    echo "✗ NotoSansArabic-Regular.ttf not found in system fonts"
fi

# Noto Sans CJK SC (Simplified Chinese)
if [ -f "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc" ]; then
    cp /usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc "$FONTS_DIR/"
    echo "✓ Copied NotoSansCJK-Regular.ttc"
elif [ -f "/usr/share/fonts/truetype/noto/NotoSansSC-Regular.otf" ]; then
    cp /usr/share/fonts/truetype/noto/NotoSansSC-Regular.otf "$FONTS_DIR/"
    echo "✓ Copied NotoSansSC-Regular.otf"
else
    echo "✗ Noto Sans CJK not found in system fonts"
    echo "  Install with: sudo apt-get install fonts-noto-cjk"
fi

echo ""
echo "Font copy complete!"
echo "Fonts are now available in: $FONTS_DIR"
