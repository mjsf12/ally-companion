use std::sync::Arc;
use std::sync::Mutex;
mod gui;
mod systeminterator;
mod tray;
mod config;
use std::sync::mpsc;


fn main() {
    let config = config::Config::new();
    let mut system_interator = systeminterator::SystemInterator {
        ryzenadj_output: String::from(""),
        maxtdp: config.maxtdp,
        mintdp: config.mintdp,
        ryzenadj_command: config.ryzenadj_path,
        battery: config.battery,
        root_path: config.root_path,
    };
    let (tx, rx) = mpsc::channel();
    let ui = gui::Sui::new();
    system_interator.inicialize();
    let system_interator = Arc::new(Mutex::new(system_interator));
    ui.inicialize(Arc::clone(&system_interator));
    ui.inicialize_callback(Arc::clone(&system_interator), Arc::clone(&ui),);
    let mut tray = tray::TrayWarper::new();
    tray.inicialize(Arc::clone(&system_interator),tx);
    let _ = ui.run();
    
    loop {
        match rx.try_recv() {
            Ok(_) => {
                let _ = ui.run();
            }
            Err(_) => {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
    
}
