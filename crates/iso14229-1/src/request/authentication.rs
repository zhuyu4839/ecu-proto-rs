//! request of Service 29


use crate::{AlgorithmIndicator, AuthenticationTask, Configuration, Error, NotNullableData, NullableData, parse_algo_indicator, parse_not_nullable, parse_nullable, RequestData, Service, utils};
use crate::request::{Request, SubFunction};

#[derive(Debug, Clone)]
pub enum Authentication {
    DeAuthenticate,                     // 0x00
    VerifyCertificateUnidirectional {   // 0x01
        config: u8,
        certificate: NotNullableData,
        challenge: NullableData,
    },
    VerifyCertificateBidirectional {    // 0x02
        config: u8,
        certificate: NotNullableData,
        challenge: NotNullableData,
    },
    ProofOfOwnership {                  // 0x03
        proof_of_ownership: NotNullableData,
        ephemeral_public_key: NullableData,
    },
    TransmitCertificate {               // 0x04
        cert_evaluation_id: u16,
        certificate: NotNullableData,
    },
    RequestChallengeForAuthentication { // 0x05
        config: u8,
        algo_indicator: AlgorithmIndicator,   // Algorithm OIDs can be looked up in the OID repository http://oid-info.com.
    },
    VerifyProofOfOwnershipUnidirectional { // 0x06
        algo_indicator: AlgorithmIndicator,
        proof_of_ownership: NotNullableData,
        challenge: NullableData,
        additional: NullableData,
    },
    VerifyProofOfOwnershipBidirectional {   // 0x07
        algo_indicator: AlgorithmIndicator,
        proof_of_ownership: NotNullableData,
        challenge: NotNullableData,
        additional: NullableData,
    },
    AuthenticationConfiguration,    // 0x08
}

impl RequestData for Authentication {
    type SubFunc = AuthenticationTask;
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        match sub_func {
            Some(v) => {
                let data_len = data.len();
                let mut offset = 0;
                match v {
                    AuthenticationTask::DeAuthenticate =>
                        Ok(Self::DeAuthenticate),
                    AuthenticationTask::VerifyCertificateUnidirectional => {
                        utils::data_length_check(data_len, 5, false)?;

                        let config = data[offset];
                        offset += 1;
                        let certificate = parse_not_nullable(data, data_len, &mut offset)?;
                        let challenge = parse_nullable(data, data_len, &mut offset)?;

                        Ok(Self::VerifyCertificateUnidirectional {
                            config,
                            certificate,
                            challenge,
                        })
                    },
                    AuthenticationTask::VerifyCertificateBidirectional => {
                        utils::data_length_check(data_len, 7, false)?;

                        let config = data[offset];
                        offset += 1;
                        let certificate = parse_not_nullable(data, data_len, &mut offset)?;
                        let challenge = parse_not_nullable(data, data_len, &mut offset)?;

                        Ok(Self::VerifyCertificateBidirectional {
                            config,
                            certificate,
                            challenge,
                        })
                    },
                    AuthenticationTask::ProofOfOwnership => {
                        let proof_of_ownership = parse_not_nullable(data, data_len, &mut offset)?;
                        let ephemeral_public_key = parse_nullable(data, data_len, &mut offset)?;

                        Ok(Self::ProofOfOwnership {
                            proof_of_ownership,
                            ephemeral_public_key,
                        })
                    },
                    AuthenticationTask::TransmitCertificate => {
                        utils::data_length_check(data_len, 5, false)?;

                        let cert_evaluation_id = u16::from_be_bytes([data[offset], data[offset + 1]]);
                        offset += 2;
                        let certificate = parse_not_nullable(data, data_len, &mut offset)?;

                        Ok(Self::TransmitCertificate {
                            cert_evaluation_id,
                            certificate,
                        })
                    },
                    AuthenticationTask::RequestChallengeForAuthentication => {
                        utils::data_length_check(data_len, 17, false)?;

                        let config = data[offset];
                        offset += 1;
                        let algo_indicator = parse_algo_indicator(data, &mut offset);

                        Ok(Self::RequestChallengeForAuthentication {
                            config,
                            algo_indicator,
                        })
                    },
                    AuthenticationTask::VerifyProofOfOwnershipUnidirectional => {
                        utils::data_length_check(data_len, 19, false)?;

                        let algo_indicator = parse_algo_indicator(data, &mut offset);
                        let proof_of_ownership = parse_not_nullable(data, data_len, &mut offset)?;
                        let challenge = parse_nullable(data, data_len, &mut offset)?;
                        let additional = parse_nullable(data, data_len, &mut offset)?;


                        Ok(Self::VerifyProofOfOwnershipUnidirectional {
                            algo_indicator,
                            proof_of_ownership,
                            challenge,
                            additional,
                        })
                    },
                    AuthenticationTask::VerifyProofOfOwnershipBidirectional => {
                        utils::data_length_check(data_len, 24, false)?;

                        let algo_indicator = parse_algo_indicator(data, &mut offset);
                        let proof_of_ownership = parse_not_nullable(data, data_len, &mut offset)?;
                        let challenge = parse_not_nullable(data, data_len, &mut offset)?;
                        let additional = parse_nullable(data, data_len, &mut offset)?;

                        Ok(Self::VerifyProofOfOwnershipBidirectional {
                            algo_indicator,
                            proof_of_ownership,
                            challenge,
                            additional,
                        })
                    },
                    AuthenticationTask::AuthenticationConfiguration => {
                        utils::data_length_check(data_len, 0, true)?;

                        Ok(Self::AuthenticationConfiguration)
                    },
                }
            },
            None => panic!("Sub-function required"),
        }
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

impl Into<Vec<u8>> for Authentication {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        match self {
            Self::DeAuthenticate => {},
            Self::VerifyCertificateUnidirectional {
                config,
                certificate,
                challenge,
            } => {
                result.push(config);
                result.append(&mut certificate.into());
                result.append(&mut challenge.into());
            },
            Self::VerifyCertificateBidirectional {
                config,
                certificate,
                challenge,
            } => {
                result.push(config);
                result.append(&mut certificate.into());
                result.append(&mut challenge.into());
            },
            Self::ProofOfOwnership {
                proof_of_ownership,
                ephemeral_public_key,
            } => {
                result.append(&mut proof_of_ownership.into());
                result.append(&mut ephemeral_public_key.into());
            },
            Self::TransmitCertificate {
                cert_evaluation_id,
                certificate,
            } => {
                result.extend(cert_evaluation_id.to_be_bytes());
                result.append(&mut certificate.into());
            },
            Self::RequestChallengeForAuthentication {
                config,
                algo_indicator,
            } => {
                result.push(config);
                result.append(&mut algo_indicator.into());
            },
            Self::VerifyProofOfOwnershipUnidirectional {
                algo_indicator,
                proof_of_ownership,
                challenge,
                additional,
            } => {
                result.append(&mut algo_indicator.into());
                result.append(&mut proof_of_ownership.into());
                result.append(&mut challenge.into());
                result.append(&mut additional.into());
            },
            Self::VerifyProofOfOwnershipBidirectional {
                algo_indicator,
                proof_of_ownership,
                challenge,
                additional,
            } => {
                result.append(&mut algo_indicator.into());
                result.append(&mut proof_of_ownership.into());
                result.append(&mut challenge.into());
                result.append(&mut additional.into());
            },
            Self::AuthenticationConfiguration => {}
        }

        result
    }
}

pub(crate) fn authentication(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_none() {
        return Err(Error::SubFunctionError(service));
    }

    let sf = AuthenticationTask::try_from(sub_func.unwrap().function)?;
    let _ = Authentication::try_parse(data.as_slice(), Some(sf), cfg)?;

    Ok(Request { service, sub_func, data })
}
