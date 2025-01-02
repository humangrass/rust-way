use std::collections::HashMap;

pub fn etaoin(text: String) -> HashMap<String, u64> {
    let mut etaoin = HashMap::new();
    for ch in text.chars() {
        *etaoin.entry(ch.to_string()).or_insert(0) += 1;
    }

    etaoin
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let result = etaoin("".to_string());
        assert_eq!(result, HashMap::new());
    }

    #[test]
    fn test_single_character() {
        let mut expected = HashMap::new();
        expected.insert("a".to_string(), 1);
        let result = etaoin("a".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_repeated_characters() {
        let mut expected = HashMap::new();
        expected.insert("a".to_string(), 3);
        expected.insert("b".to_string(), 2);
        expected.insert("c".to_string(), 1);
        let result = etaoin("aaabbc".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixed_characters_with_spaces() {
        let mut expected = HashMap::new();
        expected.insert("a".to_string(), 1);
        expected.insert("b".to_string(), 1);
        expected.insert(" ".to_string(), 1);
        let result = etaoin("a b".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_special_characters() {
        let mut expected = HashMap::new();
        expected.insert("!".to_string(), 3);
        expected.insert("@".to_string(), 2);
        expected.insert("#".to_string(), 1);
        let result = etaoin("!!!@@#".to_string());
        assert_eq!(result, expected);
    }
}
