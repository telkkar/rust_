//! fuzzy_search
//!
//! The `fuzzy_search` crate provides functions that perform fuzzy searches.
//!
//! ```
//! assert_eq!(fuzzy_search::fuzzy_search_match("fu_rch_match", "fuzzy_search_match"), true);
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
/// assert_eq!(fuzzy_search_match("abc", "Alaska Beer Crusade"), false);
/// ```
///
/// ```
/// use fuzzy_search:: fuzzy_search_match;
///
/// assert_eq!(fuzzy_search_match("abc", "abcdefg"), true);
/// ```
pub fn fuzzy_search_match(search_string: &str, reference_string: &str) -> bool {
	recursive_fuzzy_search(search_string, reference_string)
}

fn recursive_fuzzy_search(search_string: &str, reference_string: &str) -> bool {
	let search_len: usize = search_string.len();
	let reference_len: usize = reference_string.len();

	if search_string == "" {
		return true;
	}

	if reference_string == "" {
		return false;
	}

	// Search strings longer than the reference strings are not valid
	if reference_len < search_len {
		return false;
	}

	for (search_index, search_value) in search_string.char_indices() {
		// Case where all chars in search string are found
		if search_index == search_len {
			return true;
		}

		for (reference_index, reference_value) in reference_string.char_indices() {
			if search_value == reference_value {
				let (_, rhs_reference_substring) = reference_string.split_at(find_next_char_boundary(reference_string, reference_index));
				let (_, rhs_search_substring) = search_string.split_at(find_next_char_boundary(search_string, search_index));
				return recursive_fuzzy_search(rhs_search_substring, rhs_reference_substring);
			}
		}

		return false;
	}

	false
}

fn find_next_char_boundary(string: &str, index: usize) -> usize {
	for offset in 1..string.len() + 1 {
		if string.is_char_boundary(index + offset) {
			return index + offset;
		}
	}

	index
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
		let actual_value: bool = fuzzy_search_match(&search_string, &reference_string);

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
		let actual_value: bool = fuzzy_search_match(&search_string, &reference_string);

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
		let actual_value: bool = fuzzy_search_match(&search_string, &reference_string);

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
		let actual_value: bool = fuzzy_search_match(&search_string, &reference_string);

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
		let actual_value: bool = fuzzy_search_match(&search_string, &reference_string);

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
		let actual_value: bool = fuzzy_search_match(&search_string, &reference_string);

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
		let actual_value: bool = fuzzy_search_match(&search_string, &reference_string);

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
		let actual_value: bool = fuzzy_search_match(&search_string, &reference_string);

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
		let actual_value: bool = fuzzy_search_match(&search_string, &reference_string);

		// Assert
		assert_eq!(expected_value, actual_value);
	}
}
