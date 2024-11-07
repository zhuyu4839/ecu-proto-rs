//! response of Service 38

use crate::{ByteOrder, Configuration, error::UdsError, DataFormatIdentifier, ModeOfOperation, request::{Request, SubFunction}, RequestData, utils, Service};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RequestFileTransfer {
    AddFile {
        filepath: String,
        dfi: DataFormatIdentifier,
        filesize_len: u8,
        uncompressed_size: u128,
        compressed_size: u128,
    },
    DeleteFile {    // 2
        filepath: String,
    },
    ReplaceFile {   // 3
        filepath: String,
        dfi: DataFormatIdentifier,
        filesize_len: u8,
        uncompressed_size: u128,
        compressed_size: u128,
    },
    ReadFile {       // 4
        filepath: String,
        dfi: DataFormatIdentifier,
    },
    ReadDir {       // 5
        filepath: String,
    },
    ResumeFile {    // 6
        filepath: String,
        dfi: DataFormatIdentifier,
        filesize_len: u8,
        uncompressed_size: u128,
        compressed_size: u128,
    },
}

impl RequestData for RequestFileTransfer {
    type SubFunc = ModeOfOperation;
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, UdsError> {
        match sub_func {
            Some(v) => {
                let data_len = data.len();
                utils::data_length_check(data_len, 3, false)?;
                let mut offset = 0;

                let len = u16::from_be_bytes([data[offset], data[offset + 1]]) as usize;
                offset += 2;
                utils::data_length_check(data_len, offset + len + 1, false)?;

                let filepath = String::from_utf8(
                    data[offset..offset + len].to_vec()
                ).map_err(|_| UdsError::InvalidData(hex::encode(data)))?;
                offset += len;

                match v {
                    ModeOfOperation::AddFile => {
                        let dfi = DataFormatIdentifier::from(data[offset]);
                        offset += 1;
                        let filesize_len = data[offset];
                        offset += 1;
                        let uncompressed_size = utils::slice_to_u128(&data[offset..offset + filesize_len as usize], ByteOrder::Big);
                        offset += filesize_len as usize;
                        let compressed_size = utils::slice_to_u128(&data[offset..offset + filesize_len as usize], ByteOrder::Big);
                        Ok(Self::AddFile { filepath, dfi, filesize_len, uncompressed_size, compressed_size })
                    },
                    ModeOfOperation::ReplaceFile => {
                        let dfi = DataFormatIdentifier::from(data[offset]);
                        offset += 1;
                        let filesize_len = data[offset];
                        offset += 1;
                        let uncompressed_size = utils::slice_to_u128(&data[offset..offset + filesize_len as usize], ByteOrder::Big);
                        offset += filesize_len as usize;
                        let compressed_size = utils::slice_to_u128(&data[offset..offset + filesize_len as usize], ByteOrder::Big);
                        Ok(Self::ReplaceFile { filepath, dfi, filesize_len, uncompressed_size, compressed_size })
                    },
                    ModeOfOperation::ResumeFile => {
                        let dfi = DataFormatIdentifier::from(data[offset]);
                        offset += 1;
                        let filesize_len = data[offset];
                        offset += 1;
                        let uncompressed_size = utils::slice_to_u128(&data[offset..offset + filesize_len as usize], ByteOrder::Big);
                        offset += filesize_len as usize;
                        let compressed_size = utils::slice_to_u128(&data[offset..offset + filesize_len as usize], ByteOrder::Big);
                        Ok(Self::ResumeFile { filepath, dfi, filesize_len, uncompressed_size, compressed_size })
                    },
                    ModeOfOperation::DeleteFile => Ok(Self::DeleteFile { filepath }),
                    ModeOfOperation::ReadDir => Ok(Self::ReadDir { filepath }),
                    ModeOfOperation::ReadFile => {
                        let dfi = DataFormatIdentifier::from(data[offset]);

                        Ok(Self::ReadFile {
                            filepath,
                            dfi
                        })
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

impl Into<Vec<u8>> for RequestFileTransfer {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        match &self {
            Self::AddFile { .. } => result.push(ModeOfOperation::AddFile.into()),
            Self::DeleteFile { .. } => result.push(ModeOfOperation::DeleteFile.into()),
            Self::ReplaceFile { .. } => result.push(ModeOfOperation::ReplaceFile.into()),
            Self::ReadFile { .. } => result.push(ModeOfOperation::ReadFile.into()),
            Self::ReadDir { .. } => result.push(ModeOfOperation::ReadDir.into()),
            Self::ResumeFile { .. } => result.push(ModeOfOperation::ResumeFile.into()),
        }
        match self {
            Self::AddFile {
                filepath,
                dfi,
                filesize_len,
                uncompressed_size,
                compressed_size,
            } |
            Self::ReplaceFile {
                filepath,
                dfi,
                filesize_len,
                uncompressed_size,
                compressed_size,
            } |
            Self::ResumeFile {
                filepath,
                dfi,
                filesize_len,
                uncompressed_size,
                compressed_size,
            } => {
                let mut bytes: Vec<_> = filepath.bytes().collect();
                result.extend((bytes.len() as u16).to_be_bytes());
                result.append(&mut bytes);
                result.push(dfi.into());
                result.push(filesize_len);

                result.append(&mut utils::u128_to_vec(uncompressed_size, filesize_len as usize, ByteOrder::Big));
                result.append(&mut utils::u128_to_vec(compressed_size, filesize_len as usize, ByteOrder::Big));
            },
            Self::DeleteFile { filepath, } |
            Self::ReadDir { filepath, } => {
                let mut bytes: Vec<_> = filepath.bytes().collect();
                result.extend((bytes.len() as u16).to_be_bytes());
                result.append(&mut bytes);
            },
            Self::ReadFile { filepath, dfi, } => {
                let mut bytes: Vec<_> = filepath.bytes().collect();
                result.extend((bytes.len() as u16).to_be_bytes());
                result.append(&mut bytes);
                result.push(dfi.into());
            },
        }

        result
    }
}

pub(crate) fn request_file_transfer(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, UdsError> {
    if sub_func.is_none() {
        return Err(UdsError::SubFunctionError(service));
    }

    let sf = ModeOfOperation::try_from(sub_func.unwrap().function)?;
    let _ = RequestFileTransfer::try_parse(data.as_slice(), Some(sf), cfg)?;

    Ok(Request { service, sub_func, data })
}
