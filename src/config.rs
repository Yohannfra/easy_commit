use serde::Deserialize;
use std::{fs, path};

use path::PathBuf;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(skip_deserializing)]
    fp: PathBuf,

    pub conf: Option<Conf>,
}

#[derive(Deserialize, Debug)]
pub struct Conf {
    scopes: Option<Vec<String>>,
    types: Option<Vec<String>>,
}

impl Config {
    fn parse_file(fp: path::PathBuf) -> Result<Config, String> {
        let contents = match fs::read_to_string(&fp) {
            Ok(c) => c,
            Err(_) => {
                return Err(format!("Could not read file: '{}'", fp.to_string_lossy()));
            }
        };

        let mut conf: Config = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(e) => {
                return Err(format!(
                    "Unable to load data from: {:?}: {}",
                    fp.to_string_lossy(),
                    e
                ))
            }
        };
        conf.fp = fp;
        Ok(conf)
    }

    pub fn new(fp: std::path::PathBuf) -> Result<Config, String> {
        println!("Loading {}", fp.to_string_lossy());

        let conf = Self::parse_file(fp)?;

        println!("{:?}", conf);
        println!("{:?}", conf.conf);
        //pr.verify();

        Ok(conf)
    }
}
