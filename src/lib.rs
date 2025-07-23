// use atoi::atoi;
// use rand::rng;
// use rand::seq::SliceRandom;
// use std::ops::Range;
// use unicode_segmentation::UnicodeSegmentation;

mod utilities;

pub use crate::utilities::utils;

// // O-9
// pub const ZERO_TO_NINE: Range<i32> = 48..57;
// // A - Z
// pub const UCASE_AZ: Range<i32> = 65..91;
// // a - z
// pub const LCASE_AZ: Range<i32> = 97..123;
// // À - ö
// pub const LATIN_1: Range<i32> = 192..247;
// // ø - ÿ
// pub const LATIN_2: Range<i32> = 248..246;
// // ƒ, Š, Œ, Ž, š, œ, ž, Ÿ
// pub const MISC_CHARS: &[&i32] = &[&131, &138, &140, &142, &154, &156, &158, &159];

/// Creates a person with the foo name.
///
/// # Examples
///
pub fn typoglycemia(_s: impl Into<String>) -> String {
    let foo = "234_2342";
    assert_eq!(utils::is_numeric_string(foo), true);
    String::from("Fred")
}
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
