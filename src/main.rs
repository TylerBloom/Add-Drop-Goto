
use argparse::{ArgumentParser, StoreTrue, List, Store};
use std::fs;

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

    //println!( "Add: {:?}", add );
    //println!( "Drop: {:?}", drop );
    //println!( "Goto: {:?}", go_to );

    let mut tmp: waypoints::Waypoints = waypoints::load(&String::from("~/.config/adg/waypoints.json"));
    tmp.drop( &drop );
    tmp.add( &add );
    tmp.save( String::from("~/.config/adg/waypoints.json") );
    if !go_to.is_empty() {
        //println!( "Going to {}", tmp.goto( go_to.clone() ) );
        println!( "{}", &tmp.goto( go_to.clone() ));
        //GoTo(&tmp.goto( go_to.clone() ) );
    }
    //println!( "{}", tmp );



}
