
use argparse::{ArgumentParser, StoreTrue, List, Store};

mod waypoints;

fn main() {
    // Goto is the default behaviour. If any (non-verbose) arguments are specified, this does
    // nothing.
    let mut go_to: String = String::new();
    
    let mut verbose = false;
    let mut add: Vec<String> = Vec::new();
    let mut drop: Vec<String> = Vec::new();
    { 
        let mut ap = ArgumentParser::new();
        ap.set_description("Helps navigate between and inside directories structures.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        ap.refer(&mut go_to)
            .add_argument("goto", Store,
            "Go somewhere");
        ap.refer(&mut add)
            .add_option(&["-a", "--add"], List,
            "New sites and locales");
        ap.refer(&mut drop)
            .add_option(&["-d", "--drop"], List,
            "Sites and locales to drop");
        ap.parse_args_or_exit();
    }

    println!( "Add: {:?}", add );
    println!( "Drop: {:?}", drop );
    println!( "Goto: {:?}", go_to );

    let mut tmp = waypoints::new();
    tmp.drop( &drop );
    tmp.add( &add );
    if !go_to.is_empty() {
        println!( "{}", tmp.goto( go_to.clone() ) );
    }
    println!( "{}", tmp );



}
