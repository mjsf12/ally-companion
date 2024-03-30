use std::process::Command;
use std::fs::read_to_string;
// use std::fs::File;
// use std::io::Write; 
pub struct SystemInterator {
        pub ryzenadj_output: String,
        pub maxtdp: i8,
        pub mintdp: i8,
        pub ryzenadj_command: String,
        pub battery: i8,
        pub root_path: String,
    }
    impl SystemInterator {
        pub fn inicialize(&mut self) {
            self.get_ryzenadj_output();
            self.get_tdp();
            self.get_battery();
        }
        pub fn set_tdp_ryzen(&self) {
            let tdp_max = (self.maxtdp as u16)*1000;
            let tdp_min = (self.mintdp as u16)*1000;
            // sudo $ryzenadj_path --stapm-limit=$fast --fast-limit=$fast --slow-limit=$fast --tctl-temp=90
            // f"{sys_ryzenadj_path} -a {stapm_limit} -b {fast_minit} -c {slow_limit} -f {tctl_temp}"
            
            let output = Command::new(self.root_path.as_str())
                .arg(self.ryzenadj_command.as_str())
                .arg("-a").arg(tdp_max.to_string())
                .arg("-b").arg(tdp_min.to_string())
                .arg("-c").arg(tdp_max.to_string())
                .arg("-f").arg("90")
                .output();
            match output {
                Ok(output) => output,
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                }
            };
                
        
        }
        pub fn get_ryzenadj_output(&mut self)  {
            let ryzenadj_output = Command::new(self.root_path.as_str())
                .arg(self.ryzenadj_command.as_str())
                .arg("-i").output();
            let ryzenadj_output = match ryzenadj_output {
                Ok(output) => output,
                Err(e) => {
                    println!("Error: {}", e);
                    return ();
                }
            };
              let ryzenadj_output = String::from_utf8(ryzenadj_output.stdout);
            let ryzenadj_output = match ryzenadj_output {
                Ok(output) => output,
                Err(e) => {
                    println!("Error: {}", e);
                    return ();
                }
            };
            println!("Ryzenadj output: {}", ryzenadj_output);
            self.ryzenadj_output = ryzenadj_output;
        }
        pub fn get_tdp (&mut self) {
            let mut maxtdp: f64 = 0.0f64;
            let mut mintdp: f64 = 0.0f64;
            let ryzenadj_output = self.ryzenadj_output.split("\n");
            println!("Ryzenadj output: {:?}", ryzenadj_output);
            for line in ryzenadj_output {
                if line.contains("stapm-limit") {
                    let tdp = line.split("|").collect::<Vec<&str>>()[2].trim();
                    maxtdp = match tdp.parse::<f64>() {
                        Ok(output) => output,
                        Err(e) => {
                            println!("Error: {}", e);
                            0f64
     
                        }
                    };
                }
                if line.contains("fast-limit") {
                    let tdp = line.split("|").collect::<Vec<&str>>()[2].trim();
                    // println!("TDP: {}", tdp);
                    mintdp = match tdp.parse::<f64>() {
                        Ok(output) => output,
                        Err(e) => {
                            println!("Error: {}", e);
                            0f64
                        }
                    };
                    // println!("Mintdp: {}", mintdp);
                }
            }
            println!("Maxtdp: {}, Mintdp: {}", maxtdp,mintdp);
            self.maxtdp = maxtdp.round() as i8;
            self.mintdp = mintdp.round() as i8;
        }
        pub fn get_battery(&mut self) {
            let file = "/sys/class/power_supply/BAT0/charge_control_end_threshold";
            let result_string_inside = read_to_string(file);
            let string_inside = match result_string_inside {
                Ok(text) => text,
                Err(e) => {
                    println!("Error: {}", e);
                    return ();
                }
            };
            let battery = string_inside.trim().parse::<i8>();
            let porcentage_battery = match battery {
                Ok(battery) => battery,
                Err(e) => {
                    println!("Error: {}", e);
                    return ();
                }
            };       
            self.battery = porcentage_battery;
        }
        // pub fn set_battery(&self) {
        //     let battery = self.battery;
        //     let file = "/sys/class/power_supply/BAT0/charge_control_end_threshold";
        //     let max = battery.to_string();
        //     let f = File::create(file);
        //     let mut f = match f {
        //         Ok(file) => file,
        //         Err(e) => {
        //             println!("Error: {}", e);
        //             return;
        //         }
        //     };
            
        //     match f.write_all(max.as_bytes()){
        //         Ok(_) => (),
        //         Err(e) => {
        //             println!("Error: {}", e);
        //             return;
        //         }
        //     };
        //     }
        pub fn  set_battery(&self) {
            Command::new(self.root_path.as_str())
                .arg("pkexec")
                .arg("bash")
                .arg("-c")
                .arg(format!("echo {} > /sys/class/power_supply/BAT0/charge_control_end_threshold",self.battery))
                .output().unwrap();
        }
    }
