/*
* Copyright (c) <2019> <Robert Bakaric : rbakaric@exaltum.eu>
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use clap::*;


pub fn parse_cli ()->  clap::ArgMatches<'static> {

    let head : &str =
"
-------------------------------------------------
Logo or something goes HERE!!
-------------------------------------------------
 ";

    let  matches = App::new("susq")
          .version("0.01")
          .author("Robert Bakaric <rbakaric@irb.hr>")
          .about(head)
          .arg(Arg::with_name("input")
               .short("i")
               .long("input")
               .required(true)
               .value_name("FILE")
               .help("Input file [txt,fasta,fastq]")
               .takes_value(true))
          .arg(Arg::with_name("output")
               .short("o")
               .long("output")
               .required(false)
               .value_name("FILE")
               .default_value("stdout")
               .help("Output file")
               .takes_value(true))
          .arg(Arg::with_name("v")
               .short("v")
               .multiple(true)
               .help("Sets the level of verbosity"))
          .subcommand(SubCommand::with_name("kasai")
                      .about("Utilize Kasai's LCP approach")
                      .version("1.0")
                      .author("Robert Bakaric <rbakaric@irb.hr>"))
          .subcommand(SubCommand::with_name("kark")
                    .about("Utilize Karkannines's PLCP approach ")
                    .version("1.0")
                    .author("Robert Bakaric <rbakaric@irb.hr>")
                    .arg(Arg::with_name("algo")
                            .short("a")
                            .long("algo")
                            .required(false)
                            .value_name("fi|ir")
                            .default_value("fi")
                            .help("Aglorithm to use")
                            .takes_value(true)))
          .subcommand(SubCommand::with_name("gog")
                    .about("Utilize Gog's susq approach")
                    .version("1.0")
                    .author("Robert Bakaric <rbakaric@irb.hr>"))
          .subcommand(SubCommand::with_name("bak")
                    .about("Utilize Bakaric's susq approach")
                    .version("1.0")
                    .author("Robert Bakaric <rbakaric@irb.hr>"))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
        //let output = matches.value_of("output").unwrap_or("stdout");
    //println!("Value for output: {}", matches.value_of("output").unwrap());

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    //println!("Using input file: {}", matches.value_of("input").unwrap());

    //println!("Using direction: {}", matches.value_of("direction").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
   /* match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
*/
    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
/*
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }
*/
    matches
    // more program logic goes here...
}
