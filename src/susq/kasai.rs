use crate::util::errors::{Error};

use crate::susq::Compute;
use crate::susq::algos::Lcp;
use crate::susq::algos::lcp::kasai::{KasLcp};

pub type KasSusA = Vec<usize>;


impl Compute for KasSusA{

    fn compute(text: String, sa: Vec<usize>)-> Result<Vec<usize>, Error>{

        let lcp= KasLcp::lcp_compute(text,sa)?;
        //let lcp : KasSusA =  sa;
        Ok(lcp)

    }
}
