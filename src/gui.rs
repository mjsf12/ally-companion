use crate::systeminterator;
use std::sync::Mutex;
use std::sync::Arc;


slint::include_modules!();

// use slint::quit_event_loop;
pub struct Sui {
    ui: AppWindow,
}

impl Sui {
    pub fn new() -> Arc<Sui> {
        let sui = Sui {
            ui: AppWindow::new().unwrap(),
        };
        Arc::new(sui)
    }
    pub fn inicialize(&self,system_interatorrc: Arc<Mutex<systeminterator::SystemInterator>>) {
        let system_interator = system_interatorrc.lock().unwrap();
        let battery = system_interator.battery as i8;
        let ryzenadj_output = system_interator.ryzenadj_output.clone();
        let tdp = system_interator.maxtdp as i8;
        self.set_battery(battery);
        self.set_ryzenadjout(ryzenadj_output);
        self.set_tdp(tdp);

    }
    pub fn inicialize_callback(&self,system_interatorrc: Arc<Mutex<systeminterator::SystemInterator>>,ui_l: Arc<Sui>) {
        let system_interator = Arc::clone(&system_interatorrc);
        let ui_l_l = Arc::clone(&ui_l);
        self.on_request_change_bat(system_interator, ui_l_l);
        let system_interator = Arc::clone(&system_interatorrc);
        let ui_l_l = Arc::clone(&ui_l);
        self.on_request_change_tdp(system_interator, ui_l_l);
        let system_interator = Arc::clone(&system_interatorrc);
        let ui_l_l = Arc::clone(&ui_l);
        self.updateall(system_interator, ui_l_l);

        // self.ui.on_request_ryzenadj(move || {
        //     let mut system_interator_b = system_interator.lock().unwrap();
        //     system_interator_b.get_ryzenadj_output();
        //     system_interator_b.get_tdp();
        //     let tdp = system_interator_b.maxtdp as i8;
        //     let ryzenadj_output = system_interator_b.ryzenadj_output.clone();
        //     ui_l_l.set_tdp(tdp.into());
        //     ui_l_l.set_ryzenadjout(ryzenadj_output.into());
        // });
        // let invoke = invoke_from_event_loop(move || {
        //     // sleep
        //     println!("invoke_from_event_loop");
            
        // });
        // invoke.expect("invoke_from_event_loop failed");


    }
    pub fn set_battery(&self, battery: i8) {
        self.ui.set_battery(battery.into());
    }
    pub fn set_ryzenadjout(&self, ryzenadj_output: String) {
        self.ui.set_ryzenadjout(ryzenadj_output.into());
    }
    pub fn set_tdp(&self, tdp: i8) {
        self.ui.set_tdp(tdp.into());
    }
    pub fn run(&self) ->  Result<(), slint::PlatformError> {
        self.ui.run()

    }
    pub fn updateall(&self,system_interatorrc: Arc<Mutex<systeminterator::SystemInterator>>,ui_l: Arc<Sui>) {
        let system_interator = Arc::clone(&system_interatorrc);
        let ui_l_l = Arc::clone(&ui_l); 
        self.ui.on_request_ryzenadj(move || {
            let mut system_interator_b = system_interator.lock().unwrap();
            system_interator_b.get_ryzenadj_output();
            system_interator_b.get_tdp();
            system_interator_b.get_battery();
            let tdp = system_interator_b.maxtdp as i8;
            let ryzenadj_output = system_interator_b.ryzenadj_output.clone();
            let battery = system_interator_b.battery as i8;
            ui_l_l.set_tdp(tdp.into());
            ui_l_l.set_ryzenadjout(ryzenadj_output.into());
            ui_l_l.set_battery(battery.into());
            // ui_l_l.
        });

    }
    pub fn on_request_change_tdp(&self, system_interatorrc: Arc<Mutex<systeminterator::SystemInterator>>,ui_l: Arc<Sui>) {    
        self.ui.on_request_change_tdp(move |tdp| {
   
            let mut system_interator_b = system_interatorrc.lock().unwrap();
            system_interator_b.maxtdp = tdp as i8;
            system_interator_b.mintdp = tdp as i8;
            system_interator_b.set_tdp_ryzen();
            // thread::sleep(Duration::from_millis(500));
            system_interator_b.get_ryzenadj_output();
            system_interator_b.get_tdp();
            let tdp = system_interator_b.maxtdp as i8;
            let ryzenadj_output = system_interator_b.ryzenadj_output.clone();
            ui_l.set_tdp(tdp.into());
            ui_l.set_ryzenadjout(ryzenadj_output.into());
        })
    }

    pub fn on_request_change_bat(&self, system_interatorrc: Arc<Mutex<systeminterator::SystemInterator>>,ui_l: Arc<Sui>) {
        self.ui.on_request_change_bat(move |battery| {
            let mut system_interator_b = system_interatorrc.lock().unwrap();
            system_interator_b.battery = battery as i8;
            system_interator_b.set_battery();
            // thread::sleep(Duration::from_millis(500));
            system_interator_b.get_battery();
            let battery = system_interator_b.battery as i8;
            ui_l.set_battery(battery.into());
            });
}
}