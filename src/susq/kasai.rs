use std::ops::{Add,Sub};
use crate::util::errors::{Error};
use crate::susq::{Compute,SusArray};
use crate::susq::algos::{Lcp,LcpArray};
use std::fmt::Debug;
use std::convert::TryInto;



impl <T>Compute<T> for SusArray<T>
    where T: Copy +
             Add<Output = T> +
             Sub<Output = T> +
             PartialOrd +
             From<u8> +
             Into<usize>,
             usize: TryInto<T>,
             <usize as TryInto<T>>::Error: Debug,
    {
    fn suscomp_kasai(text: String, sa: Vec<T>)-> Result<SusArray<T>, Error>{

        let lcp= LcpArray::kasai_compute(text,sa)?;
        //let lcp : KasSusA =  sa;
        Ok(lcp)

    }

    fn suscomp_karkk(text: String, sa: Vec<T>)-> Result<SusArray<T>, Error>{
        Ok(vec![0.try_into().expect("xxxx");10])
    }

    fn suscomp_gog(text: String, sa: Vec<T>)-> Result<SusArray<T>, Error>{
        Ok(vec![0.try_into().expect("xxxx");10])
    }
    fn suscomp (sa: Vec<T>)-> SusArray<T>{
        vec![0.try_into().expect("xxxx");10]
    }

    fn suscomp_bakar(text: String, sa: Vec<T>)-> Result<SusArray<T>, Error>{
        Ok(vec![0.try_into().expect("xxxx");10])
    }
}
