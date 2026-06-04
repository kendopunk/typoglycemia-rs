pub mod utils {
    use atoi::atoi;
    use rand::{rng, seq::SliceRandom};
    use unicode_segmentation::UnicodeSegmentation;

    /// Build a compile-time boolean lookup table keyed by Unicode codepoint.
    /// Index = codepoint, value = `true` if valid word-boundary character.
    /// Covers: 0–9, A–Z, a–z, Latin-1 (À–ö, 192–246), Latin-2 (ø–ÿ, 248–255),
    /// and extended Latin: Œ(338), œ(339), Š(352), š(353), Ÿ(376), Ž(381), ž(382), ƒ(402).
    const fn build_valid_codepoints() -> [bool; 403] {
        let mut table = [false; 403];
        let mut i = 48usize;
        while i < 58 {
            table[i] = true;
            i += 1;
        } // 0-9
        let mut i = 65usize;
        while i < 91 {
            table[i] = true;
            i += 1;
        } // A-Z
        let mut i = 97usize;
        while i < 123 {
            table[i] = true;
            i += 1;
        } // a-z
        let mut i = 192usize;
        while i < 247 {
            table[i] = true;
            i += 1;
        } // Latin-1 (À–ö)
        let mut i = 248usize;
        while i < 256 {
            table[i] = true;
            i += 1;
        } // Latin-2 (ø–ÿ)
        table[338] = true;
        table[339] = true; // Œ, œ
        table[352] = true;
        table[353] = true; // Š, š
        table[376] = true; // Ÿ
        table[381] = true;
        table[382] = true; // Ž, ž
        table[402] = true; // ƒ
        table
    }

    /// Compile-time lookup table for valid word-boundary codepoints.
    /// Index by `c as usize`; any codepoint ≥ 403 is automatically invalid.
    static VALID_CODEPOINTS: [bool; 403] = build_valid_codepoints();

    /// Returns the index of the last allowable ASCII character in a word, per the allowance configuration  
    ///
    /// # Arguments
    ///
    /// - `s` (`&str`) - The input string
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - The last valid end index, or `None` if no valid character exists
    ///
    fn get_valid_end_index(s: &str) -> Option<usize> {
        let trimmed: &str = s.trim();
        let g: Vec<&str> = trimmed.graphemes(true).collect::<Vec<&str>>();

        for (index, character) in g.iter().rev().enumerate() {
            if let Some(c) = character.chars().next() {
                let codepoint: usize = c as usize;
                if codepoint < VALID_CODEPOINTS.len() && VALID_CODEPOINTS[codepoint] {
                    return Some(g.len() - index - 1);
                }
            }
        }

        None
    }

    /// Returns the index of the first allowable ASCII character in a word, per the allowance configuration  
    ///
    /// # Arguments
    ///
    /// - `s` (`&str`) - The input string
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - The first valid start index, or `None` if no valid character exists
    fn get_valid_start_index(s: &str) -> Option<usize> {
        let trimmed: &str = s.trim();
        let g: Vec<&str> = trimmed.graphemes(true).collect::<Vec<&str>>();

        for (index, character) in g.iter().enumerate() {
            if let Some(c) = character.chars().next() {
                let codepoint: usize = c as usize;
                if codepoint < VALID_CODEPOINTS.len() && VALID_CODEPOINTS[codepoint] {
                    return Some(index);
                }
            }
        }

        None
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
        // U+0027 typewriter apostrophe, U+2019 right single quotation mark, or
        // U+2018 left single quotation mark (all treated as apostrophes)
        let index: Option<usize> = it.position(|&r| r == "'" || r == "\u{2019}" || r == "\u{2018}");

        index.is_some()
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

        index.is_some()
    }

    /// Each part of the word between apostrophes will be typoglycemified and rejoined with an apostrophe, e.g.  
    /// "Principal O'Shag'Hennessey" => "Pirncaipl O'Shag'Hesnneesy" // Mr. Garvey
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
        // Normalize U+2019 / U+2018 smart ticks to U+0027 so the split works uniformly
        let normalized = s.replace('\u{2019}', "'").replace('\u{2018}', "'");
        let mut v: Vec<String> = Vec::new();
        let it: std::str::Split<'_, &str> = normalized.split("'");
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
        // Normalize U+2019 / U+2018 smart ticks to U+0027 so the split works uniformly
        let normalized = s.replace('\u{2019}', "'").replace('\u{2018}', "'");
        let mut v1: Vec<String> = Vec::new();
        let mut v2: Vec<String> = Vec::new();

        let it: std::str::Split<'_, &str> = normalized.split("-");
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
    fn starts_with_digit(s: &str) -> bool {
        let atoi_str: Option<u64> = atoi::<u64>(s.as_bytes());

        atoi_str.is_some()
    }

    /// The primary typoglycemic function of this crate.  
    /// Takes text input and typoglycemifies it.
    ///
    /// # Examples
    ///
    /// ```
    /// use typoglycemia::utils::scramble_word;
    /// let sentence = String::from("Now is the time for all good men to come to the aid of their country.");
    /// let lng = sentence.len();
    /// let result = typoglycemia::utils::scramble_word(sentence);
    /// assert_eq!(result.len(), lng);
    /// ```
    pub fn scramble_word(s: String) -> String {
        let input_as_str: &str = s.as_str();

        // get the graphemes
        let g: Vec<&str> = input_as_str.graphemes(true).collect::<Vec<&str>>();

        if has_apostrophes(&s) && has_hyphens(&s) {
            return handle_apostrophe_and_hyphenated_string(&s);
        }

        if has_apostrophes(&s) {
            return handle_apostrophe_string(&s);
        }

        if has_hyphens(&s) {
            return handle_hyphenated_string(&s);
        }

        // (grapheme length <= 3 or > 15) or numeric then return as-is
        if g.len() <= 3 || g.len() > 15 || starts_with_digit(s.as_str()) {
            return s;
        }

        let start_index = match get_valid_start_index(input_as_str) {
            Some(i) => i,
            None => return s,
        };
        let end_index = match get_valid_end_index(input_as_str) {
            Some(i) => i,
            None => return s,
        };

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
        fn test_starts_with_digit() {
            let lst1 = ["hello", " ", "_123"];
            for item in lst1.iter() {
                let result = starts_with_digit(item);
                assert_eq!(result, false);
            }

            let lst2 = ["12345", "3.1415", "12/22/1986", "36-26-36"];
            for item in lst2.iter() {
                let result = starts_with_digit(item);
                assert_eq!(result, true);
            }
        }

        #[test]
        fn test_get_valid_start_index() {
            let mut map: std::collections::HashMap<&'static str, usize> =
                std::collections::HashMap::new();
            map.insert("hello", 0usize);
            map.insert("    hello", 0usize); // trimmed
            map.insert("__hello", 2usize);
            map.insert("❤️ to everyone", 2usize);

            for (word, index) in map.iter() {
                assert_eq!(get_valid_start_index(word), Some(*index));
            }
        }

        #[test]
        fn test_get_valid_end_index() {
            let mut map: std::collections::HashMap<&'static str, usize> =
                std::collections::HashMap::new();
            map.insert("hello", 4usize);
            map.insert("__hello", 6usize);
            map.insert("__ hello", 7usize);
            map.insert("to everyone❤️", 10usize);

            for (word, index) in map.iter() {
                assert_eq!(get_valid_end_index(word), Some(*index));
            }
        }

        #[test]
        fn test_has_apostrophes() {
            let lst1 = ["doesn't", "won't", "couldn't", "O'Shag-hennesey"];
            for item in lst1.iter() {
                let result = has_apostrophes(item);
                assert_eq!(result, true);
            }

            // U+2019 right single quotation mark (smart tick)
            let lst1_smart = ["doesn\u{2019}t", "won\u{2019}t", "couldn\u{2019}t"];
            for item in lst1_smart.iter() {
                let result = has_apostrophes(item);
                assert_eq!(result, true);
            }

            // U+2018 left single quotation mark (opening smart tick; rare mid-word usage)
            let lst1_open = ["O\u{2018}Brien", "wouldn\u{2018}t"];
            for item in lst1_open.iter() {
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
        fn test_double_apostrophe_string() {
            let s: &'static str = "woulda'coulda'shoulda";
            let result: String = handle_apostrophe_string(s);
            let parts: Vec<&str> = result.split("'").collect();

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

            assert_eq!(first_word_first_char, "w"); // (w)oulda
            assert_eq!(first_word_last_char, "a"); // would(a)
            assert_eq!(second_word_first_char, "c"); // (c)oulda
            assert_eq!(second_word_last_char, "a"); // could(a)
            assert_eq!(third_word_first_char, "s"); // (s)houlda
            assert_eq!(third_word_last_char, "a"); // should(a)
        }

        #[test]
        fn test_has_hyphens() {
            let lst1 = ["Spanish-speaking", "all-or-nothing", "dipsy-doo-dunkaroo"];
            for item in lst1.iter() {
                let result = has_hyphens(item);
                assert_eq!(result, true);
            }

            let lst2 = ["Spanish", "all", "dipsy"];
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
            let first_word_last_char: &str = *first_word_grapheme
                .get(first_word_grapheme.len() - 1)
                .unwrap();

            let second_word_first_char: &str = *second_word_grapheme.get(0).unwrap();
            let second_word_last_char: &str = *second_word_grapheme
                .get(second_word_grapheme.len() - 1)
                .unwrap();

            assert_eq!(first_word_first_char, "n");
            assert_eq!(first_word_last_char, "y");
            assert_eq!(second_word_first_char, "g");
            assert_eq!(second_word_last_char, "y");
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
        fn test_dont_scramble_short_words() {
            let mut map: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();
            map.insert(String::from(""), String::from(""));
            map.insert(String::from("a"), String::from("a"));
            map.insert(String::from("the"), String::from("the"));
            map.insert(String::from("for"), String::from("for"));
            map.insert(String::from("and"), String::from("and"));
            map.insert(String::from("12/22/1986"), String::from("12/22/1986"));

            for (word, matcher) in map.iter() {
                assert_eq!(scramble_word(word.to_string()), matcher.to_string());
            }
        }

        #[test]
        fn test_dont_scramble_long_words() {
            let mut map: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();
            map.insert(
                String::from("antidisestablishmentarianism"),
                String::from("antidisestablishmentarianism"),
            );
            map.insert(
                String::from("anthropomorphism"),
                String::from("anthropomorphism"),
            );
            map.insert(
                String::from("unconstitutional"),
                String::from("unconstitutional"),
            );
            map.insert(
                String::from("multidimensional"),
                String::from("multidimensional"),
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
