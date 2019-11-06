pub mod algos;
pub mod kasai;
use crate::util::errors::{Error};


pub trait Compute {
    fn compute(text: String, sa: Vec<usize>)-> Result<Vec<usize>, Error>;
}
