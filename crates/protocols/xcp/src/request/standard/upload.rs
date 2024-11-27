use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct Upload {
    /// n = Number of data elements [AG]
    /// [1..MAX_CTO/AG -1] Standard mode
    /// [1..255] Block mode
    pub(crate) size: u8,
}

impl Upload {
    pub fn new(size: u8) -> Self {
        Self { size }
    }

    pub const fn length() -> usize {
        1
    }
}

impl Into<Vec<u8>> for Upload {
    fn into(self) -> Vec<u8> {
        vec![self.size, ]
    }
}

impl TryFrom<&[u8]> for Upload {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let size = data[0];

        Ok(Self::new(size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload() -> anyhow::Result<()> {
        let request = Upload::new(0x10);
        let data: Vec<_> = request.into();
        assert_eq!(data, vec![0x10]);

        let request = Upload::try_from(data.as_slice())?;
        assert_eq!(request.size, 0x10);

        Ok(())
    }
}
