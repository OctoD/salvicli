extern crate clap;
use clap::{
    App,
    Arg
};

fn main() {
    App::new("salvicli")
       .version("1.0")
       .about("Prima l'Itaglia e gli Itagliani!")
       .arg(
           Arg::with_name("ruspa")
            .long("ruspa")
            .help("similar to rm -rf, but with racism inside it")
            .takes_value(true)
       )
       .arg(
           Arg::with_name("a-casa-loro")
            .short("acl")
            .long("a-casa-loro")
            .help("similar to mv, but with a lot of racism inside it")
            .takes_value(true)
       )
       .arg(
           Arg::with_name("pieni-poteri")
            .short("cmildvce")
            .long("pieni-poteri")
            .help("just a sudo, but in a crappy fascist")
            .takes_value(true)
       )
       .author("Paolo R.")
       .get_matches(); 
}