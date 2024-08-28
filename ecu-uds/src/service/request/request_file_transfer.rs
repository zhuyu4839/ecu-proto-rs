//! response of Service 38


use isotp_rs::ByteOrder;
use crate::error::Error;
use crate::service::{Configuration, DataFormatIdentifier, ModeOfOperation, RequestData};
use crate::utils;

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
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
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
                ).map_err(|_| Error::InvalidData(utils::hex_slice_to_string(data)))?;
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

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::service::{Configuration, DataFormatIdentifier, ModeOfOperation, RequestData};
    use super::RequestFileTransfer;

    #[test]
    fn add_file() -> anyhow::Result<()> {
        // D:\mapdata\europe\germany1.yxz
        let source = hex!("3801001E443A5C6D6170646174615C6575726F70655C6765726D616E79312E79787A1102C3507530").as_slice();

        let cfg = Configuration::default();
        let request = RequestFileTransfer::try_parse(&source[2..], Some(ModeOfOperation::AddFile), &cfg)?;
        match request {
            RequestFileTransfer::AddFile {
                filepath,
                dfi,
                filesize_len,
                uncompressed_size,
                compressed_size,
            } => {
                assert_eq!(filepath, r"D:\mapdata\europe\germany1.yxz".to_string());
                assert_eq!(dfi, DataFormatIdentifier(0x11));
                assert_eq!(filesize_len, 0x02);
                assert_eq!(uncompressed_size, 0xC350);
                assert_eq!(compressed_size, 0x7530);
            },
            _ => panic!(),
        }

        Ok(())
    }
}