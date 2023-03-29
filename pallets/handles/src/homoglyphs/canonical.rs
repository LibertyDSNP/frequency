//! # Handle Converter
//!
//! `handle_converter` provides a `HandleConverter` struct to detect confusable Unicode characters in a
//! given input string and return its canonical form.
use crate::homoglyphs::confusables::build_confusables_map;
use sp_std::collections::btree_map::BTreeMap;
use unicode_normalization::UnicodeNormalization;

/// A converter for confusable characters.
///
/// Given a string, detects easily confusable characters and returns the string in canonical form.
pub struct HandleConverter {
	confusables_map: BTreeMap<char, char>,
}
/// Creates a new `HandleConverter` instance with the specified input string.
///
/// # Example
///
/// ```
/// use crate::canonical::HandleConverter;
///
/// let word = "ℂн𝔸RℒℰᏕ";
///
/// let handle_converter = HandleConverter::new();
/// let canonical_word = handle_converter.remove_confusables(word);
/// println!("{}", canonical_word);
///
/// OUTPUT:
/// charles
/// ```
impl HandleConverter {
	pub fn new() -> Self {
		let confusables_map = build_confusables_map();
		Self { confusables_map }
	}
	/// Convert `string`
	pub fn to_canonical(&self, string: &str) -> codec::alloc::string::String {
		let confusables_removed = self.remove_confusables(string);
		let diacriticals_stripped = self.strip_diacriticals(&confusables_removed);
		diacriticals_stripped.to_ascii_lowercase()
	}

	pub fn remove_confusables(&self, string: &str) -> codec::alloc::string::String {
		string
			.chars()
			.map(|character| self.confusables_map.get(&character).map_or(character, |&value| value))
			.collect::<codec::alloc::string::String>()
	}

	pub fn strip_diacriticals(&self, string: &str) -> codec::alloc::string::String {
		string.nfd().collect::<codec::alloc::string::String>()
	}
}