use std::char;

pub fn decode_unicode(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.peek() {
                Some('u') => {
                    // consume 'u'
                    chars.next();

                    // check '{'
                    let is_left_braced = chars.peek() == Some(&'{');
                    let mut is_right_braced = false;
                    if is_left_braced {
                        // consume '{'
                        chars.next();
                    }

                    // collect hex
                    let mut hex_str = String::new();
                    while let Some(&next) = chars.peek() {
                        if is_left_braced && next == '}' {
                            // consume '}'
                            chars.next();
                            is_right_braced = true;
                            break;
                        } else if !is_left_braced && hex_str.len() == 4 {
                            break;
                        } else if next.is_ascii_hexdigit() {
                            hex_str.push(next);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    // parse hex
                    if let Ok(code) = u32::from_str_radix(&hex_str, 16)
                        && ((is_left_braced && is_right_braced) || hex_str.len() == 4)
                        && let Some(decoded) = char::from_u32(code) {
                            result.push(decoded);
                            continue;
                        }

                    // parse failed
                    result.push('\\');
                    result.push('u');
                    if is_left_braced {
                        result.push('{');
                    }
                    result.push_str(&hex_str);
                    if is_right_braced {
                        result.push('}');
                    }
                }
                _ => result.push(c),
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::decode_unicode;

    #[test]
    fn test_decode_unicode() {
        assert_eq!(decode_unicode("Hello \\u4F60\\u597D"), "Hello 你好");
        assert_eq!(decode_unicode("Escaped: \\u{1F602}"), "Escaped: 😂");
        assert_eq!(decode_unicode("Invalid: \\uXYZ"), "Invalid: \\uXYZ");
        assert_eq!(decode_unicode("Mixed: A\\u0041\\u{41}"), "Mixed: AAA");
    }

    #[test]
    fn advanced_test_decode_unicode() {
        let test_cases = vec![
            ("Chinese: \\u4F60\\u597D", "Chinese: 你好"),
            (
                "Japanese: \\u{3053}\\u3093\\u306B\\u{3061}\\u{306F}",
                "Japanese: こんにちは",
            ),
            (
                "Korean: \\uC548\\uB155\\uD558\\uC138\\uC694",
                "Korean: 안녕하세요",
            ),
            (
                "Thai: \\u{0E2A}\\u{0E27}\\u{0E31}\\u{0E2A}\\u{0E14}\\u{0E35}",
                "Thai: สวัสดี",
            ),
            (
                "Vietnamese: \\u{1EA1}\\u{0301}\\u{006E} \\u{0063}\\u{1EC7}",
                "Vietnamese: ạ́n cệ",
            ),
            (
                "Russian: \\u{41F}\\u0440\\u{438}\\u{432}\\u{435}\\u{442}",
                "Russian: Привет",
            ),
            (
                "French: \\u00C7\\u0061 \\u0076\\u0061\\u{003F}",
                "French: Ça va?",
            ),
            (
                "Spanish: \\u00A1\\u0048\\u006F\\u006C\\u0061\\u{0021}",
                "Spanish: ¡Hola!",
            ),
            (
                "German: \\u0048\\u0061\\u006C\\u{006C}\\u{006F}",
                "German: Hallo",
            ),
            (
                "Greek: \\u0393\\u{03B5}\\u{03B9}\\u{03AC} \\u{03C3}\\u{03B1}\\u{03C2}",
                "Greek: Γειά σας",
            ),
            (
                "Arabic: \\u{0645}\\u{0631}\\u{062D}\\u{0628}\\u{0627}",
                "Arabic: مرحبا",
            ),
            ("Persian: \\u0633\\u0644\\u0627\\u0645", "Persian: سلام"),
            ("Urdu: \\u0627\\u0633\\u{0644}\\u0645", "Urdu: اسلم"),
            ("Emoji: \\u{1F600} \\u{1F603} \\u{1F604}", "Emoji: 😀 😃 😄"),
            ("Math: \\u{221E} \\u{2205} \\u{222B}", "Math: ∞ ∅ ∫"),
            ("Currency: \\u{20AC} \\u{00A5} \\u{00A3}", "Currency: € ¥ £"),
            (
                "Partial Escape: \\u4F6 \\u{597",
                "Partial Escape: \\u4F6 \\u{597",
            ),
            (
                "Invalid Sequence: \\uXYZ \\u{INVALID}",
                "Invalid Sequence: \\uXYZ \\u{INVALID}",
            ),
            ("Mixed Format: A\\u0041\\u{41}", "Mixed Format: AAA"),
            ("Empty Sequence: \\u{}", "Empty Sequence: \\u{}"),
        ];
        for (case, cor) in test_cases {
            assert_eq!(decode_unicode(case), cor);
        }
    }
}
