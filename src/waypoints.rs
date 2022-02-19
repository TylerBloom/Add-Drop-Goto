use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::fs::{canonicalize, metadata};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Waypoints {
    locales: HashMap<String, String>,
    sites: HashMap<String, String>,
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
    fn get(&self, loc: String) -> String {
        if self.locales.contains_key(&loc) {
            self.locales.get(&loc).unwrap().clone()
        } else if self.sites.contains_key(&loc) {
            self.sites.get(&loc).unwrap().clone()
        } else {
            loc
        }
    }

    pub fn add(&mut self, loc: &Vec<String>) {
        if loc.len() == 0 {
            ()
        } else if loc.len() % 2 == 1 {
            eprintln!( "Odd number of arguments specified: To add a site or locale, you must specify two arguments." );
            ()
        } else {
            let mut path_one_exists: bool;
            let mut path_two_exists: bool;
            for i in (0..loc.len() - 1).step_by(2) {
                path_one_exists = if metadata(&loc[i]).is_ok() {
                    metadata(&loc[i]).unwrap().is_dir()
                } else {
                    false
                };
                path_two_exists = if metadata(&loc[i + 1]).is_ok() {
                    metadata(&loc[i + 1]).unwrap().is_dir()
                } else {
                    false
                };
                if !path_one_exists && !path_two_exists {
                    self.sites.insert(loc[i].clone(), loc[i + 1].clone());
                } else if !path_one_exists && path_two_exists {
                    self.locales.insert(
                        loc[i].clone(),
                        canonicalize(PathBuf::from(&loc[i + 1]))
                            .unwrap()
                            .as_path()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    );
                } else if path_one_exists && !path_two_exists {
                    self.locales.insert(
                        loc[i + 1].clone(),
                        canonicalize(PathBuf::from(&loc[i]))
                            .unwrap()
                            .as_path()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    );
                } else {
                    eprintln!( "Both of the following are directories. Please specify at most one directory:\n{}\n{}", loc[i], loc[i+1] );
                }
            }
        }
    }

    pub fn drop(&mut self, loc: &Vec<String>) {
        for l in loc {
            self.locales.remove(l);
            self.sites.remove(l);
        }
    }

    pub fn goto(&self, loc: String) -> String {
        let mut digest = PathBuf::from("");
        for l in loc.split("/") {
            digest.push(l);
            if digest.as_path().is_dir() {
                continue;
            } else {
                digest.pop();
                digest.push(self.get(String::from(l)));
            }
        }
        digest.as_path().to_str().unwrap().to_string()
    }

    pub fn save(&self, filename: &PathBuf) {
        let content = serde_json::to_string(&self).expect("Unable to convert waypoints to json.");
        fs::write(filename, &content).expect("Could not write to file...");
    }
}
