#![warn(
    rustdoc::broken_intra_doc_links,
    unreachable_pub,
    unreachable_patterns,
    unused,
    unused_qualifications,
    dead_code,
    while_true,
    unconditional_panic,
    clippy::all
)]

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::sync::LazyLock;

static SAVE_FILE: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut waypoints_file = home::home_dir().unwrap();
    waypoints_file.push(".config/adg/waypoints.json");
    waypoints_file
});

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    subcommand: Option<Commands>,
    /// If you don't specify a subcommand, you will need to specify a path to goto.
    path: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a site or locale to your store.
    Add {
        /// The name for the to-be-stored site or locale.
        alias: String,
        /// The path to the site or locale. If this path exists and points to a directory, a site
        /// will be stored. Otherwise, a locale will be stored.
        path: PathBuf,
    },
    /// Drop a site or locale from your store.
    Drop {
        /// The name of the site or locale that will be dropped.
        alias: String,
    },
    /// Go to a directory that is hydrated with sites and locales that you have stored .
    Goto { path: PathBuf },
    /// List all of the sites and locales you have stored.
    List,
}

#[derive(Serialize, Deserialize)]
pub struct Waypoints {
    locales: HashMap<String, String>,
    sites: HashMap<String, String>,
}

impl Waypoints {
    pub(crate) fn new() -> Self {
        let file_data = read_to_string(&*SAVE_FILE).expect("Waypoints file was not found...");
        serde_json::from_str(&file_data).expect("Waypoints file was malformed...")
    }

    fn get<'a>(&'a self, loc: &'a str) -> &'a str {
        self.locales
            .get(loc)
            .or_else(|| self.sites.get(loc))
            .map(String::as_str)
            .unwrap_or(loc)
    }

    pub fn add(&mut self, alias: String, path: PathBuf) {
        if path.exists() {
            self.locales
                .insert(alias, path.canonicalize().unwrap().to_str().unwrap().into());
        } else {
            self.sites.insert(alias, path.to_str().unwrap().into());
        }
    }

    pub fn drop(&mut self, loc: String) {
        self.locales.remove(&loc);
        self.sites.remove(&loc);
    }

    pub fn goto(&self, loc: PathBuf) -> PathBuf {
        loc.iter().fold(PathBuf::default(), |mut digest, l| {
            digest.push(l);
            if !digest.is_dir() {
                digest.pop();
                digest.push(self.get(l.to_str().unwrap()));
            }
            digest
        })
    }

    pub fn list(&self) {
        println!("Locales:");
        self.locales
            .iter()
            .for_each(|(key, val)| println!("\t{} : {}", key, val));
        println!("\nSites:");
        self.locales
            .iter()
            .for_each(|(key, val)| println!("\t{} : {}", key, val))
    }

    pub fn save(&self) {
        let content = serde_json::to_string(&self).expect("Unable to convert waypoints to json.");
        std::fs::write(&*SAVE_FILE, &content).expect("Could not write to file...");
    }
}

fn main() {
    let Args { subcommand, path } = Args::parse();

    let mut waypoints = Waypoints::new();

    match (subcommand, path) {
        (Some(Commands::Add { alias, path }), _) => waypoints.add(alias, path),
        (Some(Commands::Drop { alias }), _) => waypoints.drop(alias),
        (Some(Commands::List), _) => waypoints.list(),
        (Some(Commands::Goto { path }), _) | (None, Some(path)) => {
            println!("{}", waypoints.goto(path).to_str().unwrap())
        }
        (None, None) => {
            eprintln!("Neither command nor path was found");
            std::process::exit(1)
        }
    }
    waypoints.save();
}
