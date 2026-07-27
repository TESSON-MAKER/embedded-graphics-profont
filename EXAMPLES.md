# Examples

## Basic Text

```rust
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
};
use embedded_graphics_profont::Text;

mod fonts;
use fonts::FONT;

let text = Text::new("Hello", Point::new(10, 10), &FONT, Rgb666::BLACK);
display.draw(&text).ok();
```

## Centered Text

```rust
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
};
use embedded_graphics_profont::{Text, Anchor, WithAnchor};

mod fonts;
use fonts::FONT;

let text = Text::new("Center", Point::new(160, 120), &FONT, Rgb666::BLACK)
    .with_anchor(Anchor::MiddleCenter);
display.draw(&text).ok();
```

## Letter Spacing

```rust
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
};
use embedded_graphics_profont::Text;

mod fonts;
use fonts::FONT;

let text = Text::new("Spaced", Point::new(0, 0), &FONT, Rgb666::BLACK)
    .with_tracking(2);  // 2 pixels between characters
display.draw(&text).ok();
```

## Right-Aligned Text

```rust
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
};
use embedded_graphics_profont::{Text, Anchor, WithAnchor};

mod fonts;
use fonts::FONT;

let text = Text::new("Right", Point::new(319, 10), &FONT, Rgb666::BLACK)
    .with_anchor(Anchor::TopRight);
display.draw(&text).ok();
```

## Multi-line Text

```rust
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
};
use embedded_graphics_profont::Text;

mod fonts;
use fonts::FONT;

let line_height = FONT.max_height as i32 + 2;

for (i, line) in ["Line 1", "Line 2", "Line 3"].iter().enumerate() {
    let text = Text::new(line, Point::new(10, 10 + i as i32 * line_height), &FONT, Rgb666::BLACK);
    display.draw(&text).ok();
}
```

## Single Character

```rust
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
};
use embedded_graphics_profont::{Character, Anchor, WithAnchor};

mod fonts;
use fonts::FONT;

let ch = Character::new('A', Point::new(100, 50), &FONT, Rgb666::BLACK)
    .with_anchor(Anchor::MiddleCenter);
display.draw(&ch).ok();
```

## Text Measurement

```rust
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
};
use embedded_graphics_profont::Text;

mod fonts;
use fonts::FONT;

let width = FONT.measure_str("Hello", 0);
// Center it:
let display_width = 480;  // ILI9488 width
let x = (display_width / 2) as i32 - (width as i32 / 2);
let text = Text::new("Hello", Point::new(x, 10), &FONT, Rgb666::BLACK);
display.draw(&text).ok();
```

## Dynamic Text

```rust
use core::fmt::Write;
use heapless::String;
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
};
use embedded_graphics_profont::Text;

mod fonts;
use fonts::FONT;

let mut buffer: String<32> = String::new();
write!(&mut buffer, "Value: {}", 42).unwrap();

let text = Text::new(&buffer, Point::new(10, 10), &FONT, Rgb666::BLACK);
display.draw(&text).ok();
```

## Layout Template

```rust
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
};
use embedded_graphics_profont::{Text, Anchor, WithAnchor};

mod fonts;
use fonts::FONT;

// Title
Text::new("Title", Point::new(160, 10), &FONT, Rgb666::BLACK)
    .with_anchor(Anchor::TopCenter)
    .draw(&mut display).ok();

// Content (centered)
Text::new("Content", Point::new(160, 120), &FONT, Rgb666::BLACK)
    .with_anchor(Anchor::MiddleCenter)
    .draw(&mut display).ok();

// Status (bottom-right)
Text::new("Ready", Point::new(315, 230), &FONT, Rgb666::BLACK)
    .with_anchor(Anchor::BottomRight)
    .draw(&mut display).ok();
```
