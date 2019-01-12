// 500
pub fn find_words(words: Vec<String>) -> Vec<String> {
    let keyboard_rows = vec!["qwertyuiop", "asdfghjkl", "zxcvbnm"];
    let mut result_words = vec![];

    for word in words {
        let word_copy = word.clone();
        let word = word.to_lowercase();
        let first_char = word.chars().next().unwrap();
        let mut target_row = "";
        for row in &keyboard_rows {
            if row.contains(first_char) {
                target_row = *row;
                break;
            }
        }
        let mut in_one_line = true;

        for c in word.chars() {
            if !target_row.contains(c) {
                in_one_line = false;
                break;
            }
        }

        if in_one_line {
            result_words.push(word_copy)
        }
    }

    return result_words;
}
