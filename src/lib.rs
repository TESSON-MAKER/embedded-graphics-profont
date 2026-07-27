#![no_std]
#![doc = include_str!("../README.md")]

//! # embedded-graphics-profont
//!
//! A lightweight bitmap font rendering library for `embedded-graphics`.
//!
//! ## Quick Start
//!
//! ```ignore
//! use embedded_graphics::geometry::Point;
//! use embedded_graphics_profont::{Font, Text, Anchor};
//!
//! // Create a font from pre-compiled bitmap data
//! let font = Font::new(&LOOKUP_TABLE, &FONT_DATA, 32, 126, 8, true);
//!
//! // Render text with center anchor
//! let text = Text::new("Hello", Point::new(64, 32), &font, color)
//!     .with_anchor(Anchor::MiddleCenter);
//! target.draw(&text)?;
//! ```

pub mod font;
pub mod renderer;

pub use font::{Anchor, Character, Font, GlyphEntry, Text, WithAnchor};