//! Service 85

#[cfg(test)]
mod tests {
    use iso14229_1::Configuration;

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("85")?;

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {

        Ok(())
    }
}
