use typoglycemia::typoglycemia;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
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
fn it_handles_a_sentence_one() {
    let input = "Once midnight";
    let result = typoglycemia(input);
    println!("{:?}", result);

    assert_eq!(1, 1);
    // let input = "Once upon a midnight dreary, while I pondered, weak and weary, Over many a quaint and curious volume of forgotten lore— While I nodded, nearly napping, suddenly there came a tapping, As of some one gently rapping, rapping at my chamber door.";
    // let result: String = typoglycemia(input);
    // let g = result.graphemes(true).collect::<Vec<&str>>();

    // println!("{}", result);

    // assert_eq!(result, input.to_string());

    // // let input = "😈Hi❤️";
    // // let result: String = typoglycemia(input);
    // // let g = result.graphemes(true).collect::<Vec<&str>>();

    // // assert_eq!(result, input.to_string());
    // // assert_eq!(g.get(0), Some(&"😈"));
    // // assert_eq!(g.get(3), Some(&"❤️"));
}
