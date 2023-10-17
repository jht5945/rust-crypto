use base64::{DecodeError, Engine};
use base64::engine::general_purpose::STANDARD;

pub trait FromBase64 {
    fn from_base64(&self) -> Result<Vec<u8>, DecodeError>;
}

pub trait ToBase64 {
    fn to_base64(&self) -> String;
}


impl FromBase64 for str {
    fn from_base64(&self) -> Result<Vec<u8>, DecodeError> {
        STANDARD.decode(self)
    }
}


impl ToBase64 for [u8] {
    fn to_base64(&self) -> String {
        STANDARD.encode(self)
    }
}

