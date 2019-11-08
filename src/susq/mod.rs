pub mod algos;
pub mod kasai;
use crate::util::errors::{Error};



pub type SusArray<T> = Vec<T>;


pub trait Compute <T> {
    fn suscomp_kasai(tex: String, sa: Vec<T>)-> Result<SusArray<T>, Error>;
    fn suscomp_karkk(text: String, sa: Vec<T>)-> Result<SusArray<T>, Error>;
    fn suscomp      (sa: Vec<T>)->SusArray<T>;
    fn suscomp_gog  (tex: String, sa: Vec<T>)-> Result<SusArray<T>, Error>;
    fn suscomp_bakar(text: String, sa: Vec<T>)-> Result<SusArray<T>, Error>;
}
