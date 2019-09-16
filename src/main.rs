extern crate clap;
use clap::{
    App,
    Arg
};

mod commands;

fn main() {
    let matches = App::new("salvicli")
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
            .min_values(2)
            .max_values(2)
       )
       .author("Paolo R.")
       .get_matches(); 

    if let Some(args) = matches.values_of("ruspa") {
        commands::ruspa::run(args);
    }

    if let Some(args) = matches.values_of("a-casa-loro") {
        let _result = commands::a_casa_loro::run(args);
    }
}