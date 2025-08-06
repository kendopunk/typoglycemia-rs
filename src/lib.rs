mod utilities;

pub use crate::utilities::utils; // for doctest
use crate::utilities::utils::scramble_word;

/// typoglycemia() takes a string input and will scramble it according to  
/// typoglycemic rules, i.e. where the first and last character of each word or  
/// chunk remains in their position, respectively but interior characters  
/// are randomly shuffled, e.g. "hello" => "hlelo"
///
/// # Arguments
///
/// - `s` (`&str`) - The input string or sentence
///
/// # Returns
///
/// - `String` - A typoglycemified String object
///
/// # Examples
///
/// ```
/// use typoglycemia::typoglycemia;
/// let result = typoglycemia("hello world");
/// let parts: Vec<&str> = result.split_whitespace().collect();
///
/// let v1 = vec!["hello", "hlelo", "hlleo"];
/// let v2 = vec!["world", "wolrd", "wlord", "wlrod", "wrlod", "wrold"];
///
/// let first_scrambled_word = parts.get(0).unwrap();
/// let second_scrambled_word = parts.get(1).unwrap();
///
/// assert!(v1.contains(first_scrambled_word));
/// assert!(v2.contains(second_scrambled_word));
///
/// ```
pub fn typoglycemia(s: &str) -> String {
    let tokens: Vec<&str> = s.split_whitespace().collect();
    let mut vec_of_scrambles: Vec<String> = Vec::new();

    for tok in tokens.iter() {
        vec_of_scrambles.push(scramble_word(tok.to_string()));
    }
    vec_of_scrambles.join(" ")
}

/// typoglycemia_leet() behaves the same as typoglycemia() but will do a  
/// Leet-like substitution for certain characters, depending on the  
/// level chosen.
///
/// Examples:  
/// 'A' => 'Д'  
/// 'B' | 'b' => '8'  
/// 'c' => '¢'  
/// 'E' => '€'  
/// 'e' => '3'  
/// 'H' => 'н'  
/// 'I' | 'i' => '1'  
/// 'M' => 'м'  
/// 'N' => 'И'  
/// 'n' => 'и'  
/// 'O' => '0'  
/// 'R' => 'Я'  
/// 'S' | 's' => '$'  
/// 'v' => '√'  
/// 'W' => 'Ш'  
/// 'Y' | 'y' => 'Ч'  
/// '0' => 'O'  
///
///
/// # Arguments
///
/// - `s` (`&str`) - The input string or sentence
/// - `level` (`u8`) - 1-3: Transliteration level with 1 being the most human readable
///
/// # Returns
///
/// - `String` - A typoglycemified String object
///
/// # Examples
///
/// ```
/// use typoglycemia::typoglycemia_leet;
/// let result = typoglycemia_leet("Rover", 1);
/// println!("{}", result);
///
/// let v1 = vec![
/// String::from("Яo√er"),
/// String::from("Яoe√r"),
/// String::from("Я√eor"),
/// String::from("Я√oer"),
/// String::from("Яeo√r"),
/// String::from("Яe√or")
/// ];
///
/// assert!(v1.contains(&result));
///
pub fn typoglycemia_leet(s: &str, level: u8) -> String {
    let tg: String = typoglycemia(s);

    if level == 3 {
        return tg
            .chars()
            .map(|x| match x {
                'A' => 'Д',
                'B' | 'b' => '8',
                'c' => '¢',
                'E' => '€',
                'e' => '3',
                'H' => 'н',
                'I' | 'i' => '1',
                'M' => 'м',
                'N' => 'И',
                'n' => 'и',
                'O' => '0',
                'R' => 'Я',
                'S' | 's' => '$',
                'v' => '√',
                'W' => 'Ш',
                'Y' | 'y' => 'Ч',
                '0' => 'O',
                _ => x,
            })
            .collect();
    } else if level == 2 {
        return tg
            .chars()
            .map(|x| match x {
                'A' => 'Д',
                'B' | 'b' => '8',
                'E' | 'e' => '3',
                'I' | 'i' => '1',
                'N' => 'И',
                'n' => 'и',
                'O' => '0',
                'R' => 'Я',
                'S' => '$',
                'v' => '√',
                'W' => 'Ш',
                '0' => 'O',
                _ => x,
            })
            .collect();
    } else {
        return tg
            .chars()
            .map(|x| match x {
                'A' => 'Д',
                'B' => '8',
                'i' => '1',
                'O' => '0',
                'R' => 'Я',
                'v' => '√',
                'W' => 'Ш',
                '0' => 'O',
                _ => x,
            })
            .collect();
    }
}
