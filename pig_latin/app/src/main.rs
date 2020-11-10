fn main() {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    let words = "foo bar apple juice out";

    let mut pig_latin = String::new();
    for word in words.split_whitespace() {
        let mut chars = word.chars();
        let first_char = chars.nth(0).unwrap();
        if VOWELS.contains(&first_char) {
            pig_latin.push_str(word);
        } else {
            for rest in chars {
                pig_latin.push(rest);
            }
        }

        pig_latin.push(' ');
    }

    println!("{}", pig_latin);
}
