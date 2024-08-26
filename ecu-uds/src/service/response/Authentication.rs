/// Service 29

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::error::Error;
use crate::utils;
use crate::service::{AuthenticationTask, NotNullableData, NullableData, parse_not_nullable, parse_nullable, AlgorithmIndicator, parse_algo_indicator, ALGORITHM_INDICATOR_LENGTH, ResponseData, Configuration};
use crate::service::response::Code;

lazy_static!(
    pub static ref AUTH_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestSequenceError,
        Code::CertificateVerificationFailedInvalidTimePeriod,
        Code::CertificateVerificationFailedInvalidSignature,
        Code::CertificateVerificationFailedInvalidChainOfTrust,
        Code::CertificateVerificationFailedInvalidType,
        Code::CertificateVerificationFailedInvalidFormat,
        Code::CertificateVerificationFailedInvalidContent,
        Code::CertificateVerificationFailedInvalidScope,
        Code::CertificateVerificationFailedInvalidCertificate,
        Code::OwnershipVerificationFailed,
        Code::ChallengeCalculationFailed,
        Code::SettingAccessRightsFailed,
        Code::SessionKeyCreationDerivationFailed,
        Code::ConfigurationDataUsageFailed,
        Code::DeAuthenticationFailed,
    ]);
);

/// Table B.5 — authenticationReturnParameter definitions
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthReturnValue {
    RequestAccepted = 0x00,
    GeneralReject = 0x01,
    AuthenticationConfigurationAPCE = 0x02,
    AuthenticationConfigurationACRWithAsymmetricCryptography = 0x03,
    AuthenticationConfigurationACRWithSymmetricCryptography = 0x04,
    DeAuthenticationSuccessful = 0x10,
    CertificateVerifiedOrOwnershipVerificationNecessary = 0x11,
    OwnershipVerifiedOrAuthenticationComplete = 0x12,
    CertificateVerified = 0x13,
    VehicleManufacturerSpecific(u8),
    SystemSupplierSpecific(u8),
}

impl TryFrom<u8> for AuthReturnValue {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::RequestAccepted),
            0x01 => Ok(Self::GeneralReject),
            0x02 => Ok(Self::AuthenticationConfigurationAPCE),
            0x03 => Ok(Self::AuthenticationConfigurationACRWithAsymmetricCryptography),
            0x04 => Ok(Self::AuthenticationConfigurationACRWithSymmetricCryptography),
            0x10 => Ok(Self::DeAuthenticationSuccessful),
            0x11 => Ok(Self::CertificateVerifiedOrOwnershipVerificationNecessary),
            0x12 => Ok(Self::OwnershipVerifiedOrAuthenticationComplete),
            0x13 => Ok(Self::CertificateVerified),
            0xA0..=0xCF => Ok(Self::VehicleManufacturerSpecific(value)),
            0xD0..=0xFE => Ok(Self::SystemSupplierSpecific(value)),
            v => Err(Error::InvalidParam(utils::err_msg(v))),
        }
    }
}

impl Into<u8> for AuthReturnValue {
    #[inline]
    fn into(self) -> u8 {
        match self {
            AuthReturnValue::RequestAccepted => 0x00,
            AuthReturnValue::GeneralReject => 0x01,
            AuthReturnValue::AuthenticationConfigurationAPCE => 0x02,
            AuthReturnValue::AuthenticationConfigurationACRWithAsymmetricCryptography => 0x03,
            AuthReturnValue::AuthenticationConfigurationACRWithSymmetricCryptography => 0x04,
            AuthReturnValue::DeAuthenticationSuccessful => 0x11,
            AuthReturnValue::CertificateVerifiedOrOwnershipVerificationNecessary => 0x11,
            AuthReturnValue::OwnershipVerifiedOrAuthenticationComplete => 0x12,
            AuthReturnValue::CertificateVerified => 0x13,
            AuthReturnValue::VehicleManufacturerSpecific(v) |
            AuthReturnValue::SystemSupplierSpecific(v) => v,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AuthData {
    DeAuthenticate(AuthReturnValue),
    VerifyCertificateUnidirectional {
        value: AuthReturnValue,
        challenge: NotNullableData,
        ephemeral_public_key: NullableData,
    },
    VerifyCertificateBidirectional {
        value: AuthReturnValue,
        challenge: NotNullableData,
        certificate: NotNullableData,
        proof_of_ownership: NotNullableData,
        ephemeral_public_key: NullableData,
    },
    ProofOfOwnership {
        value: AuthReturnValue,
        session_keyinfo: NullableData,
    },
    TransmitCertificate(AuthReturnValue),
    RequestChallengeForAuthentication {
        value: AuthReturnValue,
        algo_indicator: AlgorithmIndicator,
        challenge: NotNullableData,
        additional: NullableData,
    },
    VerifyProofOfOwnershipUnidirectional {
        value: AuthReturnValue,
        algo_indicator: AlgorithmIndicator,
        session_keyinfo: NullableData,
    },
    VerifyProofOfOwnershipBidirectional {
        value: AuthReturnValue,
        algo_indicator: AlgorithmIndicator,
        proof_of_ownership: NotNullableData,
        session_keyinfo: NullableData,
    },
    AuthenticationConfiguration(AuthReturnValue),
}

impl ResponseData for AuthData {
    type SubFunc = AuthenticationTask;
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 1, false)?;
        let mut offset = 0;
        let value = AuthReturnValue::try_from(data[offset])?;

        match sub_func {
            Some(v) => match v {
                AuthenticationTask::DeAuthenticate => Ok(Self::DeAuthenticate(value)),
                AuthenticationTask::VerifyCertificateUnidirectional => {
                    let challenge = parse_not_nullable(data, data_len, &mut offset)?;
                    let ephemeral_public_key = parse_nullable(data, data_len, &mut offset)?;

                    Ok(Self::VerifyCertificateUnidirectional {
                        value,
                        challenge,
                        ephemeral_public_key
                    })
                },
                AuthenticationTask::VerifyCertificateBidirectional => {
                    let challenge = parse_not_nullable(data, data_len, &mut offset)?;
                    let certificate = parse_not_nullable(data, data_len, &mut offset)?;
                    let proof_of_ownership = parse_not_nullable(data, data_len, &mut offset)?;
                    let ephemeral_public_key = parse_nullable(data, data_len, &mut offset)?;

                    Ok(Self::VerifyCertificateBidirectional {
                        value,
                        challenge,
                        certificate,
                        proof_of_ownership,
                        ephemeral_public_key
                    })
                },
                AuthenticationTask::ProofOfOwnership => {
                    let session_keyinfo = parse_nullable(data, data_len, &mut offset)?;

                    Ok(Self::ProofOfOwnership {
                        value,
                        session_keyinfo,
                    })
                },
                AuthenticationTask::TransmitCertificate => Ok(Self::TransmitCertificate(value)),
                AuthenticationTask::RequestChallengeForAuthentication => {
                    if data_len < offset + ALGORITHM_INDICATOR_LENGTH {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let algo_indicator = parse_algo_indicator(data, &mut offset);
                    let challenge = parse_not_nullable(data, data_len, &mut offset)?;
                    let additional = parse_nullable(data, data_len, &mut offset)?;

                    Ok(Self::RequestChallengeForAuthentication {
                        value,
                        algo_indicator,
                        challenge,
                        additional
                    })
                },
                AuthenticationTask::VerifyProofOfOwnershipUnidirectional => {
                    if data_len < offset + ALGORITHM_INDICATOR_LENGTH {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let algo_indicator = parse_algo_indicator(data, &mut offset);
                    let session_keyinfo = parse_nullable(data, data_len, &mut offset)
                        .map_err(|_| Error::InvalidData(utils::hex_slice_to_string(data)))?;

                    Ok(Self::VerifyProofOfOwnershipUnidirectional {
                        value,
                        algo_indicator,
                        session_keyinfo,
                    })
                },
                AuthenticationTask::VerifyProofOfOwnershipBidirectional => {
                    if data_len < offset + ALGORITHM_INDICATOR_LENGTH {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let algo_indicator = parse_algo_indicator(data, &mut offset);
                    let proof_of_ownership = parse_not_nullable(data, data_len, &mut offset)?;
                    let session_keyinfo = parse_nullable(data, data_len, &mut offset)?;

                    Ok(Self::VerifyProofOfOwnershipBidirectional {
                        value,
                        algo_indicator,
                        proof_of_ownership,
                        session_keyinfo,
                    })
                },
                AuthenticationTask::AuthenticationConfiguration =>
                    Ok(Self::AuthenticationConfiguration(value)),
            },
            None => panic!("Sub-function required"),
        }
    }
}

impl Into<Vec<u8>> for AuthData {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();

        match self {
            Self::DeAuthenticate(v) => result.push(v.into()),
            Self::VerifyCertificateUnidirectional {
                value,
                challenge,
                ephemeral_public_key,
            } => {
                result.push(value.into());
                result.append(&mut challenge.into());
                result.append(&mut ephemeral_public_key.into());
            },
            Self::VerifyCertificateBidirectional {
                value,
                challenge,
                certificate,
                proof_of_ownership,
                ephemeral_public_key,
            } => {
                result.push(value.into());
                result.append(&mut challenge.into());
                result.append(&mut ephemeral_public_key.into());
                result.append(&mut certificate.into());
                result.append(&mut proof_of_ownership.into());
            },
            Self::ProofOfOwnership {
                value,
                session_keyinfo,
            } => {
                result.push(value.into());
                result.append(&mut session_keyinfo.into());
            },
            Self::TransmitCertificate(v) => result.push(v.into()),
            Self::RequestChallengeForAuthentication {
                value,
                algo_indicator,
                challenge,
                additional,
            } => {
                result.push(value.into());
                result.append(&mut algo_indicator.into());
                result.append(&mut challenge.into());
                result.append(&mut additional.into());
            },
            Self::VerifyProofOfOwnershipUnidirectional {
                value,
                algo_indicator,
                session_keyinfo,
            } => {
                result.push(value.into());
                result.append(&mut algo_indicator.into());
                result.append(&mut session_keyinfo.into());
            },
            Self::VerifyProofOfOwnershipBidirectional {
                value,
                algo_indicator,
                proof_of_ownership,
                session_keyinfo,
            } => {
                result.push(value.into());
                result.append(&mut algo_indicator.into());
                result.append(&mut proof_of_ownership.into());
                result.append(&mut session_keyinfo.into());
            },
            Self::AuthenticationConfiguration(v) => result.push(v.into()),
        }

        result
    }
}
