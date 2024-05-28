use crate::error::Error;
use crate::tee::TEEType;
pub trait TEEProvider {
    fn get_report(&self) -> Result<String, Error>;
    fn get_type(&self) -> Result<TEEType, Error>;
}
