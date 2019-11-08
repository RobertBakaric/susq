pub mod lcp;
//pub mod plcp;


//use lcp::kasai::LcpKas;
use crate::util::errors::Error;


pub type LcpArray<T>   = Vec<T>;
pub type pLcpArray<T>  = Vec<T>;


pub trait Lcp <T>{
    fn kasai_compute(text: String, sa: Vec<T>) -> Result<LcpArray<T>,Error>;
    fn karkk_compute(text: String, sa: Vec<T>) -> Result<LcpArray<T>,Error>;
}



pub trait pLcp <T>{
    fn kasai_compute(text: String, sa: Vec<T>) -> Result<pLcpArray<T>,Error>;
    fn karkk_compute(text: String, sa: Vec<T>) -> Result<pLcpArray<T>,Error>;
}
