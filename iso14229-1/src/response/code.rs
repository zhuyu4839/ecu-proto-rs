//! response code enum

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Code {
    #[default]
    Positive = 0x00,

    GeneralReject = 0x10,
    ServiceNotSupported = 0x11,
    SubFunctionNotSupported = 0x12,
    IncorrectMessageLengthOrInvalidFormat = 0x13,
    ResponseTooLong = 0x14,

    BusyRepeatRequest = 0x21,
    ConditionsNotCorrect = 0x22,

    RequestSequenceError = 0x24,
    NoResponseFromSubnetComponent = 0x25,
    FailurePreventsExecutionOfRequestedAction = 0x26,

    RequestOutOfRange = 0x31,

    SecurityAccessDenied = 0x33,
    AuthenticationRequired = 0x34,
    InvalidKey = 0x35,
    ExceedNumberOfAttempts = 0x36,
    RequiredTimeDelayNotExpired = 0x37,
    SecureDataTransmissionRequired = 0x38,
    SecureDataTransmissionNotAllowed = 0x39,
    SecureDataVerificationFailed = 0x3A,

    CertificateVerificationFailedInvalidTimePeriod = 0x50,
    CertificateVerificationFailedInvalidSignature = 0x51,
    CertificateVerificationFailedInvalidChainOfTrust = 0x52,
    CertificateVerificationFailedInvalidType = 0x53,
    CertificateVerificationFailedInvalidFormat = 0x54,
    CertificateVerificationFailedInvalidContent = 0x55,
    CertificateVerificationFailedInvalidScope = 0x56,
    CertificateVerificationFailedInvalidCertificate = 0x57,
    OwnershipVerificationFailed = 0x58,
    ChallengeCalculationFailed = 0x59,
    SettingAccessRightsFailed = 0x5A,
    SessionKeyCreationDerivationFailed = 0x5B,
    ConfigurationDataUsageFailed = 0x5C,
    DeAuthenticationFailed = 0x5D,

    UploadDownloadNotAccepted = 0x70,
    TransferDataSuspended = 0x71,
    GeneralProgrammingFailure = 0x72,
    WrongBlockSequenceCounter = 0x73,

    RequestCorrectlyReceivedResponsePending = 0x78,

    SubFunctionNotSupportedInActiveSession = 0x7E,
    ServiceNotSupportedInActiveSession = 0x7F,

    RpmTooHigh = 0x81,
    RpmTooLow = 0x82,
    EngineIsRunning = 0x83,
    EngineIsNotRunning = 0x84,
    EngineRunTimeTooLow = 0x85,
    TemperatureTooHigh = 0x86,
    TemperatureTooLow = 0x87,
    VehicleSpeedTooHigh = 0x88,
    VehicleSpeedTooLow = 0x89,
    ThrottlePedalTooHigh = 0x8A,
    ThrottlePedalTooLow = 0x8B,
    TransmissionRangeNotInNeutral = 0x8C,
    TransmissionRangeNotInGear = 0x8D,
    BrakeSwitchNotClosed = 0x8F,
    ShifterLeverNotInPark = 0x90,
    TorqueConverterClutchLocked = 0x91,
    VoltageTooHigh = 0x92,
    VoltageTooLow = 0x93,
    ResourceTemporarilyNotAvailable = 0x94,
    VehicleManufacturerSpecific(u8), // 0xF0~0xFE
    Reserved(u8),
}

impl From<u8> for Code {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Positive,

            0x10 => Self::GeneralReject,
            0x11 => Self::ServiceNotSupported,
            0x12 => Self::SubFunctionNotSupported,
            0x13 => Self::IncorrectMessageLengthOrInvalidFormat,
            0x14 => Self::ResponseTooLong,

            0x21 => Self::BusyRepeatRequest,

            0x22 => Self::ConditionsNotCorrect,
            0x24 => Self::RequestSequenceError,
            0x25 => Self::NoResponseFromSubnetComponent,
            0x26 => Self::FailurePreventsExecutionOfRequestedAction,

            0x31 => Self::RequestOutOfRange,
            0x33 => Self::SecurityAccessDenied,
            0x34 => Self::AuthenticationRequired,
            0x35 => Self::InvalidKey,
            0x36 => Self::ExceedNumberOfAttempts,
            0x37 => Self::RequiredTimeDelayNotExpired,
            0x38 => Self::SecureDataTransmissionRequired,
            0x39 => Self::SecureDataTransmissionNotAllowed,
            0x3A => Self::SecureDataVerificationFailed,

            0x50 => Self::CertificateVerificationFailedInvalidTimePeriod,
            0x51 => Self::CertificateVerificationFailedInvalidSignature,
            0x52 => Self::CertificateVerificationFailedInvalidChainOfTrust,
            0x53 => Self::CertificateVerificationFailedInvalidType,
            0x54 => Self::CertificateVerificationFailedInvalidFormat,
            0x55 => Self::CertificateVerificationFailedInvalidContent,
            0x56 => Self::CertificateVerificationFailedInvalidScope,
            0x57 => Self::CertificateVerificationFailedInvalidCertificate,
            0x58 => Self::OwnershipVerificationFailed,
            0x59 => Self::ChallengeCalculationFailed,
            0x5A => Self::SettingAccessRightsFailed,
            0x5B => Self::SessionKeyCreationDerivationFailed,
            0x5C => Self::ConfigurationDataUsageFailed,
            0x5D => Self::DeAuthenticationFailed,

            0x70 => Self::UploadDownloadNotAccepted,
            0x71 => Self::TransferDataSuspended,
            0x72 => Self::GeneralProgrammingFailure,
            0x73 => Self::WrongBlockSequenceCounter,

            0x78 => Self::RequestCorrectlyReceivedResponsePending,

            0x7E => Self::SubFunctionNotSupportedInActiveSession,
            0x7F => Self::ServiceNotSupportedInActiveSession,

            0x81 => Self::RpmTooHigh,
            0x82 => Self::RpmTooLow,
            0x83 => Self::EngineIsRunning,
            0x84 => Self::EngineIsNotRunning,
            0x85 => Self::EngineRunTimeTooLow,
            0x86 => Self::TemperatureTooHigh,
            0x87 => Self::TemperatureTooLow,
            0x88 => Self::VehicleSpeedTooHigh,
            0x89 => Self::VehicleSpeedTooLow,
            0x8A => Self::ThrottlePedalTooHigh,
            0x8B => Self::ThrottlePedalTooLow,
            0x8C => Self::TransmissionRangeNotInNeutral,
            0x8D => Self::TransmissionRangeNotInGear,
            0x8F => Self::BrakeSwitchNotClosed,
            0x90 => Self::ShifterLeverNotInPark,
            0x91 => Self::TorqueConverterClutchLocked,
            0x92 => Self::VoltageTooHigh,
            0x93 => Self::VoltageTooLow,
            0x94 => Self::ResourceTemporarilyNotAvailable,
            0xF0..=0xFE => Self::VehicleManufacturerSpecific(value),
            _ => Self::Reserved(value),
        }
    }
}

impl Into<u8> for Code {
    fn into(self) -> u8 {
        match self {
            Self::Positive => 0x00,

            Self::GeneralReject => 0x10,
            Self::ServiceNotSupported => 0x11,
            Self::SubFunctionNotSupported => 0x12,
            Self::IncorrectMessageLengthOrInvalidFormat => 0x13,
            Self::ResponseTooLong => 0x14,

            Self::BusyRepeatRequest => 0x21,
            Self::ConditionsNotCorrect => 0x22,

            Self::RequestSequenceError => 0x24,
            Self::NoResponseFromSubnetComponent => 0x25,
            Self::FailurePreventsExecutionOfRequestedAction => 0x26,

            Self::RequestOutOfRange => 0x31,

            Self::SecurityAccessDenied => 0x33,
            Self::AuthenticationRequired => 0x34,
            Self::InvalidKey => 0x35,
            Self::ExceedNumberOfAttempts => 0x36,
            Self::RequiredTimeDelayNotExpired => 0x37,
            Self::SecureDataTransmissionRequired => 0x38,
            Self::SecureDataTransmissionNotAllowed => 0x39,
            Self::SecureDataVerificationFailed => 0x3A,

            Self::CertificateVerificationFailedInvalidTimePeriod => 0x50,
            Self::CertificateVerificationFailedInvalidSignature => 0x51,
            Self::CertificateVerificationFailedInvalidChainOfTrust => 0x52,
            Self::CertificateVerificationFailedInvalidType => 0x53,
            Self::CertificateVerificationFailedInvalidFormat => 0x54,
            Self::CertificateVerificationFailedInvalidContent => 0x55,
            Self::CertificateVerificationFailedInvalidScope => 0x56,
            Self::CertificateVerificationFailedInvalidCertificate => 0x57,
            Self::OwnershipVerificationFailed => 0x58,
            Self::ChallengeCalculationFailed => 0x59,
            Self::SettingAccessRightsFailed => 0x5A,
            Self::SessionKeyCreationDerivationFailed => 0x5B,
            Self::ConfigurationDataUsageFailed => 0x5C,
            Self::DeAuthenticationFailed => 0x5D,

            Self::UploadDownloadNotAccepted => 0x70,
            Self::TransferDataSuspended => 0x71,
            Self::GeneralProgrammingFailure => 0x72,
            Self::WrongBlockSequenceCounter => 0x73,

            Self::RequestCorrectlyReceivedResponsePending => 0x78,

            Self::SubFunctionNotSupportedInActiveSession => 0x7E,
            Self::ServiceNotSupportedInActiveSession => 0x7F,

            Self::RpmTooHigh => 0x81,
            Self::RpmTooLow => 0x82,
            Self::EngineIsRunning => 0x83,
            Self::EngineIsNotRunning => 0x84,
            Self::EngineRunTimeTooLow => 0x85,
            Self::TemperatureTooHigh => 0x86,
            Self::TemperatureTooLow => 0x87,
            Self::VehicleSpeedTooHigh => 0x88,
            Self::VehicleSpeedTooLow => 0x89,
            Self::ThrottlePedalTooHigh => 0x8A,
            Self::ThrottlePedalTooLow => 0x8B,
            Self::TransmissionRangeNotInNeutral => 0x8C,
            Self::TransmissionRangeNotInGear => 0x8D,
            Self::BrakeSwitchNotClosed => 0x8F,
            Self::ShifterLeverNotInPark => 0x90,
            Self::TorqueConverterClutchLocked => 0x91,
            Self::VoltageTooHigh => 0x92,
            Self::VoltageTooLow => 0x93,
            Self::ResourceTemporarilyNotAvailable => 0x94,
            Self::VehicleManufacturerSpecific(v) => v,
            Self::Reserved(v) => v,
        }
    }
}
