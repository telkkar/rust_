//! fuzzy_search
//!
//! The `fuzzy_search` crate provides functions that perform fuzzy searches.
//!
//! ```
//! assert_eq!(fuzzy_search::fuzzy_search_match("fuzzy_search_match", "fu_rch_match"), true);
//! ```


/// fuzzy_search_match
///
/// This function is a poor(?) implementation of a fuzzy search in rust.
/// It will return true if the search string characters appear in order in the reference string.
/// It will handle UTF-8 correctly.
///
/// # Examples
/// ```
/// use fuzzy_search::fuzzy_search_match;
///
/// assert_eq!(fuzzy_search_match("Alaska Beer Crusade", "abc"), false);
/// ```
///
/// ```
/// use fuzzy_search:: fuzzy_search_match;
///
/// assert_eq!(fuzzy_search_match("abcdefg", "abc"), true);
/// ```
pub fn fuzzy_search_match(reference_string: &str, search_string: &str) -> bool {
	fuzzy_search(reference_string, search_string)
}

fn fuzzy_search(reference_string: &str, search_string: &str) -> bool {
	let mut search_iterator = search_string.chars();
	let mut search = search_iterator.next();

	for ref_char in reference_string.chars() {
		match search {
			Some(search_char) => {
				if search_char == ref_char {
					search = search_iterator.next();
					continue;
				}
			}
			None => return true
		}
	}

	search == None
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn fuzzy_search_match_exact_string_true() {
		// Arrange
		let search_string: String = String::from("string");
		let reference_string: String = String::from("string");
		let expected_value: bool = true;

		// Act
		let actual_value: bool = fuzzy_search_match(&reference_string, &search_string);

		// Assert
		assert_eq!(expected_value, actual_value);
	}

	#[test]
	fn fuzzy_search_match_search_longer_than_reference_false() {
		// Arrange
		let search_string: String = String::from("longer");
		let reference_string: String = String::from("a");
		let expected_value: bool = false;

		// Act
		let actual_value: bool = fuzzy_search_match(&reference_string, &search_string);

		// Assert
		assert_eq!(expected_value, actual_value);
	}

	#[test]
	fn fuzzy_search_match_search_first_three_letters_true() {
		// Arrange
		let search_string: String = String::from("str");
		let reference_string: String = String::from("string");
		let expected_value: bool = true;

		// Act
		let actual_value: bool = fuzzy_search_match(&reference_string, &search_string);

		// Assert
		assert_eq!(expected_value, actual_value);
	}

	#[test]
	fn fuzzy_search_match_search_multibyte_characters_true() {
		// Arrange
		let search_string: String = String::from("老虎é");
		let reference_string: String = String::from("Löwe 老虎 Léopard");
		let expected_value: bool = true;

		// Act
		let actual_value: bool = fuzzy_search_match(&reference_string, &search_string);

		// Assert
		assert_eq!(expected_value, actual_value);
	}

	#[test]
	fn fuzzy_search_match_search_exact_multibyte_string_true() {
		// Arrange
		let search_string: String = String::from("Löwe 老虎 Léopard");
		let reference_string: String = String::from("Löwe 老虎 Léopard");
		let expected_value: bool = true;

		// Act
		let actual_value: bool = fuzzy_search_match(&reference_string, &search_string);

		// Assert
		assert_eq!(expected_value, actual_value);
	}

	#[test]
	fn fuzzy_search_match_search_string_spaces_true() {
		// Arrange
		let search_string: String = String::from("Löwe 老虎");
		let reference_string: String = String::from("Löwe 老虎 Léopard");
		let expected_value: bool = true;

		// Act
		let actual_value: bool = fuzzy_search_match(&reference_string, &search_string);

		// Assert
		assert_eq!(expected_value, actual_value);
	}

	#[test]
	fn fuzzy_search_match_code_example_true() {
		// Arrange
		let search_string: String = String::from("Super.cs");
		let reference_string: String = String::from("SuperAwesomeClass.cs");
		let expected_value: bool = true;

		// Act
		let actual_value: bool = fuzzy_search_match(&reference_string, &search_string);

		// Assert
		assert_eq!(expected_value, actual_value);
	}

	#[test]
	fn fuzzy_search_match_ref_and_search_same_letters_reversed_false() {
		// Arrange
		let search_string: String = String::from("tac");
		let reference_string: String = String::from("cat");
		let expected_value: bool = false;

		// Act
		let actual_value: bool = fuzzy_search_match(&reference_string, &search_string);

		// Assert
		assert_eq!(expected_value, actual_value);
	}

	#[test]
	fn fuzzy_search_match_split_characters_false() {
		// This one needs a bit of explaining, and is derived from the second example under
		// https://doc.rust-lang.org/std/primitive.str.html#examples-9
		// There's a chance that two or more characters combine to make a grapheme cluster, but are
		// Found in different locations in the string. Take the string, as chars,
		//
		//     yay\u{0306}  (yay̆, the y has a breve)
		//
		// And the string below
		//
		//     yao\u{0306}  (yaŏ, the o has a breve)
		//
		// Searching for the string "y̆" can match the second string if the grapheme is split into
		// two characters -- the 'y' and the breve.

		// Arrange
		let search_string: String = String::from("y̆");
		let reference_string: String = String::from("yaŏ");
		let expected_value: bool = false;

		// Act
		let actual_value: bool = fuzzy_search_match(&reference_string, &search_string);

		// Assert
		assert_eq!(expected_value, actual_value);
	}
}
