use std::io::stdin;

/**
 * Convert strings to pig latin.
 * The first consonant of each word is moved to the end of the word and “ay” is added,
 * so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
 * (“apple” becomes “apple-hay”).
 * Keep in mind the details about UTF-8 encoding!
 */

pub fn read_and_process() {
    let mut input_buffer = String::new();
    println!("Please send the input that will be translated to pig latin");
    // Read from stdin
    stdin()
        .read_line(&mut input_buffer)
        .expect("Fail to read from terminal!");
    let input_buffer = input_buffer.trim();

    // Split the words
    let splitted = input_buffer.split_whitespace();

    // Create the output string
    let mut output = Vec::new();

    for word in splitted {
        // Get the first char
        let first_char = word.chars().nth(0);

        // Only operate if the first character is present
        if let Some(first_char) = first_char {
            let is_vowel = "aeiou".contains(first_char);
            if is_vowel {
                output.push(String::from(word) + "-hay");
            } else {
                output.push(
                    String::from(&word[first_char.len_utf8()..])
                        + "-"
                        + &first_char.to_string()
                        + "ay",
                );
            }
        }
    }
    println!("Translated text: {}", output.join(" "));
}
