use regex::Regex;

/// Detects whether a log line is a continuation of a multi-line stacktrace
pub struct StacktraceMerger {
    rust_frame_re: Regex,
    java_frame_re: Regex,
    python_frame_re: Regex,
}

impl StacktraceMerger {
    pub fn new() -> Self {
        Self {
            rust_frame_re: Regex::new(r"^\s*\d+:\s+0x[0-9a-fA-F]").unwrap(),
            java_frame_re: Regex::new(r"^\s+at\s+[\w\.$]+\(").unwrap(),
            python_frame_re: Regex::new(r#"^\s+File "[^"]+", line \d+"#).unwrap(),
        }
    }

    /// Returns true if `line` is a continuation of a previous log entry (stacktrace line)
    pub fn is_continuation(&self, line: &str) -> bool {
        if line.is_empty() {
            return false;
        }

        // Indented line (tab or 2+ spaces at start)
        if line.starts_with('\t') || line.starts_with("  ") {
            return true;
        }

        let trimmed = line.trim_start();

        // Common stacktrace keywords
        if trimmed.starts_with("at ")
            || trimmed.starts_with("Caused by:")
            || trimmed.starts_with("... ")
            || trimmed.starts_with("Traceback")
            || trimmed.starts_with("During handling")
        {
            return true;
        }

        // Language-specific frame patterns
        if self.rust_frame_re.is_match(line)
            || self.java_frame_re.is_match(line)
            || self.python_frame_re.is_match(line)
        {
            return true;
        }

        false
    }
}

impl Default for StacktraceMerger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_java_stacktrace() {
        let merger = StacktraceMerger::new();
        assert!(merger.is_continuation("\tat com.example.Foo.bar(Foo.java:42)"));
        assert!(merger.is_continuation("Caused by: java.lang.NullPointerException"));
        assert!(merger.is_continuation("... 5 more"));
    }

    #[test]
    fn test_python_stacktrace() {
        let merger = StacktraceMerger::new();
        assert!(merger.is_continuation("  File \"app.py\", line 10, in main"));
        assert!(merger.is_continuation("Traceback (most recent call last):"));
    }

    #[test]
    fn test_rust_backtrace() {
        let merger = StacktraceMerger::new();
        assert!(merger.is_continuation("   0: 0x10012345 - main"));
    }

    #[test]
    fn test_normal_line_not_continuation() {
        let merger = StacktraceMerger::new();
        assert!(!merger.is_continuation("2024-01-01 INFO Request received"));
        assert!(!merger.is_continuation("[ERROR] Something went wrong"));
    }
}
