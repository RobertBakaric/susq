use std;
use crate::util::errors::LCPError;


//! # Usage Example
//!
//! ```rust

//! use susq::alphabets;
//! use susq::algos::lcp::Kasai;
//!
//! let string = b"ACGGATGCTGGATCGGATCGCGCTAGCTA$";
//!
//! // Compute susq array for a given string.
//! let alphabet = alphabets::dna::iupac_alphabet();
//! let susq = SusQ::new(string,suffarray);


use std::ops::{Add,Sub};
use std::convert::TryInto;


pub type LcpKas<T>  = Vec<T>;


impl <T> Compute<T> for LcpKas<T>
    where T:Copy +
            Add<Output = T> +
            Sub<Output = T> +
            PartialOrd +
            From<u8> +
            Into<usize>
{
    fn compute(t: String, sa: Vec<T>) -> Result<LcpKas<T>,LCPError>{

        assert_eq!(t.len(), sa.len());
        let n = t.len();
        let text = t.as_bytes();

        // provide the lexicographical rank for each suffix
        let mut rank = vec![0;n];
        for (r, p) in sa.iter().enumerate() {
            rank[(*p).into()] = r;
        }

        let mut lcp : Vec<T> = vec![T::from(0);n];
        let mut l = 0;
        for (p, &r) in rank.iter().enumerate().take(n - 1) {
            let pred = sa[r - 1].into();
            while pred + l < n &&  p + l < n && text[p + l] == text[pred + l] {
                l += 1;
            }
            lcp[r] = T::from(l.try_into().unwrap());
            l = if l > 0 {
                 l - 1
             } else {
                 0
             };
        }
        Ok(lcp)
    }
}
