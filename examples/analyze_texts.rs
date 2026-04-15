use pycld2::detect_language;
use std::default::Default;

static TEXTS: &'static [&'static str] = &[
    // English.
    // From Coleridge's "The Rime of the Ancient Mariner".
    "It is an ancient Mariner,
And he stoppeth one of three.
'By thy long grey beard and glittering eye,
Now wherefore stopp'st thou me?",
    // French.
    // Traditional children's song.
    "Sur le pont d'Avignon,
L'on y danse, l'on y danse,
Sur le pont d'Avignon
L'on y danse tous en rond.
Les belles dames font comme ça
Et puis encore comme ça.
Les messieurs font comme ça
Et puis encore comme ça.",
    // Mixed French and English.
    // Combination of the two above.
    "It is an ancient Mariner,
And he stoppeth one of three.
'By thy long grey beard and glittering eye,
Now wherefore stopp'st thou me?

Sur le pont d'Avignon,
L'on y danse, l'on y danse,
Sur le pont d'Avignon
L'on y danse tous en rond.
Les belles dames font comme ça
Et puis encore comme ça.
Les messieurs font comme ça
Et puis encore comme ça.",
    // Middle Egyptian.
    // ("rA n(y) pr(i).t m hrw" or "The Book of Going Forth by Day")
    //
    // This is intended to test Unicode characters that don't fit into 16
    // bits, and to see whether cld2 can detect obscure languages using
    // nothing but script data.
    "𓂋𓏤𓈖𓉐𓂋𓏏𓂻𓅓𓉔𓂋𓅱𓇳",
    // Short text.
    "blah",
];

fn main() {
    for (i, &text) in TEXTS.iter().enumerate() {
        println!("=== Text #{}\n", i + 1);

        let detected = detect_language(text, &Default::default());

        println!("Language: {:?}", detected.language);
        println!("Reliable: {:?}", detected.reliable);
        println!("Bytes of text: {}", detected.text_bytes);
        println!("\n= Per-language scores:\n");

        for score in detected.scores.iter() {
            println!("Language: {:?}", score.language);
            println!("Percent of input: {}%", score.percent);
            println!("Norm: {}\n", score.normalized_score);
        }
    }
}
