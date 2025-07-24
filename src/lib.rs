mod utilities;

pub use crate::utilities::utils;
use crate::utilities::utils::{handle_hyphenated_string, has_hyphens, scramble_word};

/// typoglycemia() takes a str and will scramble it according to typoglycemia rules,  
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
    let parts: Vec<&str> = s.split_whitespace().collect();
    let mut vec_of_scrambles: Vec<String> = Vec::new();

    for chunk in parts.iter() {
        let hashyph: bool = has_hyphens(chunk);
        if hashyph {
            vec_of_scrambles.push(handle_hyphenated_string(chunk));
        } else {
            vec_of_scrambles.push(scramble_word(chunk.to_string()));
        }
    }
    vec_of_scrambles.join(" ")
}
