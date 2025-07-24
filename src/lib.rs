mod utilities;

pub use crate::utilities::utils;
use crate::utilities::utils::{has_hyphens, jumble};

/// Creates a person with the foo name.
///
/// # Examples
///
///
// /// pub fn to_lowercase(input: &str) -> String {
//     input.to_lowercase()
// }
pub fn typoglycemia<T: Into<String>>(input: T) -> String {
    let s = input.into();
    let parts: Vec<&str> = s.split_whitespace().collect();
    let mut collector: Vec<String> = Vec::new();

    for chunk in parts.iter() {
        let hh = has_hyphens(chunk);
        if hh {
            collector.push(utils::handle_hyphen_string(chunk));
        } else {
            collector.push(jumble(chunk.to_string()));
        }
    }
    collector.join(" ")
}

// pub fn typoglycemia<T: Into<str>>(input: T) -> String {
//     let hello_world: &'static str = "Hello, world!";
//     // let input_string: String = input.into();

//     // let x = String::from("hello there dude");
//     // let parts: Vec<_> = x.split_whitespace().collect();
//     // // let parts: Vec<&str> = *input_string.split_whitespace().collect::<Vec<&str>>();
//     // let mut collector: Vec<String> = Vec::new();

//     // // //

//     // for chunk in parts.iter() {
//     //     if has_hyphens(chunk) {
//     //         collector.push(utils::handle_hyphen_string(chunk));
//     //     } else {
//     //         collector.push(jumble(chunk.to_string()));
//     //     }
//     // }

//     // collector.join(" ")

//     String::from("Fred")
// }

// pub fn to_lowercase<S: AsRef<str>>(input: S) -> String {
//     input.as_ref().to_lowercase()
// }
// pub fn typoglycemia(left: u64, right: u64) -> u64 {
//     left + right
// }

// mod ascii
// mod utilities

// #[cfg(test)]
// mod integration_test;

// integration_test/mod.rs

// #[cfg(test)]
// mod tests {
//     use super::*;

// #[test]
// fn it_works() {
//     let result = add(2, 2);
//     assert_eq!(result, 4);
// }

//     #[test]
//     fn it_handles_string_object() {
//         let result = typoglycemia(String::from("Marge"));
//         assert_eq!(result, String::from("Fred"));
//     }
//     #[test]
//     fn it_handles_string_slice() {
//         let result = typoglycemia("blah");
//         assert_eq!(result, String::from("Fred"));
//     }
// }
