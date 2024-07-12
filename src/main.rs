use std::fs::File;
use std::io::Read;
use serde::Deserialize;

struct Replacement {
    current: String,
    replacement: String,
}

struct Config {
    replacements: Vec<Replacement>,
    //add more later
}

fn main() {
    let mut camouflaged : File = File::open("test.camo").expect("no file");
    let mut camo_contents = String::new();
    file.read_to_string(&mut contents).expect("no read");
    let mut config_contents = String::new();
    let mut config : Config = toml::from_str(File::open("camo.toml")
        .expect("no config")
        .read_to_string(&mut config_contents));
}
