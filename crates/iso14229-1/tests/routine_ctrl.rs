//! Service 31

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, CheckProgrammingDependencies};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let source = hex::decode("3101FF01")?;
        let request = request::RoutineCtrl {
            routine_id: CheckProgrammingDependencies,
            option_record: vec![],
        };
        let result: Vec<_> = request.into();
        assert_eq!(result, source[2..].to_vec());

        let source = hex::decode("3101FF01112233445566")?;
        let request = request::RoutineCtrl::try_from(&source[2..])?;

        assert_eq!(request.routine_id, CheckProgrammingDependencies);
        assert_eq!(request.option_record, hex::decode("112233445566")?);

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let source = hex::decode("7101FF01")?;
        let response = response::RoutineCtrl::new(
            CheckProgrammingDependencies,
            None,
            vec![]
        )?;
        let result: Vec<_> = response.into();
        assert_eq!(result, source[2..].to_vec());

        let source = hex::decode("7101FF01112233445566")?;
        let response = response::RoutineCtrl::try_from(&source[2..])?;

        assert_eq!(response.routine_id, CheckProgrammingDependencies);
        assert_eq!(response.routine_info, Some(0x11));
        assert_eq!(response.routine_status, hex::decode("2233445566")?);

        Ok(())
    }
}
