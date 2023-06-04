let native_options = eframe::NativeOptions::default();
    eframe::run_native("Clock", native_options, Box::new(|_cc| Box::<Time>::default()))
    

    impl Default for Time {
        fn default() -> Self {
          Self { now: String::from("00::00::00") }
      }
  }
  impl eframe::App for Time {
      fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
         let time = get(); 
         egui::CentralPanel::default().show(ctx, |ui|{
              ui.label(time);
         });
      }
  }