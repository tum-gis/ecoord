use chrono::Utc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimestampParseError {
    #[error("failed to convert to chrono::DateTime<Utc>: {0}")]
    ChronoConversionError(#[from] chrono::ParseError),
}

pub fn parse_timestamp(arg: &str) -> Result<chrono::DateTime<Utc>, TimestampParseError> {
    let chrono_datetime: chrono::DateTime<Utc> =
        chrono::DateTime::parse_from_rfc3339(arg)?.with_timezone(&Utc);
    Ok(chrono_datetime)
}
