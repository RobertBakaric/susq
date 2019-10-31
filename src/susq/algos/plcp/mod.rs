pub mod karkkainen;




pub trait Compute <T>{
    fn compute(text: String, sa: Vec<T>) -> Result<pLcp<T>,Error>;
}
