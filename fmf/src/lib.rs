//! # FMF Markdown Language
//!
//! FMF (_field, millet, fox_) is a format for markdown-like pages.
//!
//! ## Usage
//! ```
//! use fmf::{document::Document, format::DefaultFormatEngine};
//!
//! let fragment = r#"
//!  # Hello, world!
//!  Paragraph...
//! "#;
//! let document = Document::<DefaultFormatEngine>::new(fragment);
//!
//! println!("{document}");
//! ```
//!
//! ## Documentation
//!
//! There only 0 contexts, 0 inline formatting and 0 special formats.
//!
//! ### Special formats
//!
//! Special formats is one-line format, like headings or cards.
//!
//! #### Heading
//!
//! Like markdown, headings defined using `#` symbol:
//!
//! ```markdown
//! # Heading 1
//! ## Heading 2
//! ###### Heading 6 (maximum value)
//! ```
//!
//! #### Author card
//!
//! Usually author cards adding after h1.
//!
//! ```markdown
//! /// author$name / date / category
//! ```
//!
//! It will be formatted as Author author$name with avatar /authorname.jpg.
//!
//! ### Contexts
//!
//! Context is multi-line markdown like paragraphs or code blocks.
//!
//! #### Paragraph
//!
//! Paragraph is just text without blank lines. Example:
//!
//! ```markdown
//! Paragraph 1.
//! Still paragraph 1.
//!
//! Paragraph 2.
//! ```
//!
//! #### Code blocks
//!
//! Like markdown, code blocks defined using three `` ` `` symbols:
//!
//! ```markdown
//! (due markdown limits, code blocks here starts with space, but it not required)
//!  ```language: foo, filename: bar, ...
//! int a = 10;
//! print(\*a\*); // To use inline-formatting in code blocks add `\` before format symbol
//!  ```_
//! Please ignore `_` in line upper
//! ```
//!
//! #### Ordered and unordered lists
//!
//! ```markdown
//! Ordered list:
//!
//! @ First
//! @ Second
//! @ Half-Life
//!
//! Unordered list:
//!
//! - Foo
//! - Bar
//! - Pizza
//! ```
//!
//! ### Inline-formatting
//!
//! Inline-formatting is formatting just in line, like *bold*, `code`, etc...
//!
//! 1. *Bold*. Writes using `*`: `normal *bold*`
//! 2. _Italic_. Writes using `_`: `normal _italic_`
//! 3. `Code`. Writes using `` ` ``: ``normal `code` ``
//! 4. Links. Example: `\(google.com)`, `\(google.com)[Google]`, `\[Just blue text]`
//!

/// Document implementation
pub mod document;
/// Format defination
pub mod format;

