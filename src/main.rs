mod luix_window;

use gtk::prelude::*;
use luix_window::LuixWindow;

const APP_ID: &str = "org.tp.luix.preview";

fn
main
() 
{
	// Luix::run(Settings::default()).expect("Could not start application");
    let application = gtk::Application::new(
        Some(APP_ID),
        Default::default(),
    );

    application.connect_activate(|app| {
        let window = LuixWindow::new(app);
        window.show();
    });

    application.run();
}	
