use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter)]
pub enum Format {
    #[default]
    Json,
    Csv,
}

impl Format {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(Self::Json),
            "csv" => Some(Self::Csv),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Csv => "csv",
        }
    }
}
