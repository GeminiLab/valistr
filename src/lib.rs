//! This crate provides a procedural macro to create immutable string wrapper types with values validated with regexes.
//!
//! ## Usage
//!
//! To use this crate, use `#[valistr(regex)]` attribute on a unit struct. The regex should be a string literal. Anchors `^` and `$` are automatically added to the regex if they are not present.
//!
//! The following traits are implemented for the struct:
//! - `Deref<Target = String>`
//! - `DerefMut<Target = String>`
//! - `Debug`
//! - `Display`
//! - `TryFrom<&str>`
//! - `TryFrom<String>`
//!
//! The following methods are generated for the struct:
//! - `fn new(value: impl Into<String>) -> Option<Self>`
//! - `fn validator() -> &'static Regex`
//! - `fn get_<name>(&self) -> Option<&str>` for each named capture group `<name>` in the regex.
//!   - The method is generated only if the name matches `[a-z][a-z0-9_]*`. This constraint guarantees the generated method name is a valid and clean Rust identifier.
//!
//! ## Example
//!
//! ```rust
//! use valistr::valistr;
//!
//! #[valistr("[A-Za-z]*")]
//! struct AsciiLetters;
//!
//! assert!(AsciiLetters::new("Hello").is_some());
//! assert!(AsciiLetters::new("Hello, world!").is_none());
//! ```
//!
pub use valistr_proc_macro::valistr;
