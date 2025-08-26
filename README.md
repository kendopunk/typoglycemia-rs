# typoglycemia-rs

Typoglycemia implementation for Rust, with a Leet-speak variant.

## Background

The phenomenon known as typoglycemia describes the (unproven) cognitive ability to read a word when the first and last letters are stable, but the intermediate letters are scrambled.

"Adncocrig to a rcheeasrer at Cdmgrbaie Usirveinty, it donse't metatr in waht oerdr the ltertes in a wrod are, the olny itapmrnot tinhg is that the frsit and last leettr be at the right palce. The rest can be a toatl mess and you can siltl read it wuihtot prebolm. Tihs is bsauece the haumn mnid deos not raed ervey letetr by isetlf, but the wrod as a wlohe."

"_According to a researcher at Cambridge University, it doesn't matter in what order the letters in a word are, the only important thing is that the first and last letter be at the right place. The rest can be a total mess and you can still read it without problem. This is because the human mind does not read every letter by itself, but the word as a whole._"

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
- For clarity, words beginning with a numeric character, e.g. date, time, colloquialisms, will not be typoglycemified:
  - "12/22/1986" => no change.
  - "1-for-all" => no change.<br><br>
- Words with grapheme length <= 3 or > 15 will also not be typoglycemified
  - "a", "the", "and", "but", "or", "for", "a", "I❤️", "antidisestablishmentarianism", etc.

## Usage

```
use typoglycemia::{typoglycemia, typoglycemia_leet};

fn main() {
    let s = "It was the best of times, it was the worst of \
    times, it was the age of wisdom, it was the age of \
    foolishness, it was the epoch of belief, it was the epoch \
    of incredulity, it was the season of Light, it was the \
    season of Darkness, it was the spring of hope, it was \
    the winter of despair...";

    let t = typoglycemia(s);
    println!("{}", t);

    // It was the bset of tiems, it was the wrsot of
    // tiems, it was the age of wdsiom, it was the age of
    // fssenohilos, it was the epoch of beelif, it was the epoch
    // of iledruicnty, it was the seosan of Lhgit, it was the
    // saeosn of Dnaserks, it was the sinprg of hpoe, it was
    // the wtiner of dpaiser...

    let l = typoglycemia_leet(s, 1);
    println!("{}", l);

    // 1t was th3 83st of tm31s, 1t was th3 wrsot of
    // tm31s, 1t was th3 ag3 of wdo1sm, 1t was th3 ag3 of
    // fsoslhon31s, 1t was th3 3ocph of 83l13f, 1t was th3 3poch
    // of 13c1ltrdnuy, 1t was th3 soas3n of Lhg1t, 1t was th3
    // soa3sn of Dkr3nass, 1t was th3 snpr1g of hpo3, 1t was
    // th3 w1n3tr of d1p3asr...

    let e = typoglycemia(
        "Emojis can convey emotions that might be difficult \
        to express through text alone. For example, a smiley \
        face 😊 can show happiness, while a sad face 😞 can \
        express sadness. Emojis can also emphasize certain \
        words or phrases. For example, using a thumbs-up emoji👍 \
        after a positive statement can reinforce the message. \
        It's best to use emojis sparingly to avoid overwhelming \
        the reader and maintain clarity.",
    );
    println!("{}", e);

    // Emijos can cvnoey enitooms taht mhigt be dciulfift
    // to eseprxs tourhgh text anole.  For eamplxe, a simely
    // fcae 😊 can sohw hnpaesips, wihle a sad fcae 😞 can
    // epexrss sasneds. Eojmis can aslo eihmazspe ciaertn
    // wdros or psrheas. For expmlae, using a tubmhs-up emjoi👍
    // after a pvtiiose statenemt can rencrfoie the mgsasee.
    // It's bset to use eiomjs srpgliany to aovid oeenwlvrmhig
    // the reedar and miaiatnn cialrty.
}
```

## Docs/Testing

```sh
$> cargo doc --no-deps --document-private-items
$> cargo test -- --show-output # display some examples
```

## References

- [Chunking: The Brain’s Shortcut to Understanding and Recalling Information (observer.com)](https://observer.com/2017/03/chunking-typoglycemia-brain-consume-information/)
