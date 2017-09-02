pub fn split_at_char(input: &str, ch: char) -> (String, String) {
    let length = input.len();
    let split_at = input.chars()
        .enumerate()
        .position(|(_, c)| c == ch)
        .unwrap_or(length);

    let first = (&input[0..split_at]).to_string();
    let second = if split_at == length {
        "".to_string()
    } else {
        (&input[(split_at + 1)..length]).to_string()
    };

    return (first, second);
}

pub fn split_at_space(input: &str) -> (String, String) {
    split_at_char(input, ' ')
}
