use png::Decoder;
use tray_item::{IconSource, TrayItem};
// use tray_item::
use std::io::Cursor;
use std::process::exit;
use crate::systeminterator;
use std::sync::Mutex;
use std::sync::Arc;
// slint::include_modules!();
// use slint::include_modules!();
pub struct TrayWarper {
    tray: TrayItem,
}
impl TrayWarper{
   pub fn new() -> TrayWarper {
        let cursor = Cursor::new(include_bytes!("../resources/Armoury_Crate.png"));
        let decoder = Decoder::new(cursor);
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();

        let icon = IconSource::Data {
            data: buf,
            height: 32,
            width: 32,
        };

        let tray = TrayItem::new("Tray Example", icon).unwrap();
        TrayWarper {
            tray,
        }
    }
    pub fn inicialize(&mut self,systeminterator:Arc<Mutex<systeminterator::SystemInterator>>,tx:std::sync::mpsc::Sender<String>){
        self.tray.add_menu_item("Open GUI", move || {
           let _ = tx.send("open".to_string()).unwrap();
        }).unwrap();
        self.tray.add_label("TDP:").unwrap();
        let tdp_range = vec![5, 10, 15, 20, 25, 30];
        for tdp in tdp_range {
            let systeminterator_c = Arc::clone(&systeminterator);
            // let ui_l_l = Arc::clone(&ui_l);
            self.tray.add_menu_item(&format!("{}W", tdp.to_string()), move || {
                let mut system_interator = systeminterator_c.lock().unwrap();
                system_interator.maxtdp = tdp;
                system_interator.mintdp = tdp;
                system_interator.set_tdp_ryzen();          
                println!("TDP: {}", tdp);                
            })
            .unwrap();
        }
        self.tray.add_label("Battery:").unwrap();
        let battery_range: Vec<i8> = vec![50, 60, 70, 80, 90, 100];
        for battery in battery_range {
            let systeminterator_c = Arc::clone(&systeminterator);
            self.tray.add_menu_item(&format!("{}%", battery.to_string()), move || {
                println!("Battery: {}", battery);
                let mut system_interator = systeminterator_c.lock().unwrap();
                system_interator.battery = battery;
                system_interator.set_battery();
                
            })
            .unwrap();
        }
    
        self.tray.add_menu_item("Quit", move || {
            exit(0);
        })
        .unwrap();
    }
}