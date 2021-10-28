
use argparse::{ArgumentParser, StoreTrue, List, Store};
use home;

mod waypoints;


fn main() {
    // Goto is the default behaviour. If any (non-verbose) arguments are specified, this does
    // nothing.
    let mut go_to: String = String::new();
    
    let mut verbose = false;
    let mut list = false;
    let mut add: Vec<String> = Vec::new();
    let mut drop: Vec<String> = Vec::new();
    { 
        let mut ap = ArgumentParser::new();
        ap.set_description("Helps navigate between and inside directories structures.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        ap.refer(&mut list)
            .add_option(&["-l", "--list"], StoreTrue,
            "List all waypoints");
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

    let mut waypoints_file = home::home_dir().unwrap();
    waypoints_file.push(".config/adg/waypoints.json");
    let mut wp: waypoints::Waypoints = waypoints::load(&waypoints_file);
    wp.drop( &drop );
    wp.add( &add );
    wp.save( &waypoints_file );
    // If we want to go_to somewhere, we need to communicate that to the shell
    // script that wraps this.
    if list && go_to.is_empty() {
        println!( "{}", &wp);
    }
    else if !go_to.is_empty() {
        println!( "{}", &wp.goto( go_to.clone() ));
    }

}
