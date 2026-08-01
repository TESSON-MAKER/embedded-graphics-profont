use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    pixelcolor::PixelColor,
    Drawable,
};

use crate::renderer;

/// Text anchor point for positioning.
///
/// Defines which point of the text bounding box should be placed at the given position.
/// This allows precise control over text alignment without calculating dimensions manually.
///
/// # Examples
///
/// - `TopLeft`: Position is at top-left corner (default)
/// - `MiddleCenter`: Position is at the center of the text
/// - `BottomRight`: Position is at bottom-right corner
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    /// Position text so the given point is at its top-left corner.
    TopLeft,
    /// Position text so the given point is at its top-center.
    TopCenter,
    /// Position text so the given point is at its top-right corner.
    TopRight,
    /// Position text so the given point is at its middle-left.
    MiddleLeft,
    /// Position text so the given point is at its center.
    MiddleCenter,
    /// Position text so the given point is at its middle-right.
    MiddleRight,
    /// Position text so the given point is at its bottom-left corner.
    BottomLeft,
    /// Position text so the given point is at its bottom-center.
    BottomCenter,
    /// Position text so the given point is at its bottom-right corner.
    BottomRight,
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::TopLeft
    }
}

/// Trait for types that support anchoring.
///
/// Allows configuring the anchor point for text and character positioning.
///
/// # Examples
///
/// ```ignore
/// use embedded_graphics_profont::{Text, Anchor, WithAnchor};
/// use embedded_graphics::geometry::Point;
///
/// let text = Text::new("Hello", Point::new(100, 100), &font, color)
///     .with_anchor(Anchor::MiddleCenter);
/// ```
pub trait WithAnchor: Sized {
    /// Set the anchor point for positioning this object.
    fn with_anchor(self, anchor: Anchor) -> Self;
}

/// A bitmap font definition.
///
/// Contains all data needed to render text: glyph lookup table, bitmap data,
/// and metadata about the font.
///
/// # Fields
///
/// - `lookup_table`: Array of glyph entries, one per character in the supported range
/// - `data`: Raw bitmap data for all glyphs, packed as bits
/// - `ascii_begin`: First character code in the supported range (e.g., 32 for space)
/// - `ascii_end`: Last character code in the supported range (e.g., 126 for tilde)
/// - `max_height`: Height of all glyphs in pixels
/// - `proportional_width`: Whether glyphs have variable widths (vs fixed-width)
///
/// # Examples
///
/// ```ignore
/// let font = Font::new(
///     &LOOKUP_TABLE,    // &[GlyphEntry]
///     &FONT_DATA,       // &[u8]
///     32,               // ASCII start
///     126,              // ASCII end
///     8,                // Height
///     true,             // Proportional
/// );
/// ```
pub struct Font {
    /// Lookup table mapping characters to glyph metadata.
    pub lookup_table: &'static [GlyphEntry],
    /// Packed bitmap data for all glyphs.
    pub data: &'static [u8],
    /// First character code supported by this font.
    pub ascii_begin: u32,
    /// Last character code supported by this font.
    pub ascii_end: u32,
    /// Maximum height of glyphs in this font (in pixels).
    pub max_height: u8,
    /// Whether this font uses proportional widths (vs monospace).
    #[allow(dead_code)]
    pub proportional_width: bool,
}

/// Metadata for a single glyph in a font.
///
/// Used as entries in the font's lookup table to locate and describe each character's bitmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlyphEntry {
    /// Width of this glyph in pixels.
    pub width: u8,
    /// Byte offset in font data where this glyph's bitmap starts.
    pub offset: u16,
}

/// A drawable text string with configurable positioning and styling.
///
/// Implements the `Drawable` trait from `embedded-graphics`, allowing it to be
/// rendered on any `DrawTarget`.
///
/// # Fields (Internal)
///
/// - `text`: String to render
/// - `position`: Position for the anchor point
/// - `font`: Font to use for rendering
/// - `color`: Pixel color for the text
/// - `background_color`: Optional background color
/// - `anchor`: Which point of the text bounds the position refers to
/// - `tracking`: Extra pixel spacing between characters
///
/// # Examples
///
/// ```ignore
/// use embedded_graphics::geometry::Point;
/// use embedded_graphics_profont::{Text, Anchor, WithAnchor};
///
/// // Simple left-aligned text
/// let text = Text::new("Hello", Point::new(0, 0), &font, Rgb565::WHITE);
/// target.draw(&text)?;
///
/// // Centered text with letter spacing
/// let text = Text::new("Spaced", Point::new(64, 32), &font, color)
///     .with_anchor(Anchor::MiddleCenter)
///     .with_tracking(2);
/// target.draw(&text)?;
/// ```
pub struct Text<'a, C> {
    text: &'a str,
    position: Point,
    font: &'a Font,
    color: C,
    background_color: Option<C>,
    anchor: Anchor,
    tracking: i32,
}

/// A single drawable character with configurable positioning.
///
/// Like `Text`, this implements `Drawable` for rendering on `DrawTarget`.
///
/// Useful for rendering individual characters with positioning and anchor support.
///
/// # Examples
///
/// ```ignore
/// use embedded_graphics_profont::{Character, Anchor, WithAnchor};
/// use embedded_graphics::geometry::Point;
///
/// let ch = Character::new('A', Point::new(10, 10), &font, color)
///     .with_anchor(Anchor::MiddleCenter);
/// target.draw(&ch)?;
/// ```
pub struct Character<'a, C> {
    ch: char,
    position: Point,
    font: &'a Font,
    color: C,
    background_color: Option<C>,
    anchor: Anchor,
}

impl Font {
    /// Create a new font definition.
    ///
    /// # Arguments
    ///
    /// * `lookup_table` - Array of glyph entries, one per supported character
    /// * `data` - Raw bitmap data for all glyphs
    /// * `ascii_begin` - First supported character code
    /// * `ascii_end` - Last supported character code (inclusive)
    /// * `max_height` - Height of all glyphs in pixels
    /// * `proportional_width` - Whether glyphs have variable widths
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let font = Font::new(
    ///     &GLYPHS,      // 95 entries for ASCII 32-126
    ///     &FONT_DATA,   // Binary bitmap data
    ///     32,           // Space
    ///     126,          // Tilde
    ///     8,            // 8-pixel height
    ///     false,        // Monospace
    /// );
    /// ```
    pub const fn new(
        lookup_table: &'static [GlyphEntry],
        data: &'static [u8],
        ascii_begin: u32,
        ascii_end: u32,
        max_height: u8,
        proportional_width: bool,
    ) -> Self {
        Self {
            lookup_table,
            data,
            ascii_begin,
            ascii_end,
            max_height,
            proportional_width,
        }
    }

    /// Get the glyph metadata for a character.
    ///
    /// Returns `None` if the character is not supported by this font.
    #[inline]
    pub fn get_glyph(&self, c: char) -> Option<&GlyphEntry> {
        let code = c as u32;
        if code >= self.ascii_begin && code <= self.ascii_end {
            let index = (code - self.ascii_begin) as usize;
            self.lookup_table.get(index)
        } else {
            None
        }
    }

    /// Get the bitmap data for a glyph.
    ///
    /// Data is stored as packed bits: 8 bits per byte, where each set bit
    /// represents a pixel that should be drawn.
    pub fn glyph_data(&self, entry: &GlyphEntry) -> &[u8] {
        let start = entry.offset as usize;
        let bytes_per_row = (entry.width as usize + 7) / 8;
        let end = start + (bytes_per_row * self.max_height as usize);
        &self.data[start..end]
    }

    /// Measure the pixel width of a text string.
    ///
    /// Returns the total width including any unsupported characters (which contribute 0 width)
    /// and the specified tracking value.
    ///
    /// # Arguments
    ///
    /// * `text` - The string to measure
    /// * `tracking` - Extra spacing to add between characters in pixels
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let width = font.measure_str("Hello", 0);      // No extra spacing
    /// let width = font.measure_str("Hello", 2);      // 2 pixels between chars
    /// ```
    pub fn measure_str(&self, text: &str, tracking: i32) -> u32 {
        let mut total_width: i32 = 0;
        let mut count = 0;

        for c in text.chars() {
            if let Some(entry) = self.get_glyph(c) {
                total_width += entry.width as i32 + tracking;
                count += 1;
            }
        }

        if count > 0 && tracking > 0 {
            total_width -= tracking;
        }

        total_width.max(0) as u32
    }
}

impl<'a, C> Text<'a, C>
where
    C: PixelColor,
{
    /// Create a new text drawable.
    ///
    /// The text will be positioned at the given point using the default
    /// `TopLeft` anchor. Use `with_anchor()` to change the anchor point.
    ///
    /// # Arguments
    ///
    /// * `text` - String to render
    /// * `position` - Position of the anchor point
    /// * `font` - Font to use
    /// * `color` - Pixel color for the text
    pub fn new(text: &'a str, position: Point, font: &'a Font, color: C) -> Self {
        Self {
            text,
            position,
            font,
            color,
            background_color: None,
            anchor: Anchor::TopLeft,
            tracking: 0,
        }
    }

    /// Set an optional background color for the text.
    ///
    /// # Arguments
    /// 
    /// * `color` - Pixel color for the background
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let text = Text::new("Hello", Point::new(0, 0), &font, color)
    ///     .with_background_color(Rgb565::BLACK);
    /// ```
    pub fn with_background_color(mut self, color: C) -> Self { 
        self.background_color = Some(color);
        self
    }

    /// Set the spacing between characters in pixels.
    ///
    /// # Arguments
    ///
    /// * `tracking` - Extra pixels to add between characters (can be negative)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let text = Text::new("Hello", Point::new(0, 0), &font, color)
    ///     .with_tracking(2);  // 2 pixels between each character
    /// ```
    pub fn with_tracking(mut self, tracking: i32) -> Self {
        self.tracking = tracking;
        self
    }
}

impl<'a, C: PixelColor> WithAnchor for Text<'a, C> {
    fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }
}

impl<'a, C> Character<'a, C>
where
    C: PixelColor,
{
    /// Create a new character drawable.
    ///
    /// The character will be positioned at the given point using the default
    /// `TopLeft` anchor. Use `with_anchor()` to change the anchor point.
    ///
    /// If the character is not supported by the font, it will simply not render.
    ///
    /// # Arguments
    ///
    /// * `ch` - Character to render
    /// * `position` - Position of the anchor point
    /// * `font` - Font to use
    /// * `color` - Pixel color for the character
    pub fn new(ch: char, position: Point, font: &'a Font, color: C) -> Self {
        Self {
            ch,
            position,
            font,
            color,
            background_color: None,
            anchor: Anchor::TopLeft,
        }
    }

    /// Set an optional background color for the character.
    ///
    /// # Arguments
    /// 
    /// * `color` - Pixel color for the background
    pub fn with_background_color(mut self, color: C) -> Self {
        self.background_color = Some(color);
        self
    }
}

impl<'a, C: PixelColor> WithAnchor for Character<'a, C> {
    fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }
}

impl<'a, C> Drawable for Text<'a, C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let text_width = self.font.measure_str(self.text, self.tracking) as i32;
        let text_height = self.font.max_height as i32;

        let (x, y) = match self.anchor {
            Anchor::TopLeft => (
                self.position.x, 
                self.position.y
            ),
            Anchor::TopCenter => (
                self.position.x - (text_width / 2), 
                self.position.y
            ),
            Anchor::TopRight => (
                self.position.x - text_width, 
                self.position.y
            ),
            Anchor::MiddleLeft => (
                self.position.x, 
                self.position.y - (text_height / 2)
            ),
            Anchor::MiddleCenter => (
                self.position.x - (text_width / 2),
                self.position.y - (text_height / 2),
            ),
            Anchor::MiddleRight => (
                self.position.x - text_width,
                self.position.y - (text_height / 2),
            ),
            Anchor::BottomLeft => (
                self.position.x, 
                self.position.y - text_height
            ),
            Anchor::BottomCenter => (
                self.position.x - (text_width / 2), 
                self.position.y - text_height
            ),
            Anchor::BottomRight => (
                self.position.x - text_width, 
                self.position.y - text_height
            ),
        };

        renderer::draw_str(
            target,
            self.text,
            Point::new(x, y),
            self.font,
            self.color,
            self.background_color,
            self.tracking,
        )?;

        Ok(())
    }
}

impl<'a, C> Drawable for Character<'a, C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let char_width = match self.font.get_glyph(self.ch) {
            Some(entry) => entry.width as i32,
            None => return Ok(()),
        };
        let char_height = self.font.max_height as i32;

        let (x, y) = match self.anchor {
            Anchor::TopLeft => (
                self.position.x, 
                self.position.y
            ),
            Anchor::TopCenter => (
                self.position.x - (char_width / 2), 
                self.position.y
            ),
            Anchor::TopRight => (
                self.position.x - char_width, 
                self.position.y
            ),
            Anchor::MiddleLeft => (
                self.position.x, 
                self.position.y - (char_height / 2)
            ),
            Anchor::MiddleCenter => (
                self.position.x - (char_width / 2),
                self.position.y - (char_height / 2),
            ),
            Anchor::MiddleRight => (
                self.position.x - char_width,
                self.position.y - (char_height / 2),
            ),
            Anchor::BottomLeft => (
                self.position.x, 
                self.position.y - char_height
            ),
            Anchor::BottomCenter => (
                self.position.x - (char_width / 2),
                self.position.y - char_height,
            ),
            Anchor::BottomRight => (
                self.position.x - char_width, 
                self.position.y - char_height
            ),
        };

        renderer::draw_char(
            target, 
            self.ch, 
            Point::new(x, y), 
            self.font, 
            self.color, 
            self.background_color,
        )?;

        Ok(())
    }
}