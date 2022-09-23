use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;

const APP_ID: &str = "org.tp.techniktobi.luixpreview";

fn
main
() 
{
	
	// Create a new application
	// This uses the builder pattern,
	// which is supported by many gtk-rs objects
	let app = Application::builder()
		.application_id(APP_ID)
		.build();

	// Connect the build_ui function to the "activate" signal of "app"
	app.connect_activate(build_ui);

	// Run the application
	app.run();
}

fn
build_ui
(
	app: &Application
)
{
	// Create a new window & set its title
	let window = ApplicationWindow::builder()
		.application(app)
		.title("LuixPreview")
		.build();
	
	// Present the window
	window.present();
}
