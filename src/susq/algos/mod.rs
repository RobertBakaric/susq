pub mod lcp;
//pub mod plcp;


//use lcp::kasai::LcpKas;
use crate::util::errors::Error;



pub trait Lcp <T>{
    fn lcp_compute(text: String, sa: Vec<T>) -> Result<Vec<T>,Error>;
}
