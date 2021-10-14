
pub mod Waypoints {

    use std::fs::{metadata, canonicalize};
    use std::path;
    use std::collections::HashMap;
    use std::fmt;

    pub struct Waypoints {
        locales: HashMap<String, String>,
        sites: HashMap<String, String>,
    }

    impl std::fmt::Display for Waypoints {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Locales:\n");
            for (key, val) in &self.locales {
                write!(f, "{} : {}\n", key, val)?;
            }
            write!(f, "\nSites:\n");
            for (key, val) in &self.sites {
                write!(f, "{} : {}\n", key, val)?;
            }
            Ok(())
        }
    }

    pub fn new( ) -> Waypoints {
        Waypoints {
            locales: HashMap::new(),
            sites: HashMap::new()
        }
    }

    impl Waypoints {

        fn get( &self, loc: String ) -> String {
            if self.locales.contains_key( &loc ) {
                self.locales.get( &loc ).unwrap().clone()
            } else if self.sites.contains_key( &loc ){
                self.sites.get( &loc ).unwrap().clone()
            } else {
                loc
            }
        }

        pub fn add( &mut self, loc: &Vec<String> ) {
            if loc.len() == 0 {
                ()
            } else if loc.len() % 2 == 1{
                eprintln!( "Odd number of arguments specified: To add a site or locale, you must specify two arguments." );
                ()
            } else {
                let mut path_one_exists: bool;
                let mut path_two_exists: bool;
                for i in (0..loc.len()-1).step_by(2) {
                    path_one_exists = if metadata(&loc[i]).is_ok() {metadata(&loc[i]).unwrap().is_dir()} else {false};
                    path_two_exists = if metadata(&loc[i+1]).is_ok() {metadata(&loc[i+1]).unwrap().is_dir()} else {false};
                    if !path_one_exists && !path_two_exists {
                        self.sites.insert( loc[i].clone(), loc[i+1].clone() );
                    } else if !path_one_exists && path_two_exists {
                        self.locales.insert( loc[i].clone(), canonicalize(std::path::PathBuf::from(&loc[i+1])).unwrap().as_path().to_str().unwrap().to_string() );
                    } else if path_one_exists && !path_two_exists {
                        self.locales.insert( loc[i+1].clone(), canonicalize(std::path::PathBuf::from(&loc[i])).unwrap().as_path().to_str().unwrap().to_string() );
                    } else {
                        eprintln!( "Both of the following are directories. Please specify at most one directory:\n{}\n{}", loc[i], loc[i+1] );
                    }
                }
            }
        }

        pub fn drop( &mut self, loc: &Vec<String> ) {
            for l in loc {
                self.locales.remove( l );
                self.sites.remove( l );
            }
        }

        pub fn goto( &self, loc: String ) -> String {
            let mut digest = String::new();
            for l in loc.split( "/" ) {
                digest += &self.get( String::from(l) );
                digest += "/"
            }
            digest
        }

    }
}
