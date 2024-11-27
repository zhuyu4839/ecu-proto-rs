use getset::{CopyGetters, Getters};
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters, Getters)]
pub struct UserDefine {
    #[getset(get_copy = "pub")]
    pub(crate) sub_cmd: u8,
    #[getset(get = "pub")]
    pub(crate) param: Vec<u8>,
}

impl UserDefine {
    pub fn new(sub_cmd: u8, param: Vec<u8>) -> Self {
        Self { sub_cmd, param }
    }

    pub const fn length() -> usize {
        1
    }
}

impl Into<Vec<u8>> for UserDefine {
    fn into(mut self) -> Vec<u8> {
        let mut result = vec![self.sub_cmd, ];
        result.append(&mut self.param);

        result
    }
}

impl TryFrom<&[u8]> for UserDefine {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let sub_cmd = data[offset];
        offset += 1;
        let data = data[offset..].to_vec();

        Ok(Self::new(sub_cmd, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_define() -> anyhow::Result<()> {
        let request = UserDefine::new(0x00, vec![0x12, 0x34]);
        let data: Vec<_> = request.into();
        assert_eq!(data, vec![0x00, 0x12, 0x34]);

        let request = UserDefine::try_from(data.as_slice())?;
        assert_eq!(request.sub_cmd, 0x00);
        assert_eq!(request.param, vec![0x12, 0x34]);

        Ok(())
    }
}

