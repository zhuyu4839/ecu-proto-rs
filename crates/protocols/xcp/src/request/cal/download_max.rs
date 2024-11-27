use getset::Getters;
use crate::{AddressGranularity, TryFromWith, IntoWith, XcpError};

#[derive(Debug, Clone, Getters)]
#[get = "pub"]
pub struct DownloadMax {
    pub(crate) elements: Vec<u8>,
}

impl DownloadMax {
    pub fn new(elements: Vec<u8>) -> Self {
        Self { elements }
    }
}

impl IntoWith<Vec<u8>, AddressGranularity> for DownloadMax {
    fn into_with(mut self, ag: AddressGranularity) -> Vec<u8> {
        // Depending upon AG, 1 or 3 alignment bytes must be used in order to meet alignment requirements.
        let mut result = match ag {
            AddressGranularity::Byte => vec![],
            AddressGranularity::Word => vec![0x00, ],
            AddressGranularity::DWord => vec![0x00, 0x00, 0x00]
        };
        result.append(&mut self.elements);

        result
    }
}

impl TryFromWith<&[u8], AddressGranularity> for DownloadMax {
    type Error = XcpError;

    fn try_from_with(data: &[u8], ag: AddressGranularity) -> Result<Self, Self::Error> {
        let offset = match ag {
            AddressGranularity::Byte => 0x00,
            AddressGranularity::Word => 0x01,
            AddressGranularity::DWord => 0x03,
        };

        Ok(Self::new(data[offset..].to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_max() -> anyhow::Result<()> {
        let request = DownloadMax::new(vec![0x00, 0x01, 0x02, 0x03]);
        let data: Vec<_> = request.into_with(AddressGranularity::Byte);
        assert_eq!(vec![0x00, 0x01, 0x02, 0x03], data);
        let request = DownloadMax::try_from_with(&data, AddressGranularity::Byte)?;
        assert_eq!(vec![0x00, 0x01, 0x02, 0x03], request.elements);

        let data: Vec<_> = request.into_with(AddressGranularity::Word);
        assert_eq!(vec![0x00, 0x00, 0x01, 0x02, 0x03], data);
        let request = DownloadMax::try_from_with(&data, AddressGranularity::Word)?;
        assert_eq!(vec![0x00, 0x01, 0x02, 0x03], request.elements);

        let data: Vec<_> = request.into_with(AddressGranularity::DWord);
        assert_eq!(vec![0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03], data);
        let request = DownloadMax::try_from_with(&data, AddressGranularity::DWord)?;
        assert_eq!(vec![0x00, 0x01, 0x02, 0x03], request.elements);

        Ok(())
    }
}
