//! Service 2C

#[cfg(test)]
mod tests {
    use iso14229_1::{request, Configuration, DynamicallyDID, DynamicallyMemAddr, RequestData};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = Configuration::default();
        let source = hex::decode("2C01F30112340102567801019ABC0104")?;
        let request = request::DynamicallyDefineDID::DefineByIdentifier {
            did: DynamicallyDID::try_from(0xF301)?,
            source: DynamicallyMemAddr {
                did: 0x1234,
                position: 1,
                mem_size: 2,
            },
            others: vec![
                DynamicallyMemAddr {
                    did: 0x5678,
                    position: 1,
                    mem_size: 1,
                },
                DynamicallyMemAddr {
                    did: 0x9ABC,
                    position: 1,
                    mem_size: 4,
                }
            ]
        };
        let result: Vec<_> = request.to_vec(&cfg);
        assert_eq!(result, source[2..].to_vec());

        let source = hex::decode("2C02F302240009196900012109196900012109196b0102131019950001")?;
        let request = request::DynamicallyDefineDID::DefineByMemoryAddress {
            did: DynamicallyDID::try_from(0xF302)?,
            memory: (0x00091969, 1),
            others: vec![
                (0x21091969, 1),
                (0x2109196B, 0x0102),
                (0x13101995, 1),
            ],
        };
        let result: Vec<_> = request.to_vec(&cfg);
        assert_eq!(result, source[2..].to_vec());

        Ok(())
    }
}
