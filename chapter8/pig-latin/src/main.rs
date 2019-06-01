// Convert strings to pig latin. The first consonant of each word is moved to
// the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
// that start with a vowel have “hay” added to the end instead (“apple” becomes
// “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn main() {
    let input = "hello to the world of rust";
    println!("'{}' -> '{}'", input, pig_latin(input));
}

fn pig_latin(text: &str) -> String {
    let mut result = String::new();

    for word in text.split_whitespace() {
        let translated = pig_latin_word(word);
        result.push_str(&translated);
        result.push(' ');
    }

    result.pop();
    result
}

fn pig_latin_word(word: &str) -> String {
    let (first_letter, remainder) = word.split_at(1);

    match first_letter {
        "a" | "e" | "i" | "o" | "u" => format!("{}-hay", word),
        _ => format!("{}-{}ay", remainder, first_letter),
    }
}
