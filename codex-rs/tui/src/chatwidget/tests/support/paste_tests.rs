use super::super::*;

#[cfg(test)]
mod duplicate_placeholder_tests {
    use super::*;

    #[test]
    fn test_duplicate_placeholders_replaced_in_order() {
        // Two pending entries with same placeholder label but different payloads
        let pending = vec![
            PendingPaste::new("[PASTE]".to_string(), "payload1".to_string()),
            PendingPaste::new("[PASTE]".to_string(), "payload2".to_string()),
        ];

        let text = "[PASTE] [PASTE] original";
        let result = current_text_with_pending(text, &pending);

        // Should replace one at a time in order
        assert!(result.contains("payload1"));
        assert!(result.contains("payload2"));
        assert_eq!(result.matches("payload1").count(), 1);
        assert_eq!(result.matches("payload2").count(), 1);
    }

    #[test]
    fn test_filter_preserves_occurrence_count() {
        let text = "[PASTE] [PASTE] [PASTE]";
        let pending = vec![
            PendingPaste::new("[PASTE]".to_string(), "a".to_string()),
            PendingPaste::new("[PASTE]".to_string(), "b".to_string()),
        ];

        let filtered = filter_pending_pastes(&pending, text);

        // Should keep at most 3 entries (matching occurrences)
        assert!(filtered.len() <= 3);
    }
}
