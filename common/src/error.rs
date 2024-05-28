use crate::tee::TEEType;
use std::path::PathBuf;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("FileError '{path}': {message}")]
    FileError { path: PathBuf, message: String },
    #[error("NetworkError '{address}': {message}")]
    NetworkError { address: String, message: String },
    #[error("SerDeError: {message}")]
    SerDeError { message: String },
    #[error("VerificationError: type {teetype}, reason {error}")]
    VerificationError {
        teetype: TEEType,
        error: VerificationError,
    },
    #[error("UnkownTEETypeERROR: {message}")]
    UnkownTEETypeERROR { message: String },
    #[error("AttestationError: type {teetype}, reason {error}")]
    AttestationError {
        teetype: TEEType,
        error: AttestationError,
    },
    #[error("RegistrationError: {message}")]
    RegistrationError { message: String },
    #[error("RegistrationError: {message}")]
    ServerError { message: String },
    #[error("InferenceError: {message}")]
    InferenceError { message: String },
    #[error("DownloadingModelError: {message}")]
    DownloadingModelError { model: String, message: String },
}

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("KidNotFoundError {kid}")]
    KidNotFoundError { kid: String },
    #[error("SigAlgMismatchError {algorithm}")]
    SigAlgMismatchError { algorithm: String },
    #[error("ValidateTokenError {message}")]
    ValidateTokenError { message: String },
    #[error("GoldenValueMismatchError {value} expect: {expect} get: {get}")]
    GoldenValueMismatchError {
        value: String,
        expect: String,
        get: String,
    },
}

#[derive(Error, Debug)]
pub enum AttestationError {
    #[error("ReportError: {message}")]
    ReportError { message: String },
}
