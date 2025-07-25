use typoglycemia::typoglycemia;
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

// #[test]
// /**
//  * Example output
//  * $cargo test -- --show-output
//  */
// fn example_1() {
//     let input = "Once upon a midnight dreary, while I pondered, weak and weary, \
//     Over many a quaint and curious volume of forgotten lore, \
//     While I nodded, nearly napping, suddenly there came a tapping, \
//     As of some one gently rapping, rapping at my chamber door.";
//     let result = typoglycemia(input);
//     println!("{}", result);
//     assert_eq!(1, 1);
// }

// #[test]
// /**
//  * Example output w/ some emojis
//  * $cargo test -- --show-output
//  */
// fn example_2() {
//     let input = "Four score and seven years ago 📜, our fathers🧓 brought forth on this \
//     continent a new ❤️nation, conceived in Liberty, and dedicated to the\
//     proposition that all men are created equal. 🇺🇸";
//     let result = typoglycemia(input);
//     println!("{}", result);
//     assert_eq!(2, 2);
// }
