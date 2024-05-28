use crate::error::Error;
use crate::tee::TEEType;
pub trait TEEVerifier {
    fn verify(
        &self,
        report: String,
        skip_verify_image_digest: bool,
    ) -> impl std::future::Future<Output = Result<bool, Error>> + Send;
    fn get_type(&self) -> Result<TEEType, Error>;
}
