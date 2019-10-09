pub mod algos;



//  Here make a  new susq object and call
//  different algos depending on which approacj is being used


pub struct SuSQ <T> {
    sa: Vec<T>,
    text: String,
    susa: Vec<T>
}



impl <T> SuSQ <T>{
    // implement the object
    pub fn new(self, text: String)-> Self{
        // compute sa

        // store string
        // alocate array for sus
        SuSQ{
            sa: sufa,
            text: text,
            susa: susa
        }
    }
}



pub trait SusQ <T>{

    fn compute(text:String)->  Result< Vec<T>, ComErr >;

    //fn query(i:T, j:T)-> Result<Vec<T>, QuErr>;

    //fn query(i:T)-> Result<T,QuErr>;
}



impl SusQ <i32> for SuSQ<i32> {

    fn compute(text:String)->  Result< Vec<i32>, ComErr >

}

impl SusQ <i64> for SuSQ<i64> {

    fn compute(text:String)->  Result< Vec<i64>, ComErr >

}
