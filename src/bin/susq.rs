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



//! This is main binary




mod cli;
use cli::susq_cli::*;


use susq::{SuSQ,SuSComp};




fn main (){

    // Parse CLI
    let cli = parse_cli();

    // Open and read files
    // true -> record sequential, false -> join records
/*
    let text = read_fastq(
        cli.value_of("input").unwrap(),
        true
    );
*/

    let text = "ACCGCTAGCTA$".to_string();

    // construct an object
    let susq = SuSQ::new(text).make_sa();

    println!("{:?}", susq.get_sa());

    //Alternatives:
    //let sa =  suffix_array(text); // if set an explicit sa is being calculated
    //let susq = SuSQ::new(text,sa);


    // utilize the approach

    match susq.compute(
        cli.subcommand_name()
    ){
        Ok(true) =>  println!("Computation carried out !"),
        Ok(false) => panic!("ERROR: Computation terminated !"),
        Err(e)    => panic!("ERROR: {} ",e)
    }

    // write resultss
/*    write_susa(
        susq.get_susa(),
        cli.value_of("output").unwrap()
    );
*/
}
