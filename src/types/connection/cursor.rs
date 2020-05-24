use crate::ID;
use byteorder::{ReadBytesExt, BE};
use std::convert::Infallible;
use std::fmt::Display;

/// Cursor type
///
/// A custom scalar that serializes as a string.
/// https://relay.dev/graphql/connections.htm#sec-Cursor
pub trait CursorType: Sized {
    type DecodeError: Display;

    fn decode_cursor(s: &str) -> Result<Self, Self::DecodeError>;

    fn encode_cursor(self) -> String;
}

impl CursorType for usize {
    type DecodeError = anyhow::Error;

    fn decode_cursor(s: &str) -> Result<Self, Self::DecodeError> {
        let data = base64::decode(s)?;
        Ok(data.as_slice().read_u32::<BE>()? as usize)
    }

    fn encode_cursor(self) -> String {
        base64::encode((self as u32).to_be_bytes())
    }
}

impl CursorType for String {
    type DecodeError = Infallible;

    fn decode_cursor(s: &str) -> Result<Self, Self::DecodeError> {
        Ok(s.to_string())
    }

    fn encode_cursor(self) -> String {
        self.clone()
    }
}

impl CursorType for ID {
    type DecodeError = Infallible;

    fn decode_cursor(s: &str) -> Result<Self, Self::DecodeError> {
        Ok(s.to_string().into())
    }

    fn encode_cursor(self) -> String {
        self.to_string()
    }
}
