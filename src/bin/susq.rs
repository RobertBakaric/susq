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



//! This is my binary file




mod cli;
use cli::susq_cli::*;
use susq::*;




fn main (){

    // Parse CLI
    let cli = parse_cli();

    // Open anNot implemented at this pointd read file
//    let text = read(options.value_of("input").unwrap(), true);
    // true -> record sequential, false -> join records

    // compute SA
//    let sa = compute_suffix_array(text);


//    let susq = SuSQ::new();
    // utilize the approach
    match cli.subcommand_name() {
        Some("kasai") => {
            println!("Using {}...",cli.subcommand_name().unwrap());
//            susq.compute().kasai();

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

}
