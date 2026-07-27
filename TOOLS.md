# Font Generation Tools

Complete workflow for creating and converting bitmap fonts for use with `embedded-graphics-profont`.

## Workflow Overview

```
1. Font Creation/Selection
   ↓
2. Export from GLCD Font Creator (.c format)
   ↓
3. Convert with main.py
   ↓
4. Use in Rust project
```

## Step 1: GLCD Font Creator

**GLCD Font Creator** is a bitmap font editor designed for embedded displays.

### Download & Install

- **Website:** https://www.mikroe.com/glcd-font-creator
- **Platform:** Windows (standalone executable) and Linux with Wine
- **Cost:** Free

### Creating/Editing Fonts

1. **Open GLCD Font Creator**
2. Create new font or open existing font file
3. Design/edit glyphs as needed
4. Adjust font parameters (height, baseline, etc.)

### Exporting Font

**Critical:** Correct export format is essential.

1. Click **Export for TFT and new GLCD**
2. Set export option:
   - **Format:** MikroC (TFT)
3. Save file (e.g., `myfont.c`)

**Wrong format will not work with conversion script!**

## Step 2: Convert with main.py

### Prerequisites

```bash
# Ensure Python 3.6+ is installed
python3 --version
```

### Conversion Script

The `main.py` script converts GLCD-exported `.c` files to Rust constants.

**Location:** Project root or fonts directory

### Usage

```bash
python3 main.py input_font.c -o output_fonts.rs
```

**Options:**
- `-o, --output` - Output file name (default: `fonts.rs`)
- `-n, --name` - Font constant name (default: auto from filename)

### Example

```bash
# Convert myfont.c → fonts.rs
python3 main.py myfont.c

# Custom output name
python3 main.py myfont.c -o my_custom_fonts.rs

# Custom constant name
python3 main.py myfont.c -n "ARIAL_8"
```

### Script Workflow

The conversion script:

1. **Parses** the GLCD C export file
2. **Extracts** glyph bitmap data
3. **Generates** Rust constants:
   - `GLYPHS` - Array of GlyphEntry (width, offset)
   - `FONT_DATA` - Binary bitmap data
   - `FONT` - Const Font instance
4. **Writes** Rust source file

### Output Example

```rust
// Generated fonts.rs
use embedded_graphics_profont::GlyphEntry;

pub const GLYPHS: [GlyphEntry; 95] = [
    GlyphEntry { width: 5, offset: 0 },
    GlyphEntry { width: 4, offset: 40 },
    // ... more entries
];

pub const FONT_DATA: [u8; 4000] = [
    0x60, 0x90, 0xF0, 0x90,  // Glyph data
    // ... more data
];

pub const FONT: Font = Font::new(
    &GLYPHS,
    &FONT_DATA,
    32,    // ASCII start
    126,   // ASCII end
    8,     // Height
    false, // Monospace
);
```

## Step 3: Integrate into Project

### Option A: Direct Copy

1. Run conversion script to generate `fonts.rs`
2. Place `fonts.rs` in `src/fonts/` directory
3. Use in code:

```rust
// In main.rs
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
};

use embedded_graphics_profont::{
    Anchor, 
    Text, 
    WithAnchor
};

mod fonts;
use fonts::D_DIN41X44 as D_DIN;

// In main loop
let center = Point::new(100, 100);

Text::new("test", center, &D_DIN, Rgb666::BLACK)
    .with_anchor(Anchor::MiddleLeft)
    .draw(&mut display).ok();
```

### Option B: Multiple Fonts

Generate multiple fonts and organize:

```bash
# Create separate files in src/fonts/
python3 main.py arial.c -o src/fonts/arial.rs
python3 main.py courier.c -o src/fonts/courier.rs
python3 main.py gothic.c -o src/fonts/gothic.rs
```

Then in code:

```rust
// In main.rs
mod fonts;

use fonts::arial::FONT as ARIAL_FONT;
use fonts::courier::FONT as COURIER_FONT;

// Use as needed
Text::new("Arial Text", Point::new(0, 0), &ARIAL_FONT, Rgb666::BLACK)
    .draw(&mut display).ok();

Text::new("Courier Text", Point::new(0, 20), &COURIER_FONT, Rgb666::BLACK)
    .draw(&mut display).ok();
```

## Troubleshooting

### "Script fails to parse .c file"

**Cause:** Wrong export format from GLCD Font Creator

**Fix:**
- Verify you used "Export for tft" with MikroC format
- Check file contains `#include <stdint.h>` at top
- Ensure file has `uint8_t` data arrays (not other formats)

### "Generated font looks wrong"

**Cause:** Glyph dimensions mismatch

**Fix:**
- Check GLCD export height matches `max_height` in generated code
- Verify monospace/proportional flag matches font type
- Test with simpler font first

### "Text doesn't render"

**Cause:** Font data not included or character out of range

**Fix:**
```rust
// Debug: Check font properties
defmt::info!("Font height: {}", font.max_height);
defmt::info!("Range: {}-{}", font.ascii_begin, font.ascii_end);
defmt::info!("Can render 'A': {}", font.get_glyph('A').is_some());
```

### "Characters are missing"

**Cause:** ASCII range mismatch between exported range and what's needed

**Fix:**
- Regenerate font with extended range
- Or check if character is printable ASCII (32-126)

## Font Recommendations

### Small Fonts (8-10px)
- Good for system/status text
- Limited detail, still readable
- Small generated code size

### Medium Fonts (12-14px)
- Balanced readability and size
- Good for UI labels
- Recommended for most applications

### Large Fonts (16-24px)
- High readability
- Larger code/data footprint
- Use for titles or critical information

## Complete Workflow Example

```bash
# 1. Export from GLCD Font Creator as myfont.c
# 2. Convert to Rust
python3 main.py myfont.c -o src/fonts/myfont.rs

# 3. In Rust code (main.rs)
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
};
use embedded_graphics_profont::{Text, Anchor, WithAnchor};

mod fonts;
use fonts::FONT;

// Render centered text in main loop
let center = Point::new(160, 120);
Text::new("Hello STM32", center, &FONT, Rgb666::BLACK)
    .with_anchor(Anchor::MiddleCenter)
    .draw(&mut display).ok();
```

## Best Practices

1. **Keep font files versioned** - Track `.c` and generated `.rs` files
2. **Test fonts early** - Verify rendered output before committing
3. **Use standard sizes** - 8, 12, 16px for consistency
4. **Document font purpose** - Comment what each font is for
5. **Minimize font count** - Use 1-2 fonts per project when possible
