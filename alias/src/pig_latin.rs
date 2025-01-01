/// Поросячья латынь.
///
/// Если слово начинается с согласной, то она перемещается в конец, затем в конец слова добавляется "ay".
/// Если слово начинается с гласной, то в конец слова добавляется "way".

pub fn eng_to_pig(eng: String) -> String {
    let vowels = ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];

    eng.split_whitespace()
        .map(|word| {
            let first_char = word.chars().next();
            let mut punctuation = String::new();
            let word = word.trim_end_matches(|c: char| {
                if c.is_ascii_punctuation() {
                    punctuation.push(c);
                    true
                } else {
                    false
                }
            });

            if let Some(mut c) = first_char {
                if vowels.contains(&c.to_string().as_str()) {
                    format!(
                        "{}way{}",
                        word,
                        punctuation.chars().rev().collect::<String>()
                    )
                } else {
                    let mut chars = word.chars();
                    chars.next();
                    let mut rest: String = chars.collect();

                    if c.is_uppercase() {
                        c = c.to_ascii_lowercase();
                        rest = capitalize_first_letter(&rest);
                    }

                    format!(
                        "{}{}ay{}",
                        rest,
                        c,
                        punctuation.chars().rev().collect::<String>()
                    )
                }
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn capitalize_first_letter(input: &str) -> String {
    let mut chars = input.chars();
    if let Some(first_char) = chars.next() {
        format!("{}{}", first_char.to_uppercase(), chars.collect::<String>())
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eng_to_pig_1() {
        let result = eng_to_pig("Hello, world! Awesome.".to_string());
        assert_eq!(result, "Ellohay, orldway! Awesomeway.".to_string());
    }
    #[test]
    fn test_eng_to_pig_2() {
        let result = eng_to_pig(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        );
        assert_eq!(result, "Oremlay Ipsumway isway implysay ummyday exttay ofway hetay rintingpay andway ypesettingtay industryway.".to_string());
    }

    #[test]
    fn test_capitalize_first_letter_1() {
        let result = capitalize_first_letter("0");
        assert_eq!(result, "0");
    }
    #[test]
    fn test_capitalize_first_letter_2() {
        let result = capitalize_first_letter("w");
        assert_eq!(result, "W");
    }

    #[test]
    fn test_capitalize_first_letter_3() {
        let result = capitalize_first_letter("W");
        assert_eq!(result, "W");
    }
}
