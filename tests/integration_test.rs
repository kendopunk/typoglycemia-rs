use typoglycemia::tg_standard;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
#[test]
fn it_handles_a_string_slice() {
    let s: &str = "slice";
    let result = tg_standard(s);
    assert_eq!(result.chars().nth(0), Some('s'));
    assert_eq!(result.chars().nth(4), Some('e'));
}

#[test]
fn it_does_not_typoglycemify_short_words() {
    let lst = [String::from("a"), String::from("an"), String::from("foo")];
    for word in lst.iter() {
        let result = tg_standard(word);
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
        let result = tg_standard(word);
        assert_eq!(result, *word);
    }
}

#[test]
fn it_ignores_beginning_non_ascii() {
    let input = "❤️Hi";
    let result: String = tg_standard(input);
    let g = result.graphemes(true).collect::<Vec<&str>>();
    assert_eq!(result, input.to_string());
    assert_eq!(g.get(0), Some(&"❤️"));
}

#[test]
fn it_ignores_ending_non_ascii() {
    let input = "Hi❤️";
    let result: String = tg_standard(input);
    let g = result.graphemes(true).collect::<Vec<&str>>();

    assert_eq!(result, input.to_string());
    assert_eq!(g.get(2), Some(&"❤️"));
}

#[test]
fn it_ignores_beginning_and_ending_non_ascii() {
    let input = "😈Hi❤️";
    let result: String = tg_standard(input);
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
fn example_raven_english() {
    let input = "Once upon a midnight dreary, while I pondered, weak and weary, \
    Over many a quaint and curious volume of forgotten lore, \
    While I nodded, nearly napping, suddenly there came a tapping, \
    As of some one gently rapping, rapping at my chamber door.";
    let result = tg_standard(input);

    println!("");
    println!("{}", "*".repeat(40));
    println!("Integration test example ouput: example_raven_english()");
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
fn example_raven_french() {
    let input = "Jadis, par une minuit lugubre, tandis que je pensais, faible et las, à maints \
    grimoires oubliés, et que je hochais la tête, presque endormi, soudain il se fit un heurt, \
    comme de quelqu'un qui frapperait doucement, frappant à la porte de ma chambre";
    let result = tg_standard(input);

    println!("");
    println!("{}", "*".repeat(40));
    println!("Integration test example ouput: example_raven_french()");
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
fn example_raven_german() {
    let input = "Einst in einer Mittnacht schaurig, als ich in entschwundner Kunde wunderlicher Bücher forschte, bis mein Geist die Kraft verlor, und mir's trübe ward im Kopfe, kam mir's plötzlich vor, als klopfe, jemand leis ans Tor, als klopfe - klopfe jemand sacht ans Tor.";
    let result = tg_standard(input);

    println!("");
    println!("{}", "*".repeat(40));
    println!("Integration test example ouput: example_raven_german()");
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
fn example_gettysburg_emojies() {
    let input = "Four score and seven years ago📜, our 🧓fathers brought \
    forth on this continent a new nation, conceived in Liberty, and dedicated to the \
    proposition that all men are created equal. 🇺🇸";
    let result = tg_standard(input);

    println!("");
    println!("{}", "*".repeat(40));
    println!("Integration test example ouput: example_gettysburg_emojis()");
    println!("{}", "*".repeat(40));
    println!("Original:\n");
    println!("{}", input);
    println!("\nResult:\n");
    println!("{}", result);
    assert_eq!(1, 1);
}
