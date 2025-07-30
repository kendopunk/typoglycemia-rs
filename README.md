# typoglycemia-rs

Typoglycemia implementation for Rust, with a Leet-speak option.

## Background

The phenomenon known as typoglycemia, is the ability to understand words when the first and last letters are stable, but the intermediate letters are scrambled. Your brain puts the letters back into a sequence again.

"Adncocrig to a rcheeasrer at Cdmgrbaie Usirveinty, it donse't metatr in waht oerdr  
the ltertes in a wrod are, the olny itapmrnot tinhg is that the frsit and last  
leettr be at the right palce. The rest can be a toatl mess and you can siltl read  
it wuihtot prebolm. Tihs is bsauece the haumn mnid deos not raed ervey letetr by  
isetlf, but the wrod as a wlohe."

"_According to a researcher at Cambridge University, it doesn't matter in what order  
the letters in a word are, the only important thing is that the first and last  
letter be at the right place. The rest can be a total mess and you can still read  
it without problem. This is because the human mind does not read every letter by  
itself, but the word as a whole._"

## Features

- Designed primarily for Latinate languages - English, Spanish, French, etc. - but should work well for Germanic languages<br><br>
- Standard Typoglycemia functionality, e.g.
  - "Once upon a midnight dreary, while I pondered, weak and weary" => "Ocne upon a mnihdigt derray, wilhe I pernoedd, waek and wraey"<br><br>
- Leet-speak function for added complexity, e.g.
  - Only a small subset of Leet substitutions, see lib.rs
  - "Once upon a midnight dreary, while I pondered, weak and weary" => "0cn3 upon a mgd1hn1t dr3ary, whl13 1 podn33rd, wa3k and w3ray"<br><br>
- Hyphenated words will retain their hyphen positions, e.g.
  - "Spanish-speaking country" => "Spsniah-siapenkg cnoruty"<br><br>
- Same with apostrophes
  - "I wouldn't or I wouldn't've" => "I wulodn't or I wludon't've"<br><br>
- For clarify, words beginning with a numeric character, e.g. date, time, colloquialisms, will not be typoglycemified:
  - "12/22/1986" => no change.
  - "1-for-all" => no change.<br><br>
- Words with grapheme length <= 3 or > 20 will also not be typoglycemified
  - "a", "the", "and", "but", "or", "for", "a", "I❤️", etc.

## Usage

## References

- [Chunking: The Brain’s Shortcut to Understanding and Recalling Information (observer.com)](https://observer.com/2017/03/chunking-typoglycemia-brain-consume-information/)
