//! Service 83

#[cfg(any(feature = "std2006", feature = "std2013"))]
#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, Configuration, TimingParameter, TimingParameterAccessType, TryFromWithCfg};

    /// The TimingParameterRequestRecord is only present if timingParameterAccessType = setTimingParametersToGivenValues.
    /// The structure and content of the TimingParameterRequestRecord is data-link-layer-dependent and therefore defined in the
    /// implementation specification(s) of ISO 14229.
    #[test]
    fn test_request() -> anyhow::Result<()> {   // TODO
        let cfg = Configuration::default();

        let source = hex::decode("8301")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<TimingParameterAccessType>()?, TimingParameterAccessType::ReadExtendedTimingParameterSet);

        let source = hex::decode("8302")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<TimingParameterAccessType>()?, TimingParameterAccessType::SetTimingParametersToDefaultValues);

        let source = hex::decode("8303")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<TimingParameterAccessType>()?, TimingParameterAccessType::ReadCurrentlyActiveTimingParameters);

        let source = hex::decode("830400")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<TimingParameterAccessType>()?, TimingParameterAccessType::SetTimingParametersToGivenValues);
        let data: TimingParameter = request.data::<TimingParameterAccessType, _>(&cfg)?;
        assert_eq!(data, TimingParameter(vec![0x00]));

        Ok(())
    }

    /// The TimingParameterResponseRecord is only present if timingParameterAccessType =
    /// readExtendedTimingParameterSet or readCurrentlyActiveTimingParameters. The structure and content of the
    /// TimingParameterResponseRecord is data-link-layer-dependent and therefore defined in the implementation
    /// specification(s) of ISO 14229.
    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("C30100")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<TimingParameterAccessType>()?, TimingParameterAccessType::ReadExtendedTimingParameterSet);
        let data: TimingParameter = response.data::<TimingParameterAccessType, _>(&cfg)?;
        assert_eq!(data, TimingParameter(vec![0x00]));

        let source = hex::decode("C302")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<TimingParameterAccessType>()?, TimingParameterAccessType::SetTimingParametersToDefaultValues);

        let source = hex::decode("C303")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<TimingParameterAccessType>()?, TimingParameterAccessType::ReadCurrentlyActiveTimingParameters);

        let source = hex::decode("C304")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<TimingParameterAccessType>()?, TimingParameterAccessType::SetTimingParametersToGivenValues);

        Ok(())
    }
}
