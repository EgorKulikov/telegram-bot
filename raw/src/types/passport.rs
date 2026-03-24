use crate::types::*;

/// Describes Telegram Passport data shared with the bot by the user.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

/// Describes documents or other Telegram Passport elements shared with the bot by the user.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub files: Option<Vec<PassportFile>>,
    pub front_side: Option<PassportFile>,
    pub reverse_side: Option<PassportFile>,
    pub selfie: Option<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

/// This object represents a file uploaded to Telegram Passport.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Integer,
    pub file_date: Integer,
}

/// Describes data required for decrypting and authenticating EncryptedPassportElement.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

/// This object represents an error in the Telegram Passport element.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "source")]
pub enum PassportElementError {
    #[serde(rename = "data")]
    DataField {
        #[serde(rename = "type")]
        type_: String,
        field_name: String,
        data_hash: String,
        message: String,
    },
    #[serde(rename = "front_side")]
    FrontSide {
        #[serde(rename = "type")]
        type_: String,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "reverse_side")]
    ReverseSide {
        #[serde(rename = "type")]
        type_: String,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "selfie")]
    Selfie {
        #[serde(rename = "type")]
        type_: String,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "file")]
    File {
        #[serde(rename = "type")]
        type_: String,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "files")]
    Files {
        #[serde(rename = "type")]
        type_: String,
        file_hashes: Vec<String>,
        message: String,
    },
    #[serde(rename = "translation_file")]
    TranslationFile {
        #[serde(rename = "type")]
        type_: String,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "translation_files")]
    TranslationFiles {
        #[serde(rename = "type")]
        type_: String,
        file_hashes: Vec<String>,
        message: String,
    },
    #[serde(rename = "unspecified")]
    Unspecified {
        #[serde(rename = "type")]
        type_: String,
        element_hash: String,
        message: String,
    },
}
