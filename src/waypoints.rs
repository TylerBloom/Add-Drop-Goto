use std::fs::{read_to_string, metadata, canonicalize};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct RawWaypoints {
    locale_names: Vec<String>,
    locale_values: Vec<String>,
    site_names: Vec<String>,
    site_values: Vec<String>,
}

pub struct Waypoints {
    locales: HashMap<String, String>,
    sites: HashMap<String, String>,
}

pub fn new( ) -> Waypoints {
    Waypoints {
        locales: HashMap::new(),
        sites: HashMap::new()
    }
}

pub fn load( filename: &PathBuf ) -> Waypoints {
    let file_data = read_to_string(filename).expect("Waypoints file was not found...");
    let input_waypoints: RawWaypoints = serde_json::from_str(&file_data).expect("Waypoints file was malformed...");
    let mut digest: Waypoints = new();
    for (i, name) in input_waypoints.locale_names.iter().enumerate() {
        digest.locales.insert( name.clone(), input_waypoints.locale_values[i].clone() );
    }
    for (i, name) in input_waypoints.site_names.iter().enumerate() {
        digest.sites.insert( name.clone(), input_waypoints.site_values[i].clone() );
    }
    digest
}

impl std::fmt::Display for Waypoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Locales:\n")?;
        for (key, val) in &self.locales {
            write!(f, "\t{} : {}\n", key, val)?;
        }
        write!(f, "\nSites:\n")?;
        for (key, val) in &self.sites {
            write!(f, "\t{} : {}\n", key, val)?;
        }
        Ok(())
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
    
    pub fn save( &self, filename: &PathBuf ) {
        let mut content = String::from("{");
        let mut locale_names = String::from("\n\t\"locale_names\" : [ ");
        let mut locale_values = String::from("\n\t\"locale_values\" : [ ");
        let mut first: bool = true;
        for (name, value) in &self.locales {
            locale_names += if first {""} else {", "};
            locale_names += &format!( "\"{}\"", &name );
            locale_values += if first {""} else {", "};
            locale_values += &format!( "\"{}\"", &value );
            first = false;
        }
        locale_names += "],";
        locale_values += "],";
        let mut site_names = String::from("\n\t\"site_names\" : [ ");
        let mut site_values = String::from("\n\t\"site_values\" : [ ");
        first = true;
        for (name, value) in &self.sites {
            site_names += if first {""} else {", "};
            site_names += &format!( "\"{}\"", name );
            site_values += if first {""} else {", "};
            site_values += &format!( "\"{}\"", value );
            first = false;
        }
        site_names += "],";
        site_values += "]";
        content += &locale_names;
        content += &locale_values;
        content += &site_names;
        content += &site_values;
        content += "\n}";
        fs::write(filename, &content).expect( "Could not write to file..." );
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
                    self.locales.insert(
                        loc[i].clone(),
                        canonicalize(
                            PathBuf::from(&loc[i+1])
                        ).unwrap().as_path().to_str().unwrap().to_string() );
                } else if path_one_exists && !path_two_exists {
                    self.locales.insert(
                        loc[i+1].clone(),
                        canonicalize(
                            PathBuf::from(&loc[i])
                        ).unwrap().as_path().to_str().unwrap().to_string() );
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
        let mut digest = PathBuf::from("");
        for l in loc.split( "/" ) {
            digest.push( l );
            if digest.as_path().is_dir() {
                continue;
            } else {
                digest.pop();
                digest.push( self.get( String::from(l) ) );
            }
        }
        digest.as_path().to_str().unwrap().to_string()
    }

}
