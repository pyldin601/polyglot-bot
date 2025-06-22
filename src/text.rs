pub(crate) fn split_text_into_chunks(text: &str, max_len: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    let mut sentence_start = 0;

    let chars: Vec<char> = text.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c == '.' || c == '!' || c == '?' {
            let sentence: String = chars[sentence_start..=i].iter().collect();
            let sentence = sentence.trim();

            if !sentence.is_empty() {
                if current_chunk.len() + sentence.len() + 1 > max_len {
                    if !current_chunk.is_empty() {
                        chunks.push(current_chunk.trim().to_string());
                        current_chunk = String::new();
                    }
                }
                current_chunk.push_str(sentence);
                current_chunk.push(' ');
            }

            sentence_start = i + 1; // наступне речення
        }
    }

    if sentence_start < chars.len() {
        let sentence: String = chars[sentence_start..].iter().collect();
        let sentence = sentence.trim();
        if !sentence.is_empty() {
            if current_chunk.len() + sentence.len() + 1 > max_len {
                if !current_chunk.is_empty() {
                    chunks.push(current_chunk.trim().to_string());
                    current_chunk = String::new();
                }
            }
            current_chunk.push_str(sentence);
        }
    }

    if !current_chunk.trim().is_empty() {
        chunks.push(current_chunk.trim().to_string());
    }

    chunks
}
