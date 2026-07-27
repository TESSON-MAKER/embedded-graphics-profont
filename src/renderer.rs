//! Low-level text rendering algorithms.
//!
//! This module provides the core rendering functions that power the `Text` and `Character`
//! drawables. It handles the actual unpacking and drawing of bitmap glyphs.

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::PixelColor,
    prelude::*,
    Pixel,
};

use crate::font::Font;

/// Render a single character using transparent mode.
///
/// In transparent mode, only the pixels set in the glyph bitmap are drawn.
/// The background pixels remain untouched, allowing overlapping text and complex layouts.
///
/// # Arguments
///
/// * `target` - The draw target to render to
/// * `ch` - Character to render
/// * `position` - Top-left position where the character should be drawn
/// * `font` - Font definition to use
/// * `text_color` - Color for the text pixels
///
/// # Returns
///
/// Returns the width of the rendered character, or 0 if the character is not
/// supported by the font.
///
/// # Rendering Algorithm
///
/// 1. Lookup the glyph in the font's lookup table
/// 2. Extract the glyph's bitmap data from the packed binary buffer
/// 3. Iterate over each bit in the bitmap
/// 4. For each set bit, create a pixel at the appropriate coordinate
/// 5. Draw all pixels to the target using `DrawTarget::draw_iter()`
pub fn draw_char<D, C>(
    target: &mut D,
    ch: char,
    position: Point,
    font: &Font,
    text_color: C,
) -> Result<u32, D::Error>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    if let Some(entry) = font.get_glyph(ch) {
        let bytes_per_row = (entry.width as usize + 7) / 8;
        let glyph_bytes = font.glyph_data(entry);

        let pixels = (0..font.max_height).flat_map(|row| {
            let actual_row = font.max_height as usize - 1 - row as usize;
            let row_offset = actual_row * bytes_per_row;

            (0..entry.width).filter_map(move |col| {
                let byte_index = row_offset + (col as usize / 8);
                let bit_index = 7 - (col % 8);
                let is_set = (glyph_bytes[byte_index] & (1 << bit_index)) != 0;

                if is_set {
                    Some(Pixel(
                        Point::new(position.x + col as i32, position.y + row as i32),
                        text_color,
                    ))
                } else {
                    None
                }
            })
        });

        target.draw_iter(pixels)?;
        Ok(entry.width as u32)
    } else {
        Ok(0)
    }
}

/// Render a string using transparent mode with tracking support.
///
/// Renders each character of the string in sequence, advancing the cursor
/// position by the character width plus any configured tracking.
///
/// # Arguments
///
/// * `target` - The draw target to render to
/// * `text` - String to render
/// * `position` - Starting position for the first character (top-left)
/// * `font` - Font definition to use
/// * `text_color` - Color for the text pixels
/// * `tracking` - Extra spacing to add between characters in pixels
///
/// # Returns
///
/// Returns the position where the next character would be drawn (i.e., after the last
/// character plus any trailing tracking). This is useful for continuing text on the
/// same baseline.
///
/// # Unsupported Characters
///
/// Characters not in the font's supported range are silently skipped (contribute 0 width).
///
/// # Examples
///
/// ```ignore
/// use embedded_graphics::geometry::Point;
/// use embedded_graphics_profont::renderer;
///
/// // Draw text with 2-pixel spacing between characters
/// let end_pos = renderer::draw_str(
///     &mut target,
///     "Hello",
///     Point::new(10, 10),
///     &font,
///     color,
///     2,
/// )?;
///
/// // Continue on the same line
/// renderer::draw_str(
///     &mut target,
///     " World",
///     end_pos,
///     &font,
///     color,
///     2,
/// )?;
/// ```
pub fn draw_str<D, C>(
    target: &mut D,
    text: &str,
    position: Point,
    font: &Font,
    text_color: C,
    tracking: i32,
) -> Result<Point, D::Error>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    let mut cursor = position;

    for c in text.chars() {
        let width = draw_char(target, c, cursor, font, text_color)?;
        cursor.x += width as i32 + tracking;
    }

    Ok(cursor)
}