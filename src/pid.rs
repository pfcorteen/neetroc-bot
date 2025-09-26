use std::fmt;
use std::sync::OnceLock;
use regex::Regex;
use crate::pieces::PieceType;
use crate::pieces::PieceTypeData;
use crate::pieces::Side;
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


    pub fn get_square(&self) -> &str {
        let sq = &self.0[0..2];
        sq
    }

    pub fn get_side(self) -> Side {
        let piece_char = &self.0.chars().nth(2).unwrap();
        if piece_char.is_uppercase() {
            Side::White
        } else {
            Side::Black
        }
    }

    pub fn get_piece_data(self) -> &'static PieceTypeData {
        let piece_type_char = self.0.chars().nth(2).unwrap();
        let piece_type_ref = PieceType::get_piece_type(piece_type_char);
        if let Some(piece_type) = piece_type_ref {
            piece_type.get_data()
        } else {
            panic!("Invalid piece type")
        }
    }
}

// Implement Display for easy printing
impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}