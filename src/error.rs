
use thiserror::Error;
use tlv::prelude::TlvError;

#[derive(Error, Debug)]
pub enum NasParsingError {
    #[error("Invalid NAS PDU: length is less than 2")]
    InvalidNasPdu,
    #[error("Unkown NAS PDU: Unknown value in message type")]
    UnknownNasPdu,
    #[error("TLV Error: Error while parsing TLV")]
    TlvError(#[from] TlvError),
}