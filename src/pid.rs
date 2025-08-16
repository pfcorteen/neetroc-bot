use std::fmt;
use std::sync::OnceLock;
use regex::Regex;

// Define the new type.
#[derive(Debug, Clone, PartialEq, Eq)]
// pub(crate) struct Pid(String);
pub struct Pid(String);

// Create a static, lazily-compiled Regex object.
// This ensures the regex is compiled only once for efficiency.
fn code_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"^[a-h][1-8][PpNnBbRrQqKk]$").unwrap()
    })
}

impl Pid {
    // 3. Update the constructor to use the regex for validation.
    pub fn new(s: &str) -> Result<Self, String> {
        if !code_regex().is_match(s) {
            return Err(format!("'{}' is not a valid 3-letter uppercase code.", s));
        }

        // Validate length is exactly 3
        if s.len() != 3 {
            return Err(format!("'{}' must be exactly 3 characters long.", s));
        }

        Ok(Self(s.to_string()))
    }

    // Get the underlying string reference
    pub fn as_str(&self) -> &str {
        &self.0
    }

    // Get the underlying string
    pub fn into_string(self) -> String {
        self.0
    }
}

// Implement Display for easy printing
impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}