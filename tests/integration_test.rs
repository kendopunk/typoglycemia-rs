use typoglycemia::{typoglycemia, typoglycemia_leet};
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
#[test]
fn it_handles_a_string_slice() {
    let s: &str = "slice";
    let result = typoglycemia(s);
    assert_eq!(result.chars().nth(0), Some('s'));
    assert_eq!(result.chars().nth(4), Some('e'));
}

#[test]
fn it_does_not_typoglycemify_short_words() {
    let lst = [String::from("a"), String::from("an"), String::from("foo")];
    for word in lst.iter() {
        let result = typoglycemia(word);
        assert_eq!(result, *word);
    }
}

#[test]
fn it_does_not_typoglycemify_long_words() {
    let lst = [
        String::from("glossolabiopharyngeal"),
        String::from("constitutionalization"),
        String::from("paleoceanographically"),
        String::from("electrochromatography"),
        String::from("sesquipedalianistically"),
    ];
    for word in lst.iter() {
        let result = typoglycemia(word);
        assert_eq!(result, *word);
    }
}

#[test]
fn it_ignores_beginning_non_ascii() {
    let input = "❤️Hi";
    let result: String = typoglycemia(input);
    let g = result.graphemes(true).collect::<Vec<&str>>();
    assert_eq!(result, input.to_string());
    assert_eq!(g.get(0), Some(&"❤️"));
}

#[test]
fn it_ignores_ending_non_ascii() {
    let input = "Hi❤️";
    let result: String = typoglycemia(input);
    let g = result.graphemes(true).collect::<Vec<&str>>();

    assert_eq!(result, input.to_string());
    assert_eq!(g.get(2), Some(&"❤️"));
}

#[test]
fn it_ignores_beginning_and_ending_non_ascii() {
    let input = "😈Hi❤️";
    let result: String = typoglycemia(input);
    let g = result.graphemes(true).collect::<Vec<&str>>();

    assert_eq!(result, input.to_string());
    assert_eq!(g.get(0), Some(&"😈"));
    assert_eq!(g.get(3), Some(&"❤️"));
}

#[test]
/**
 * Example output, The Raven by E.A. Poe (English)
 * $cargo test -- --show-output
 */
fn poe_the_raven_english() {
    let input: &'static str = "Once upon a midnight dreary, while I pondered, weak and weary, \
    Over many a quaint and curious volume of forgotten lore, \
    While I nodded, nearly napping, suddenly there came a tapping, \
    As of some one gently rapping, rapping at my chamber door.";
    let result: String = typoglycemia(input);
    println!("");
    println!("{}", "*".repeat(40));
    println!("Integration test example ouput: poe_the_raven_english()");
    println!("{}", "*".repeat(40));
    println!("Original:\n");
    println!("{}", input);
    println!("\nResult:\n");
    println!("{}", result);
    assert_eq!(1, 1);
}

#[test]
/**
 * Example output, The Raven by E.A. Poe (French)
 * $cargo test -- --show-output
 */
fn poe_the_raven_french() {
    let input = "Jadis, par une minuit lugubre, tandis que je pensais, faible et las, à maints \
    grimoires oubliés, et que je hochais la tête, presque endormi, soudain il se fit un heurt, \
    comme de quelqu'un qui frapperait doucement, frappant à la porte de ma chambre";
    let result = typoglycemia(input);

    println!("");
    println!("{}", "*".repeat(40));
    println!("Integration test example ouput: poe_the_raven_french()");
    println!("{}", "*".repeat(40));
    println!("Original:\n");
    println!("{}", input);
    println!("\nResult:\n");
    println!("{}", result);
    assert_eq!(1, 1);
}

#[test]
/**
 * Example output, The Raven by E.A. Poe (English)
 * $cargo test -- --show-output
 */
fn poe_the_raven_german() {
    let input = "Einst in einer Mittnacht schaurig, als ich in entschwundner Kunde wunderlicher Bücher forschte, \
    bis mein Geist die Kraft verlor, und mir's trübe ward im Kopfe, kam mir's plötzlich vor, als klopfe, \
    jemand leis ans Tor, als klopfe - klopfe jemand sacht ans Tor.";
    let result = typoglycemia(input);

    println!("");
    println!("{}", "*".repeat(40));
    println!("Integration test example ouput: poe_the_raven_german()");
    println!("{}", "*".repeat(40));
    println!("Original:\n");
    println!("{}", input);
    println!("\nResult:\n");
    println!("{}", result);
    assert_eq!(1, 1);
}

#[test]
/**
 * Example output, The Gettysburg Address with emojis
 * $cargo test -- --show-output
 */
fn gettysburg_address_with_emojis() {
    let input = "Four score and seven years ago📜, our 🧓fathers brought \
    forth on this continent a new nation, conceived in Liberty, and dedicated to the \
    proposition that all men are created equal. 🇺🇸";
    let result = typoglycemia(input);

    println!("");
    println!("{}", "*".repeat(40));
    println!("Integration test example ouput: gettysburg_address_with_emojis");
    println!("{}", "*".repeat(40));
    println!("Original:\n");
    println!("{}", input);
    println!("\nResult:\n");
    println!("{}", result);
    assert_eq!(1, 1);
}

#[test]
/**
 * Leet output
 * $cargo test -- --show-output
 */
fn typoglycemia_leet_test() {
    let input = "Leet-speak is a mixture of words (mostly computer-related \
    jargon) spelled incorrectly intentionally*, usually coming from typographical errors \
    (e.g. the becomes t3h). The words of Leet-speak are usually put together to create a \
    dialect (small language). This dialect is used in some places for funniness. Leet-speak \
    uses numbers, ASCII symbols, and diacritics together to make symbols that look like \
    Latin letters.";
    let result = typoglycemia_leet(input, 1);

    println!("");
    println!("{}", "*".repeat(40));
    println!("Integration test example ouput: typoglycemia_leet_test()");
    println!("{}", "*".repeat(40));
    println!("Original:\n");
    println!("{}", input);
    println!("\nResult:\n");
    println!("{}", result);
    assert_eq!(1, 1);
}

// ── Tests targeting the code-review fixes ────────────────────────────────────

// Fix #1: ZERO_TO_NINE was 48..57 (exclusive), silently excluding '9' (ASCII 57).
//         Corrected to 48..58 so '9' is a valid word boundary.

#[test]
fn digit_nine_is_valid_end_boundary() {
    // '9' must be anchored as the last character, not shuffled into the middle.
    let result = typoglycemia("testing9"); // t-e-s-t-i-n-g-9, 8 graphemes
    let g: Vec<&str> = result.graphemes(true).collect();
    assert_eq!(g.first(), Some(&"t"));
    assert_eq!(g.last(), Some(&"9"));
    assert_eq!(g.len(), 8);
}

#[test]
fn digit_nine_triggers_starts_with_digit_guard() {
    // A token starting with '9' satisfies starts_with_digit and is returned as-is.
    let result = typoglycemia("9eleven");
    assert_eq!(result, "9eleven");
}

// Fix #2/#4/#5: Latin-1/MISC chars were invisible to boundary detection because
//               the old code used is_ascii() + bytes[0], which returns false for
//               any codepoint > 127.  Now uses `c as usize` (Unicode codepoint).

#[test]
fn latin1_char_is_valid_start_boundary() {
    // 'é' (U+00E9 = codepoint 233) is in LATIN_1 (192..247) and must be
    // anchored as the first grapheme of "école".
    let result = typoglycemia("école"); // é-c-o-l-e, 5 graphemes
    let g: Vec<&str> = result.graphemes(true).collect();
    assert_eq!(g.first(), Some(&"é"));
    assert_eq!(g.last(), Some(&"e"));
    assert_eq!(g.len(), 5);
}

#[test]
fn latin1_char_is_valid_end_boundary() {
    // 'é' (U+00E9 = codepoint 233, LATIN_1) must be anchored as the last grapheme
    // of "café".  Old code skipped it and incorrectly protected 'f' instead.
    let result = typoglycemia("café"); // c-a-f-é, 4 graphemes
    let g: Vec<&str> = result.graphemes(true).collect();
    assert_eq!(g.first(), Some(&"c"));
    assert_eq!(g.last(), Some(&"é"));
    assert_eq!(g.len(), 4);
}

#[test]
fn latin2_char_is_valid_start_boundary() {
    // 'ü' (U+00FC = codepoint 252) is in LATIN_2 (248..256) and must be
    // anchored as the first grapheme of "übersee".
    let result = typoglycemia("übersee"); // ü-b-e-r-s-e-e, 7 graphemes
    let g: Vec<&str> = result.graphemes(true).collect();
    assert_eq!(g.first(), Some(&"ü"));
    assert_eq!(g.last(), Some(&"e"));
    assert_eq!(g.len(), 7);
}

#[test]
fn misc_char_is_valid_start_boundary() {
    // 'Š' (U+0160 = codepoint 352) is in MISC_CHARS.  Before the fix, MISC_CHARS
    // stored Windows-1252 byte values (138 for Š) instead of Unicode codepoints,
    // so 'Š' could never match and 'i' was incorrectly used as the start boundary.
    let result = typoglycemia("Šimon"); // Š-i-m-o-n, 5 graphemes
    let g: Vec<&str> = result.graphemes(true).collect();
    assert_eq!(g.first(), Some(&"Š"));
    assert_eq!(g.last(), Some(&"n"));
    assert_eq!(g.len(), 5);
}

#[test]
fn non_alpha_prefix_with_latin1_chars_has_correct_boundaries() {
    // "__àéîõ__": underscores wrap four Latin-1 chars (all codepoints 192..247).
    // Old code: is_ascii() blocked all Latin-1 chars; the silent default of 0
    // set both boundaries to the underscores, causing them to be shuffled into
    // the interior along with the accented characters.
    // New code: codepoint check correctly identifies 'à' (224) as the start
    // boundary and 'õ' (245) as the end boundary.
    let input = "__àéîõ__"; // 8 graphemes: _,_,à,é,î,õ,_,_
    let result = typoglycemia(input);
    let g: Vec<&str> = result.graphemes(true).collect();
    assert_eq!(g.get(0), Some(&"_"), "leading '_' at [0] must not move");
    assert_eq!(g.get(1), Some(&"_"), "leading '_' at [1] must not move");
    assert_eq!(g.get(2), Some(&"à"), "'à' is the protected start boundary");
    assert_eq!(g.get(5), Some(&"õ"), "'õ' is the protected end boundary");
    assert_eq!(g.get(6), Some(&"_"), "trailing '_' at [6] must not move");
    assert_eq!(g.get(7), Some(&"_"), "trailing '_' at [7] must not move");
}

// Fix #3/#6: get_valid_start/end_index now return Option<usize>.  scramble_word
//            returns the token unchanged on None rather than silently defaulting
//            to index 0, which previously caused all-non-ASCII tokens to be shuffled.

#[test]
fn all_non_ascii_token_is_returned_unchanged() {
    // A token composed entirely of emoji has no valid alphanumeric boundary.
    // Old code: None-case defaulted to index 0, shuffling the interior graphemes.
    // New code: Option::None propagates to an early return of the original string.
    let input = "😈❤️🔥🎉😊🌟"; // 6 graphemes, no valid alphanumeric characters
    let result = typoglycemia(input);
    assert_eq!(result, input);
}

// Fix #7: is_numeric_string renamed to starts_with_digit (behavior unchanged).

#[test]
fn numeric_prefix_tokens_are_returned_unchanged() {
    // Tokens whose leading bytes parse as digits are passed through untouched.
    // atoi stops at the first non-digit, so this covers dates, decimals, and
    // mixed numeric strings (the function checks the prefix, not the whole value).
    let cases = [
        ("12/22/1986", "date"),
        ("3.1415926", "decimal"),
        ("9lives", "digit-prefixed word"),
    ];
    for (input, label) in cases.iter() {
        assert_eq!(
            typoglycemia(input),
            *input,
            "{} \"{}\" should be returned unchanged",
            label,
            input
        );
    }
}
