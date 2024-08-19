use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: i64,
    pub file_date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPassportElement {
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassportElementErrorDataType {
    #[serde(rename = "personal_details")]
    PersonalDetails,
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "driver_license")]
    DriverLicense,
    #[serde(rename = "identity_card")]
    IdentityCard,
    #[serde(rename = "internal_passport")]
    InternalPassport,
    #[serde(rename = "address")]
    Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassportElementErrorFrontSideType {
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "driver_license")]
    DriverLicense,
    #[serde(rename = "identity_card")]
    IdentityCard,
    #[serde(rename = "internal_passport")]
    InternalPassport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassportElementErrorReverseSideType {
    #[serde(rename = "driver_license")]
    DriverLicense,
    #[serde(rename = "identity_card")]
    IdentityCard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassportElementErrorSelfieType {
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "driver_license")]
    DriverLicense,
    #[serde(rename = "identity_card")]
    IdentityCard,
    #[serde(rename = "internal_passport")]
    InternalPassport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassportElementErrorFileType {
    #[serde(rename = "utility_bill")]
    UtilityBill,
    #[serde(rename = "bank_statement")]
    BankStatement,
    #[serde(rename = "rental_agreement")]
    RentalAgreement,
    #[serde(rename = "passport_registration")]
    PassportRegistration,
    #[serde(rename = "temporary_registration")]
    TemporaryRegistration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassportElementErrorTranslationFileType {
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "driver_license")]
    DriverLicense,
    #[serde(rename = "identity_card")]
    IdentityCard,
    #[serde(rename = "internal_passport")]
    InternalPassport,
    #[serde(rename = "utility_bill")]
    UtilityBill,
    #[serde(rename = "bank_statement")]
    BankStatement,
    #[serde(rename = "rental_agreement")]
    RentalAgreement,
    #[serde(rename = "passport_registration")]
    PassportRegistration,
    #[serde(rename = "temporary_registration")]
    TemporaryRegistration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "source")]
pub enum PassportElementError {
    #[serde(rename = "data")]
    Data {
        r#type: PassportElementErrorDataType,
        field_name: String,
        data_hash: String,
        message: String,
    },
    #[serde(rename = "front_side")]
    FrontSile {
        r#type: PassportElementErrorFrontSideType,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "reverse_side")]
    ReverseSide {
        r#type: PassportElementErrorReverseSideType,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "selfie")]
    Selfie {
        r#type: PassportElementErrorSelfieType,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "file")]
    File {
        r#type: PassportElementErrorFileType,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "files")]
    Files {
        r#type: PassportElementErrorFileType,
        file_hashes: Vec<String>,
        message: String,
    },
    #[serde(rename = "translation_file")]
    TranslationFile {
        r#type: PassportElementErrorTranslationFileType,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "translation_files")]
    TranslationFiles {
        r#type: PassportElementErrorTranslationFileType,
        file_hashes: Vec<String>,
        message: String,
    },
    #[serde(rename = "unspecified")]
    Unspecified {
        r#type: String,
        element_hash: String,
        message: String,
    },
}
