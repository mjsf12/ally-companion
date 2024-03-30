// use std::path::Path;
// use std::fs::read_to_string;
// use std::fs::File;
// use std::io::Write;
// use serde_json::from_str;
pub struct Config {
    pub maxtdp: i8,
    pub mintdp: i8,
    pub battery: i8,
    pub ryzenadj_path: String,
    pub root_path: String,
}
impl Config {
    pub fn new() -> Config {
        // let config: Config;
        // if Path::new("/home/$USER/.config/ally-companion/config.json").exists() {
        //     let filereader = read_to_string("/home/$USER/.config/ally-companion/config.json").expect("Unable to read file");
            
        //     config = from_str(&filereader).unwrap();
        // }
        // else {
            // if not exist create config
        // let config =
         Config {
                maxtdp: 15,
                mintdp: 15,
                battery: 100,
                ryzenadj_path: String::from("/usr/bin/ryzenadj"),
                root_path: String::from("/usr/bin/pkexec")
            }
            // let serialized = serde_json::to_string(&config).unwrap();
            // let mut file = File::create("/home/$USER/.config/ally-companion/config.json").expect("Unable to create file");
            // file.write_all(serialized.as_bytes()).expect("Unable to write data");
        }
        // config    
    // pub fn save(&self) {
    //     let serialized = serde_json::to_string(&self).unwrap();
    //     let mut file = File::create("/home/$USER/.config/ally-companion/config.json").expect("Unable to create file");
    //     file.write_all(serialized.as_bytes()).expect("Unable to write data");
    // }
}