pub mod kasai;
// pub mod karkkainen;


pub trait Compute <T>{
    fn compute(text: String, sa: Vec<T>) -> Result<Lcp<T>,LCPError>;
}


#[derive(Debug, Fail)]
pub enum Error {
    /// An error occurred while uploading the file.
    #[fail(display = "")]
    Lcp(#[cause] LcpError),
}


impl From<LcpError> for Error {
    fn from(err: LcpError) -> Error {
        Error::Lcp(err)
    }
}
