pub fn to_latin_text(text: &str) -> String {
    let mut latin = String::new();
    for word in text.split_whitespace() {
        let mut word_chars: std::str::Chars<'_> = word.chars();
        if let Some(ch) = word_chars.next() {
            match ch {
                'a' | 'e' | 'i' | 'o' | 'y' | 'u' => {
                    let temp_word = format!("{word}-hay ");
                    latin.push_str(&temp_word);
                }
                _ => {
                    let slice: String = word_chars.collect();
                    let temp_word = format!("{slice}-{ch}ay ");
                    latin.push_str(&temp_word);
                }
            }
        }
    }
    latin
}
