//! Service 38

#[cfg(any(feature = "std2013", feature = "std2020"))]
#[cfg(test)]
mod tests {
    use iso14229_1::{request, Configuration, DataFormatIdentifier, ModeOfOperation, RequestData};

    #[test]
    fn test_request_file_transfer_request() -> anyhow::Result<()> {
        // D:\mapdata\europe\germany1.yxz
        let source = hex::decode("3801001E443A5C6D6170646174615C6575726F70655C6765726D616E79312E79787A1102C3507530")?;

        let cfg = Configuration::default();
        let request = request::RequestFileTransfer::try_parse(&source[2..], Some(ModeOfOperation::AddFile), &cfg)?;
        match request {
            request::RequestFileTransfer::AddFile {
                filepath,
                dfi,
                filesize_len,
                uncompressed_size,
                compressed_size,
            } => {
                assert_eq!(filepath, r"D:\mapdata\europe\germany1.yxz".to_string());
                assert_eq!(dfi, DataFormatIdentifier::new(0x01, 0x01));
                assert_eq!(filesize_len, 0x02);
                assert_eq!(uncompressed_size, 0xC350);
                assert_eq!(compressed_size, 0x7530);
            },
            _ => panic!(),
        }

        Ok(())
    }
}
