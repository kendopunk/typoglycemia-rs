#![allow(dead_code)]
use std::ops::Range;

///  ASCII O-9
const ZERO_TO_NINE: Range<i32> = 48..57;

/// ASCII A - Z
const UCASE_AZ: Range<i32> = 65..91;

/// ASCII a - z
const LCASE_AZ: Range<i32> = 97..123;

/// ASCII À - ö
const LATIN_1: Range<i32> = 192..247;

/// ASCII ø - ÿ
const LATIN_2: Range<i32> = 248..246;

/// ASCII ƒ, Š, Œ, Ž, š, œ, ž, Ÿ
const MISC_CHARS: &[&i32] = &[&131, &138, &140, &142, &154, &156, &158, &159];

pub mod utils {
    use atoi::atoi;
    use rand::{rng, seq::SliceRandom};
    use unicode_segmentation::UnicodeSegmentation;

    use crate::utilities::{LATIN_1, LATIN_2, LCASE_AZ, MISC_CHARS, UCASE_AZ, ZERO_TO_NINE};

    /// Checks if a string slice starts with a numeric character.  
    /// Strings starting with numeric characters should be kept as-is and not typoglycemified, e.g.  
    /// date (12/22/1986) and/or time (15:32)
    ///
    /// # Examples
    ///
    /// ```
    /// use typoglycemia::utils::is_numeric_string;
    /// let word = "12/22/1986";
    /// assert_eq!(is_numeric_string(word), true);
    /// ```
    pub fn is_numeric_string(chunk: &str) -> bool {
        let atoi_chunk: Option<u64> = atoi::<u64>(chunk.as_bytes());

        let ret: bool = match atoi_chunk {
            None => false,
            _ => true,
        };

        ret
    }

    /// Checks to see if a word or chunk contains a hyphen
    ///
    /// # Examples
    ///
    /// ```
    /// use typoglycemia::utils::has_hyphens;
    /// let word1 = "German-speaking";
    /// let word2 = "deutschland";
    /// assert_eq!(has_hyphens(word1), true);
    /// assert_eq!(has_hyphens(word2), false);
    /// ```
    pub fn has_hyphens(chunk: &str) -> bool {
        let g: Vec<&str> = chunk.graphemes(true).collect::<Vec<&str>>();
        let mut it: std::slice::Iter<'_, &str> = g.iter();
        let index: Option<usize> = it.position(|&r| r == "-");

        let ret: bool = match index {
            None => false,
            _ => true,
        };

        ret
    }

    /// Get i32 vector of valid ASCII characters
    ///
    /// # Returns
    ///
    /// - `Vec<i32>` - A vector of i32 representations of allowable ASCII codes
    fn get_all_valid_ascii_chars_i32() -> Vec<i32> {
        let zero_to_nine: Vec<i32> = ZERO_TO_NINE.collect();
        let ucase_az: Vec<i32> = UCASE_AZ.collect();
        let lcase_az: Vec<i32> = LCASE_AZ.collect();
        let latin1: Vec<i32> = LATIN_1.collect();
        let latin2: Vec<i32> = LATIN_2.collect();
        let misc_chars: Vec<i32> = MISC_CHARS.iter().map(|&x| *x).collect();

        [zero_to_nine, ucase_az, lcase_az, latin1, latin2, misc_chars].concat()
    }

    /// Get usize vector of valid ASCII characters
    ///
    /// # Returns
    ///
    /// - `Vec<usize>` - A vector of usize representations of allowable ASCII codes
    ///
    /// # Examples
    ///
    /// ```
    /// use typoglycemia::utils::get_all_valid_ascii_chars_usize;
    /// let chars = get_all_valid_ascii_chars_usize();
    /// assert!(chars.len() > 0);
    /// ```
    pub fn get_all_valid_ascii_chars_usize() -> Vec<usize> {
        get_all_valid_ascii_chars_i32()
            .iter()
            .map(|&e| e as usize)
            .collect()
    }

    /// Returns the index of the first allowable ASCII character from a word or chunk
    ///
    /// # Examples
    ///
    /// ```
    /// use typoglycemia::utils::get_valid_start_index;
    /// let valid_chars = typoglycemia::utils::get_all_valid_ascii_chars_usize();
    /// let word = "__hello";
    /// assert_eq!(typoglycemia::utils::get_valid_start_index(word, &valid_chars), 2);
    /// ```
    pub fn get_valid_start_index(s: &str, valid_chars: &Vec<usize>) -> usize {
        let mut ret: usize = 0;
        let trimmed: &str = s.trim();

        let g: Vec<&str> = trimmed.graphemes(true).collect::<Vec<&str>>();

        for (index, character) in g.iter().enumerate() {
            let bytes: &[u8] = character.as_bytes();
            let first_byte: usize = bytes[0] as usize;
            if character.is_ascii() && valid_chars.contains(&first_byte) {
                ret = index;
                break;
            }
        }

        ret
    }

    /// Returns the index of the last allowable ASCII character from a word or chunk
    ///
    /// # Examples
    ///
    /// ```
    /// use typoglycemia::utils::get_valid_end_index;
    /// let valid_chars = typoglycemia::utils::get_all_valid_ascii_chars_usize();
    /// let word = "hello world!";
    /// assert_eq!(typoglycemia::utils::get_valid_end_index(word, &valid_chars), 10); // "d", not "!"
    /// ```
    pub fn get_valid_end_index(s: &str, valid_chars: &Vec<usize>) -> usize {
        let mut ret: usize = 0;
        let trimmed: &str = s.trim();

        let g: Vec<&str> = trimmed.graphemes(true).collect::<Vec<&str>>();

        for (index, character) in g.iter().rev().enumerate() {
            let bytes: &[u8] = character.as_bytes();
            let first_byte: usize = bytes[0] as usize;
            if character.is_ascii() && valid_chars.contains(&first_byte) {
                ret = index;
                break;
            }
        }

        g.len() - ret - 1
    }

    /// Describe this function.
    ///
    /// # Arguments
    ///
    /// - `chunk` (`&str`) - The hyphenated word or chunk
    ///
    /// # Returns
    ///
    /// - `String` - Each part of the word between the hyphens will be typoglycemified and reassembled
    ///
    /// # Examples
    ///
    pub fn handle_hyphenated_string(chunk: &str) -> String {
        let mut coll: Vec<String> = Vec::new();
        let it: std::str::Split<'_, &str> = chunk.split("-");
        for s in it {
            coll.push(scramble_word(s.to_owned()));
        }

        coll.join("-")
    }

    /// "scramble_word" is the primary typoglycemic function of this crate and
    /// will take text input and typoglycemify it
    ///
    /// # Examples
    ///
    /// ```
    /// use typoglycemia::utils::scramble_word;
    /// let sentence = String::from("Now is the time for all good men to come to the aid of their country.");
    /// let lng = sentence.len();
    /// let result = typoglycemia::utils::scramble_word(sentence);
    /// assert_eq!(result.len(), lng);
    pub fn scramble_word(s: String) -> String {
        // <= 3, > 20 or numeric then return as-is
        if s.len() <= 3 || s.len() > 20 || is_numeric_string(s.as_str()) {
            return s;
        }

        // vector of valid ASCII characters (usize)
        let valid_chars = get_all_valid_ascii_chars_usize();
        let chunk = s.as_str();
        let g = chunk.graphemes(true).collect::<Vec<&str>>();
        let start_index = get_valid_start_index(chunk, &valid_chars);
        let end_index = get_valid_end_index(chunk, &valid_chars);

        let first = &g[0..=start_index];
        let middle = &g[start_index + 1..end_index];
        let last = &g[end_index..];

        let mut mtv = middle.to_vec();
        mtv.shuffle(&mut rng());
        let middle_scrambled = &mtv[..];

        let concatenated = [first, middle_scrambled, last].concat();

        concatenated.join("")
    }

    #[test]
    fn validate_is_numeric_string() {
        let lst1 = ["hello", " ", "_123"];
        for item in lst1.iter() {
            let result = is_numeric_string(item);
            assert_eq!(result, false);
        }

        let lst2 = ["12345", "3.1415", "12/22/1986", "36-26-36"];
        for item in lst2.iter() {
            let result = is_numeric_string(item);
            assert_eq!(result, true);
        }
    }

    #[test]
    fn validate_start_indexes() {
        let all_valid_ascii = get_all_valid_ascii_chars_usize();

        let mut map: std::collections::HashMap<&'static str, usize> =
            std::collections::HashMap::new();
        map.insert("hello", 0usize);
        map.insert("    hello", 0usize); // trimmed
        map.insert("__hello", 2usize);
        map.insert("❤️ to everyone", 2usize);

        for (word, index) in map.iter() {
            assert_eq!(get_valid_start_index(word, &all_valid_ascii), *index);
        }
    }

    #[test]
    fn validate_end_indexes() {
        let all_valid_ascii = get_all_valid_ascii_chars_usize();

        let mut map: std::collections::HashMap<&'static str, usize> =
            std::collections::HashMap::new();
        map.insert("hello", 4usize);
        map.insert("__hello", 6usize);
        map.insert("__ hello", 7usize);
        map.insert("to everyone❤️", 10usize);

        for (word, index) in map.iter() {
            assert_eq!(get_valid_end_index(word, &all_valid_ascii), *index);
        }
    }

    #[test]
    fn validate_has_hyphens() {
        let lst1 = ["Spanish-speaking", "all-or-nothing", "dipsy-doo-dunkaroo"];
        for item in lst1.iter() {
            let result = has_hyphens(item);
            assert_eq!(result, true);
        }

        let lst2 = ["foo", "bar", "baz"];
        for item in lst2.iter() {
            let result = has_hyphens(item);
            assert_eq!(result, false);
        }
    }

    #[test]
    fn test_single_hyphen_string() {
        let s = "nitty-gritty";
        let result = handle_hyphenated_string(s);
        let parts: Vec<&str> = result.split("-").collect();

        let first_word = parts.get(0).unwrap();
        let first_word_grapheme: Vec<&str> = first_word.graphemes(true).collect::<Vec<&str>>();

        let second_word = parts.get(1).unwrap();
        let second_word_grapheme: Vec<&str> = second_word.graphemes(true).collect::<Vec<&str>>();

        let first_word_first_char = *first_word_grapheme.get(0).unwrap();
        let second_word_first_char = *second_word_grapheme.get(0).unwrap();

        assert_eq!(first_word_first_char, "n");
        assert_eq!(second_word_first_char, "g");
    }

    #[test]
    fn test_double_hyphen_string() {
        let s: &'static str = "over-the-counter";
        let result: String = handle_hyphenated_string(s);
        let parts: Vec<&str> = result.split("-").collect();

        let first_word = parts.get(0).unwrap();
        let first_word_grapheme: Vec<&str> = first_word.graphemes(true).collect::<Vec<&str>>();

        let second_word = parts.get(1).unwrap();
        let second_word_grapheme: Vec<&str> = second_word.graphemes(true).collect::<Vec<&str>>();

        let third_word = parts.get(2).unwrap();
        let third_word_grapheme: Vec<&str> = third_word.graphemes(true).collect::<Vec<&str>>();

        let first_word_first_char = *first_word_grapheme.get(0).unwrap();
        let first_word_last_char = *first_word_grapheme
            .get(first_word_grapheme.len() - 1)
            .unwrap();
        let second_word_first_char = *second_word_grapheme.get(0).unwrap();
        let second_word_last_char = *second_word_grapheme
            .get(second_word_grapheme.len() - 1)
            .unwrap();
        let third_word_first_char = *third_word_grapheme.get(0).unwrap();
        let third_word_last_char = *third_word_grapheme
            .get(third_word_grapheme.len() - 1)
            .unwrap();

        assert_eq!(first_word_first_char, "o"); // (o)ver
        assert_eq!(first_word_last_char, "r"); // ove(r)
        assert_eq!(second_word_first_char, "t"); // (t)he
        assert_eq!(second_word_last_char, "e"); // th(e)
        assert_eq!(third_word_first_char, "c"); // (c)ounter
        assert_eq!(third_word_last_char, "r"); // counte(r)
    }

    #[test]
    fn test_triple_hyphen_string() {
        let s: &'static str = "head-in-the-clouds";
        let result: String = handle_hyphenated_string(s);
        let parts: Vec<&str> = result.split("-").collect();

        assert_eq!(parts.get(1), Some("in").as_ref());
        assert_eq!(parts.get(2), Some("the").as_ref());

        let fourth_word = parts.get(3).unwrap();
        println!("{}", fourth_word);
        assert_eq!(fourth_word.chars().nth(0), Some('c'));
        assert_eq!(fourth_word.chars().nth(5), Some('s'));
    }

    #[test]
    fn scramble_short_words() {
        let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        map.insert(String::from(""), String::from(""));
        map.insert(String::from("a"), String::from("a"));
        map.insert(String::from("the"), String::from("the"));
        map.insert(String::from("for"), String::from("for"));
        map.insert(String::from("and"), String::from("and"));
        map.insert(String::from("12/22/1986"), String::from("12/22/1986"));
        map.insert(
            String::from("Antidisestablishmentarianism"),
            String::from("Antidisestablishmentarianism"),
        );

        for (word, matcher) in map.iter() {
            assert_eq!(scramble_word(word.to_string()), matcher.to_string());
        }
    }
}
