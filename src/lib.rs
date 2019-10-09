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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
