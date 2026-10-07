//! Utility functions and prompt formatters for AI services.

/// Encodes raw byte slice into standard base64 string.
pub fn base64_encode(data: &[u8]) -> String {
    const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0];
        let b1 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] } else { 0 };

        result.push(CHARSET[(b0 >> 2) as usize] as char);
        result.push(CHARSET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARSET[(((b1 & 0x0F) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARSET[(b2 & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

/// Formats text with standardized instructions for cleanup and tone transformation.
pub fn format_enhancement_prompt(text: &str, tone: &str) -> String {
    let normalized = tone.trim().to_lowercase();
    match normalized.as_str() {
        "clean" | "message-ready" | "clean-language" => {
            format!(
                "You are converting someone's dictated speech into a message they would have typed themselves.\n\
                Remove speech filler words (um, uh, like, you know, etc.), false starts, and speech recognition artifacts.\n\
                Keep all original wording, tone, emotion, humor, and meaning intact. Do not translate.\n\
                Preserve all profanity verbatim.\n\
                The output MUST ONLY be the cleaned text - no commentary.\n\n\
                Input:\n{}\n\n\
                Cleaned output:",
                text
            )
        }
        "casual" => {
            format!(
                "You are refining dictated speech into a casual, relaxed, conversational message.\n\
                Remove speech filler words and mechanical artifacts.\n\
                Keep the tone casual, friendly, and natural, exactly as spoken. Do not formalize words.\n\
                Preserve all information and profanity verbatim.\n\
                The output MUST ONLY be the casual text - no commentary.\n\n\
                Input:\n{}\n\n\
                Casual output:",
                text
            )
        }
        "professional" | "markdown-document" | "structure-1" => {
            format!(
                "You are formatting dictated speech into a well-structured, professional document or note.\n\
                Fix grammar, punctuation, and remove speech filler words.\n\
                Structure the text with clean paragraphs and section headings where appropriate for clarity.\n\
                Preserve all original facts, decisions, details, and meaning.\n\
                The output MUST ONLY be the formatted professional text - no commentary.\n\n\
                Input:\n{}\n\n\
                Professional formatted output:",
                text
            )
        }
        "bullet points" | "bullet_points" | "bullets" | "structure-2" | "structure-3" => {
            format!(
                "Convert the following dictated text into clear, well-organized bullet points.\n\
                Extract distinct points and key ideas using dashes (-).\n\
                Maintain logical ordering and preserve all essential information and details.\n\
                The output MUST ONLY be the bullet points - no preamble or commentary.\n\n\
                Input:\n{}\n\n\
                Bullet points output:",
                text
            )
        }
        "concise" | "short" | "brief" => {
            format!(
                "You are refining dictated speech into a concise, direct message.\n\
                Remove speech filler words, repetition, and extraneous phrasing while keeping all core facts and meaning.\n\
                The output MUST ONLY be the concise text - do NOT generate code, explanations, notes, or commentary.\n\n\
                Input:\n{}\n\n\
                Concise output:",
                text
            )
        }
        _ => {
            format!(
                "Clean up and format the following dictated speech, removing filler words and fixing punctuation.\n\
                The output MUST ONLY be the cleaned text - do NOT generate code, explanations, notes, or commentary.\n\n\
                Input:\n{}\n\n\
                Output:",
                text
            )
        }
    }
}

/// Applies rule-based local formatting and cleanup for speech transcripts offline.
///
/// Removes filler words (um, uh, er, ah, you know), normalizes whitespace,
/// fixes capitalization at sentence starts, and ensures trailing punctuation.
pub fn apply_smart_local_formatting(text: &str, tone: &str) -> String {
    if tone.eq_ignore_ascii_case("raw") {
        return text.to_string();
    }

    if text.trim().is_empty() {
        return String::new();
    }

    let mut cleaned = text.to_string();

    // Filler phrases and words to remove (longest first to avoid partial substring collisions)
    const FILLERS: &[&str] = &["you know", "um", "uh", "er", "ah"];

    for filler in FILLERS {
        cleaned = remove_filler_word(&cleaned, filler);
    }

    // Collapse multiple whitespace characters into a single space
    let mut normalized = String::with_capacity(cleaned.len());
    let mut prev_space = false;
    for ch in cleaned.chars() {
        if ch.is_whitespace() {
            if !prev_space {
                normalized.push(' ');
                prev_space = true;
            }
        } else {
            normalized.push(ch);
            prev_space = false;
        }
    }

    // Clean up spacing around punctuation: " ," -> ",", " ." -> ".", etc.
    let mut punctuation_fixed = normalized
        .replace(" ,", ",")
        .replace(" .", ".")
        .replace(" !", "!")
        .replace(" ?", "?")
        .replace(",,", ",")
        .replace(",.", ".");

    // Remove any leading comma or punctuation
    while punctuation_fixed.starts_with(',') || punctuation_fixed.starts_with(' ') {
        punctuation_fixed.remove(0);
    }

    let trimmed = punctuation_fixed.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    // Capitalize sentence starts
    let mut capitalized = String::with_capacity(trimmed.len());
    let mut capitalize_next = true;

    for ch in trimmed.chars() {
        if capitalize_next && ch.is_alphabetic() {
            capitalized.extend(ch.to_uppercase());
            capitalize_next = false;
        } else {
            capitalized.push(ch);
            if ch == '.' || ch == '!' || ch == '?' {
                capitalize_next = true;
            }
        }
    }

    // Ensure sentence ends with punctuation ('.', '!', or '?')
    let mut final_text = capitalized.trim().to_string();
    if !final_text.is_empty() && !final_text.ends_with(['.', '!', '?']) {
        final_text.push('.');
    }

    final_text
}

fn remove_filler_word(text: &str, filler: &str) -> String {
    let filler_len = filler.len();
    let mut result = String::with_capacity(text.len());
    let mut i = 0;

    while i < text.len() {
        let is_match = text[i..]
            .get(..filler_len)
            .map(|s| s.eq_ignore_ascii_case(filler))
            .unwrap_or(false);

        if is_match {
            let before_boundary = i == 0
                || !text[..i]
                    .chars()
                    .last()
                    .map(|c| c.is_alphanumeric())
                    .unwrap_or(false);
            let next_index = i + filler_len;
            let after_boundary = next_index >= text.len()
                || !text[next_index..]
                    .chars()
                    .next()
                    .map(|c| c.is_alphanumeric())
                    .unwrap_or(false);

            if before_boundary && after_boundary {
                // If there's a trailing comma attached to the filler word (e.g. "um,"), consume it
                let mut advance_to = next_index;
                if next_index < text.len() && text[next_index..].starts_with(',') {
                    advance_to += 1;
                }
                i = advance_to;
                result.push(' ');
                continue;
            }
        }

        let ch = text[i..].chars().next().unwrap();
        result.push(ch);
        i += ch.len_utf8();
    }

    result
}

/// Standard HTTP client configured with sensible timeouts for AI provider network requests.
pub fn default_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
        assert_eq!(base64_encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn test_apply_smart_local_formatting() {
        let raw = "um hello world   this is a test  uh you know ";
        let clean = apply_smart_local_formatting(raw, "Clean");
        assert_eq!(clean, "Hello world this is a test.");

        let raw_passthrough = apply_smart_local_formatting(raw, "Raw");
        assert_eq!(raw_passthrough, raw);
    }
}
