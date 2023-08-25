use std::{string::FromUtf8Error, fmt::{Display, Debug}};




pub enum NbtError{
    Utf8Error,
    NBTFormatError,
    NBTCastError,
}

impl Debug for NbtError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for NbtError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            NbtError::Utf8Error => "Invalid utf-8 sequence of",
            NbtError::NBTFormatError => "NBT size does not match tag format",
            NbtError::NBTCastError => "Error occured on NBT value"
        })
    }
}

impl From<FromUtf8Error> for NbtError{
    fn from(_value: FromUtf8Error) -> Self {
        NbtError::Utf8Error
    }
}