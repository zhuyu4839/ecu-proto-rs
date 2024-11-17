use iso13400_2::{ActiveCode, DiagnosticNegativeCode, HeaderNegativeCode, Iso13400Error};

#[derive(Debug, thiserror::Error)]
pub enum DoIpError {
    #[error("DoIP - io error: {0}")]
    IoError(std::io::Error),
    #[error("DoIP - {0}")]
    Iso13400Error(Iso13400Error),

    #[error("DoIP - response header negative code: {0:?}")]
    HeaderNegativeError(HeaderNegativeCode),

    #[error("DoIP - routing active error code: {0:?}")]
    ActiveError(ActiveCode),

    #[error("DoIP - diagnostic negative code: {code:?}, previous diagnostic message: {data}")]
    DiagnosticNegativeError { code: DiagnosticNegativeCode, data: String },
}
