use crate::constant::SIZE_OF_LENGTH;
use crate::DoIpError;

pub(crate) fn data_len_check(data: &[u8], struct_len: usize, equal: bool) -> Result<(usize, usize), DoIpError> {
    let data_len = data.len();
    let expected = SIZE_OF_LENGTH + struct_len;
    if equal {
        if data_len != expected {
            return Err(DoIpError::InvalidLength { actual: data_len, expected });
        }

        let length = u32::from_be_bytes(data[..SIZE_OF_LENGTH].try_into().unwrap()) as usize;
        if length != struct_len {
            return Err(DoIpError::InvalidDataLen { actual: length, expected: struct_len });
        }
    }
    else {
        if data_len < expected {
            return Err(DoIpError::InvalidLength { actual: data_len, expected });
        }

        let length = u32::from_be_bytes(data[..SIZE_OF_LENGTH].try_into().unwrap()) as usize;
        if length < struct_len {
            return Err(DoIpError::InvalidDataLen { actual: length, expected: struct_len });
        }
    }

    Ok((data_len, SIZE_OF_LENGTH))
}
