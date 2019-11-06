/*
Copyright <2019> <Robert Bakaric <robertbaklaric@zoho.com> >

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//! # SusQ, a shortest unique substring query/computation library for Rust.
//! The library provides implementations for several different algorithms
//! and data structures used in shortest unique substring computations via
//! either LCP or PLCP information
//!
//! All provided implementations are rigorously tested via continuous
//! integration.
//!
//! For installation instructions and a general overview, visit:
//! ---------------------
//!
//! Currently, susq library provides sus computation utilizing:
//!
//! * Kasai's LCP computation algorithm
//! * Karkainen's PLCP computation algorithm via Fi and naive irreducible plcp array
//! * Gog PLCP computation algorithm via irreducible plcp array
//! * Bakaric direct susq computation algorithm via irreducible plcp array
//!
//! # Example
//!
//! ```rust
//! #use bio::data_structures::suffix_array::suffix_array;
//! #use bio::data_structures::bwt::{bwt, less, Occ};
//! use susq::alphabets;
//! use susq::{SusQ, }
//!
//! let string = b"ACGGATGCTGGATCGGATCGCGCTAGCTA$";
//!
//! // Compute susq array for a given string.
//! let alphabet = alphabets::dna::iupac_alphabet();
//! let susq = SusQ::new(alphabet,Kasai);
//!
//! // Compute susq array
//! let sus_array = susq.compute(string);
//!
//! // Execute query for a given interval [i,j] for i,j \in [1..|string|]
//! let i = 2;
//! let j = string.len()-2;
//! let sus_query = susq.query(i,j);
//!
//! // Execute point interval query for i \in [1..|string|]
//! let sus_query = susq.query(i);
//! ```
//!
//! # Multithreaded Example  (not implemented)
//!
//! ```rust
//!
//! ```
//!
//! Documentation and further examples for each module can be found in the module descriptions below.

// ok thsi is the main library that needs to b e


use bio::data_structures::suffix_array::suffix_array;
use snafu::{ensure,Backtrace, ErrorCompat, ResultExt, Snafu};
mod susq;
mod util;

//use util::utils::{fastq, fasta, file};
use susq::{Compute};
use susq::kasai::{KasSusA};

use util::errors::Error;
use util::errors::Error::{SetErr};

//  Here make a  new susq object and call
//  different algos depending on which approacj is being used

#[derive(Clone,Debug)]
pub struct SuSQ  {
    sufa: Vec<usize>,
    text: String,
    susa: Vec<usize>
}


// Susq Constructor
impl  SuSQ {
    /// Construct a new SuSQ object
    ///
    ///
    /// # Example
    ///
    /// ```
    /// // Constructor can be called by either prowiding an already
    /// // computed suffix array by invocking set_sa() or call  make_sa() to
    /// // utilize objects internal method for creating one
    ///
    /// let txt = "this is my text".to_string();
    ///
    ///
    /// let sa = my_suff_arr_function(txt);
    ///
    /// let susq = SuSQ::new().set_text(txt).set_sa(sa);
    ///
    /// // Alternativly (if you do not have your favorite sa construction algorithm arround)
    ///
    /// let susq = SuSQ::new().set_text(txt).make_sa();
    ///
    /// ```
    pub fn new(text: String)-> Self{
        // store string
        // alocate array for sus and sa
        SuSQ{
            sufa: vec![0;text.len()],
            susa: vec![0;text.len()],
            text: text,
        }
    }
    // builders
    pub fn make_sa(mut self)-> Self {
//        ensure!(self.text.len() > 0, SetErr {a:"set_text()".to_string(),b:"make_sa()".to_string()});
        self.sufa =  suffix_array(&(self.text.as_bytes()));
        self
    }

    // setters
    pub fn set_sa(mut self, sa: Vec<usize>)-> Self {
        self.sufa =  sa;
        self
    }
    pub fn set_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    // getters
    pub fn get_susa (&self)-> Result<Vec<usize>,Error>{
        Ok(self.susa.clone())
    }
    pub fn get_sa (&self)-> Result<Vec<usize>,Error>{
        Ok(self.sufa.clone())
    }
}



pub trait SuSComp {

    fn compute(&self, mode:Option<&str>)->  Result< bool, Error >;

}


pub trait SuSQuery  {

    //fn query(i:T, j:T)-> Result<Vec<T>, QuErr>;

    //fn query(i:T)-> Result<T,QuErr>;
}



impl SuSComp  for SuSQ {

    fn compute(&self, mode:Option<&str>)-> Result<bool, Error>{

        match mode {
            Some("kasai") => {
                let algo = mode.unwrap();
                println!("Using {}...",algo);
                let lcp = KasSusA::compute(self.text.clone(),self.sufa.clone())?;
                println!("lcp{:?}", lcp);
//                kasai::susq(lcp);

            },
            Some("kark") => {
                println!("Not implemented at this point");
    //            susq.compute().karkainnen();
            },
            Some("gog") => {
                println!("Not implemented at this point");
    //            susq.compute().gog();
            },
            Some("bak") => {
                println!("Good choice, but not implemented at this point");
    //            susq.compute().bakaric();
            },
            _ =>  println!(" \nDon't be crazy!! \
                             \nYou have to choose an algorithm! \
                             \n  See help(-h) for details...\n")
         }


         Ok(true)
         //Err(Error::LengthErr{a: 4, b:5})
        // r: T if ok Err if not
    }


}

// make SA -> simply utilize bio lib



// compute LCP Kasai

// compute LCP Karkainnen - 1
// compute LCP Karkainnen - 2

// compute PLCP Karkainnen - 1
// compute PLCP Karkainnen - 2


// compute susq Kasai
// compute susq Karkainnen 1
// compute susq Karkainnen 2
// compute susq Gog
// compute susq Bakaric



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
