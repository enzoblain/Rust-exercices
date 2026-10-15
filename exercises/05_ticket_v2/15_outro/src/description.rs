use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct TicketDescription(String);

#[derive(Debug)]
pub enum TicketDescriptionError {
    Empty,
    TooLong,
}

impl fmt::Display for TicketDescriptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicketDescriptionError::Empty => {
                write!(f, "The description cannot be empty")
            }
            TicketDescriptionError::TooLong => {
                write!(f, "The description cannot be longer than 500 bytes")
            }
        }
    }
}

impl Error for TicketDescriptionError {}

impl TryFrom<String> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(TicketDescriptionError::Empty);
        }

        if value.len() > 500 {
            return Err(TicketDescriptionError::TooLong);
        }

        Ok(TicketDescription(value))
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TicketDescription::try_from(value.to_string())
    }
}
