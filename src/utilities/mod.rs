use std::ops::Range;

///  ASCII O-9
const ZERO_TO_NINE: Range<usize> = 48..57;

/// ASCII A - Z
const UCASE_AZ: Range<usize> = 65..91;

/// ASCII a - z
const LCASE_AZ: Range<usize> = 97..123;

/// ASCII À - ö
const LATIN_1: Range<usize> = 192..247;

/// ASCII ø - ÿ
const LATIN_2: Range<usize> = 248..256;

/// ASCII ƒ, Š, Œ, Ž, š, œ, ž, Ÿ
const MISC_CHARS: &[&i32] = &[&131, &138, &140, &142, &154, &156, &158, &159];

pub mod utils {
    use atoi::atoi;
    use rand::{rng, seq::SliceRandom};
    use unicode_segmentation::UnicodeSegmentation;

    use crate::utilities::{LATIN_1, LATIN_2, LCASE_AZ, MISC_CHARS, UCASE_AZ, ZERO_TO_NINE};

    /// Get usize vector of valid ASCII characters
    ///
    /// # Returns
    ///
    /// - `Vec<usize>` - Describe the return value.
    ///
    fn get_all_valid_ascii_chars() -> Vec<usize> {
        let zero_to_nine: Vec<usize> = ZERO_TO_NINE.collect();
        let ucase_az: Vec<usize> = UCASE_AZ.collect();
        let lcase_az: Vec<usize> = LCASE_AZ.collect();
        let latin1: Vec<usize> = LATIN_1.collect();
        let latin2: Vec<usize> = LATIN_2.collect();
        let misc_chars: Vec<usize> = MISC_CHARS.iter().map(|&x| *x as usize).collect();

        [zero_to_nine, ucase_az, lcase_az, latin1, latin2, misc_chars].concat()
    }

    /// Returns the index of the last allowable ASCII character in a word
    ///
    /// # Arguments
    ///
    /// - `s` (`&str`) - The input string
    /// - `valid_chars` (`&Vec<usize>`) - vector comprised of usize valid ASCII characters
    ///
    /// # Returns
    ///
    /// - `usize` - The first valid start index
    ///
    fn get_valid_end_index(s: &str, valid_chars: &Vec<usize>) -> usize {
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

    /// Returns the index of the first allowable ASCII character in a word
    ///
    /// # Arguments
    ///
    /// - `s` (`&str`) - The input string
    /// - `valid_chars` (`&Vec<usize>`) - vector comprisesd of usize valid ASCII characters
    ///
    /// # Returns
    ///
    /// - `usize` - The first valid start index
    fn get_valid_start_index(s: &str, valid_chars: &Vec<usize>) -> usize {
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

    /// Determines if a word contains apostrophes
    ///
    /// # Arguments
    ///
    /// - `s` (`&str`) - The input string
    ///
    /// # Returns
    ///
    /// - `bool` - Whether or not apostrophes exist
    ///
    fn has_apostrophes(s: &str) -> bool {
        let g: Vec<&str> = s.graphemes(true).collect::<Vec<&str>>();
        let mut it: std::slice::Iter<'_, &str> = g.iter();
        let index: Option<usize> = it.position(|&r| r == "'");

        let ret: bool = match index {
            None => false,
            _ => true,
        };

        ret
    }

    /// Determines if a word contains hyphens
    ///
    /// # Arguments
    ///
    /// - `s` (`&str`) - The input string
    ///
    /// # Returns
    ///
    /// - `bool` - Whether or not hyphens exist
    ///
    fn has_hyphens(s: &str) -> bool {
        let g: Vec<&str> = s.graphemes(true).collect::<Vec<&str>>();
        let mut it: std::slice::Iter<'_, &str> = g.iter();
        let index: Option<usize> = it.position(|&r| r == "-");

        let ret: bool = match index {
            None => false,
            _ => true,
        };

        ret
    }

    /// Each part of the word between apostrophes will be typoglycemified and rejoined with an apostrophe, e.g.  
    /// "Spanish-speaking" => "Sanipsh-spkeanig"
    ///
    /// # Arguments
    ///
    /// - `s` (`&str`) - The word containing apostrophes
    ///
    /// # Returns
    ///
    /// - `String` - The modified string with portions scrambled
    ///
    fn handle_apostrophe_string(s: &str) -> String {
        let mut v: Vec<String> = Vec::new();
        let it: std::str::Split<'_, &str> = s.split("'");
        for part in it {
            v.push(scramble_word(part.to_owned()));
        }

        v.join("'")
    }

    /// Each part of the word between the hyphens will be typoglycemified and rejoined with hyphens, e.g.  
    /// "Spanish-speaking" => "Sanipsh-spkeanig"
    ///
    /// # Arguments
    ///
    /// - `s` (`&str`) - The hyphenated word
    ///
    /// # Returns
    ///
    /// - `String` - The re-hyphenated string with portions scrambled
    ///
    fn handle_hyphenated_string(s: &str) -> String {
        let mut coll: Vec<String> = Vec::new();
        let it: std::str::Split<'_, &str> = s.split("-");
        for part in it {
            coll.push(scramble_word(part.to_owned()));
        }

        coll.join("-")
    }

    /// Each part of the word between the apoostrophes and hyphens will be typoglycemified and rejoined e.g.  
    /// "O'Leary-sanctioned" => "O'Lraey-sninactoed"
    ///
    /// # Arguments
    ///
    /// - `s` (`&str`) - The hyphenated word
    ///
    /// # Returns
    ///
    /// - `String` - The re-joined string
    ///
    fn handle_apostrophe_and_hyphenated_string(s: &str) -> String {
        let mut v1: Vec<String> = Vec::new();
        let mut v2: Vec<String> = Vec::new();

        let it: std::str::Split<'_, &str> = s.split("-");
        for i in it {
            let st = i.split("'");
            for s in st {
                v2.push(scramble_word(s.to_owned()));
            }
            v1.push(v2.join("'"));
            v2.clear();
        }

        v1.join("-")
    }

    /// Checks if a string slice starts with a numeric character.  
    /// Strings starting with numeric characters should be kept as-is and not typoglycemified, e.g.  
    /// date (12/22/1986) and/or time (15:32)
    ///
    fn is_numeric_string(s: &str) -> bool {
        let atoi_str: Option<u64> = atoi::<u64>(s.as_bytes());

        let ret: bool = match atoi_str {
            None => false,
            _ => true,
        };

        ret
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
        let input_as_str: &str = s.as_str();

        // vector of valid ASCII characters (usize)
        let valid_chars: Vec<usize> = get_all_valid_ascii_chars();

        // get the graphemes
        let g: Vec<&str> = input_as_str.graphemes(true).collect::<Vec<&str>>();

        // (grapheme length <= 3 or > 20) or numeric then return as-is
        if g.len() <= 3 || g.len() > 20 || is_numeric_string(s.as_str()) {
            return s;
        }

        if has_apostrophes(&s) && has_hyphens(&s) {
            return handle_apostrophe_and_hyphenated_string(&s);
        }

        if has_apostrophes(&s) {
            return handle_apostrophe_string(&s);
        }

        if has_hyphens(&s) {
            return handle_hyphenated_string(&s);
        }

        let start_index = get_valid_start_index(input_as_str, &valid_chars);
        let end_index = get_valid_end_index(input_as_str, &valid_chars);

        // for example, this weird string w/ only one valid ascii character -> __a__
        if start_index == end_index {
            return s;
        }

        let first = &g[0..=start_index];
        let middle = &g[start_index + 1..end_index];
        let last = &g[end_index..];

        let mut mtv = middle.to_vec();
        mtv.shuffle(&mut rng());
        let middle_scrambled = &mtv[..];

        let concatenated = [first, middle_scrambled, last].concat();

        concatenated.join("")
    }

    // testing pub / private functions
    #[cfg(test)]
    mod tests {

        // Import all items from the parent module
        use super::*;

        #[test]
        fn test_is_numeric_string() {
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
        fn test_get_valid_start_index() {
            let all_valid_ascii = get_all_valid_ascii_chars();

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
        fn test_get_valid_end_index() {
            let all_valid_ascii = get_all_valid_ascii_chars();

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
        fn test_has_apostrophes() {
            let lst1 = ["doesn't", "won't", "couldn't", "O'Shag-hennesey"];
            for item in lst1.iter() {
                let result = has_apostrophes(item);
                assert_eq!(result, true);
            }

            let lst2 = ["foo", "bar", "baz"];
            for item in lst2.iter() {
                let result = has_apostrophes(item);
                assert_eq!(result, false);
            }
        }

        #[test]
        fn test_single_apostrophe_string() {
            let s: &'static str = "O'Shaghennessy"; // Mr. Garvey
            let result: String = handle_apostrophe_string(s);
            let parts: Vec<&str> = result.split("'").collect();

            let first_word: &&str = parts.get(0).unwrap();
            let first_word_grapheme: Vec<&str> = first_word.graphemes(true).collect::<Vec<&str>>();

            let second_word: &&str = parts.get(1).unwrap();
            let second_word_grapheme: Vec<&str> =
                second_word.graphemes(true).collect::<Vec<&str>>();

            let first_word_first_char: &str = *first_word_grapheme.get(0).unwrap();
            let second_word_first_char: &str = *second_word_grapheme.get(0).unwrap();
            let second_word_last_char: &str = *second_word_grapheme
                .get(second_word_grapheme.len() - 1)
                .unwrap();

            assert_eq!(first_word_first_char, "O");
            assert_eq!(second_word_first_char, "S");
            assert_eq!(second_word_last_char, "y");
        }

        #[test]
        fn test_has_hyphens() {
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
            let s: &'static str = "nitty-gritty";
            let result: String = handle_hyphenated_string(s);
            let parts: Vec<&str> = result.split("-").collect();

            let first_word: &&str = parts.get(0).unwrap();
            let first_word_grapheme: Vec<&str> = first_word.graphemes(true).collect::<Vec<&str>>();

            let second_word: &&str = parts.get(1).unwrap();
            let second_word_grapheme: Vec<&str> =
                second_word.graphemes(true).collect::<Vec<&str>>();

            let first_word_first_char: &str = *first_word_grapheme.get(0).unwrap();
            let second_word_first_char: &str = *second_word_grapheme.get(0).unwrap();

            assert_eq!(first_word_first_char, "n");
            assert_eq!(second_word_first_char, "g");
        }

        #[test]
        fn test_double_hyphen_string() {
            let s: &'static str = "over-the-counter";
            let result: String = handle_hyphenated_string(s);
            let parts: Vec<&str> = result.split("-").collect();

            let first_word: &&str = parts.get(0).unwrap();
            let first_word_grapheme: Vec<&str> = first_word.graphemes(true).collect::<Vec<&str>>();

            let second_word: &&str = parts.get(1).unwrap();
            let second_word_grapheme: Vec<&str> =
                second_word.graphemes(true).collect::<Vec<&str>>();

            let third_word: &&str = parts.get(2).unwrap();
            let third_word_grapheme: Vec<&str> = third_word.graphemes(true).collect::<Vec<&str>>();

            let first_word_first_char: &str = *first_word_grapheme.get(0).unwrap();
            let first_word_last_char: &str = *first_word_grapheme
                .get(first_word_grapheme.len() - 1)
                .unwrap();
            let second_word_first_char: &str = *second_word_grapheme.get(0).unwrap();
            let second_word_last_char: &str = *second_word_grapheme
                .get(second_word_grapheme.len() - 1)
                .unwrap();
            let third_word_first_char: &str = *third_word_grapheme.get(0).unwrap();
            let third_word_last_char: &str = *third_word_grapheme
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

            let fourth_word: &&str = parts.get(3).unwrap();
            assert_eq!(fourth_word.chars().nth(0), Some('c'));
            assert_eq!(fourth_word.chars().nth(5), Some('s'));
        }

        #[test]
        fn test_scramble_short_words() {
            let mut map: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();
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

        #[test]
        fn test_one_valid_ascii_char() {
            let mut map: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();
            map.insert(String::from("_a"), String::from("_a"));
            map.insert(String::from("a_"), String::from("a_"));
            map.insert(String::from("___a___"), String::from("___a___"));
            map.insert(String::from("a...,,,"), String::from("a...,,,"));
            map.insert(String::from("!@#$%a"), String::from("!@#$%a"));

            for (word, matcher) in map.iter() {
                assert_eq!(scramble_word(word.to_string()), matcher.to_string());
            }
        }
    }
}
