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

/// leet() behaves the same as typoglycemia() but will do a Leet-like substitution  
/// for certain characters  
///
/// 'A' => 'Д'  
/// 'B' | 'b' => '8'  
/// 'E' | 'e' => '3'  
/// 'I' | 'i' => '1'  
/// 'O' => '0'  
/// 'R' => 'Я'  
/// 'T' => '†'  
/// 'V' | 'v' => '√'  
/// 'W' => 'Ш'  
/// '0' => 'O'  
/// '2' => 'Z'  
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
/// use typoglycemia::leet;
/// let result = leet("Real Leet");
/// let parts: Vec<&str> = result.split_whitespace().collect();
///
/// let v1 = vec!["Я3al", "Яa3l"];
/// let v2 = vec!["L33t"];
///
/// let first_scrambled_word = parts.get(0).unwrap();
/// let second_scrambled_word = parts.get(1).unwrap();
///
/// assert!(v1.contains(first_scrambled_word));
/// assert!(v2.contains(second_scrambled_word));
///
/// ```
pub fn leet(s: &str) -> String {
    let tg: String = typoglycemia(s);

    tg.chars()
        .map(|x| match x {
            'A' => 'Д',
            'B' | 'b' => '8',
            'E' | 'e' => '3',
            'I' | 'i' => '1',
            'O' => '0',
            'R' => 'Я',
            'T' => '†',
            'V' | 'v' => '√',
            'W' => 'Ш',
            '0' => 'O',
            '2' => 'Z',
            _ => x,
        })
        .collect()
}
