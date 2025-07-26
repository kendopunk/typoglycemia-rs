mod utilities;

pub use crate::utilities::utils;
use crate::utilities::utils::scramble_word;

/// tg_standard() takes a str as input and will scramble it according to typoglycemia rules,  
/// where the first and last character of every word or chunk will retain their  
/// first/last positions, respectively but interior characters will be scrambled.
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
/// use typoglycemia::tg_standard;
/// let result = tg_standard("hello world");
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
pub fn tg_standard(s: &str) -> String {
    let tokens: Vec<&str> = s.trim().split_whitespace().collect();
    let mut vec_of_scrambles: Vec<String> = Vec::new();

    for tok in tokens.iter() {
        vec_of_scrambles.push(scramble_word(tok.to_string()));
    }
    vec_of_scrambles.join(" ")
}

/// tg_plus() behaves the same as tg_standard() but will replace certain characters  
/// with numeric "look-alikes", so to speak.
///
/// - 'B' => '8'  
/// - 'e' => '3'  
/// - 'i' | 'I' => '1'   
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
/// use typoglycemia::tg_plus;
/// let result = tg_plus("hello world");
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
pub fn tg_plus(s: &str) -> String {
    let result = tg_standard(s);

    String::from("foo")
    // let tokens: Vec<&str> = s.trim().split_whitespace().collect();
    // let mut vec_of_scrambles: Vec<String> = Vec::new();

    // for tok in tokens.iter() {
    //     vec_of_scrambles.push(scramble_word(tok.to_string()));
    // }
    // vec_of_scrambles.join(" ")
}
