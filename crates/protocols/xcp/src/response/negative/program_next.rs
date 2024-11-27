use getset::CopyGetters;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct ProgramNextNegative {
    // pub(crate) err_sequence: u8,
    /// Number of expected data elements
    pub(crate) expect_number: u8,
}


