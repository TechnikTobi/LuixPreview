use gtk::prelude::*;
use gtk::AboutDialog;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Button;

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

	// Create the about dialog
	let about_dialog = AboutDialog::builder()
		.version("1.2.3")
		.authors(vec!["Tobias Prisching".to_string()])
		.build();

	// Create a button
	let button = Button::builder()
		.label("Press me!")
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();

	button.connect_clicked(button_clicked);

	// Create a new window & set its title
	let window = ApplicationWindow::builder()
		.application(app)
		.title("LuixPreview")
		.child(&about_dialog)
		.child(&button)
		.build();
	
	// Present the window
	window.present();
}

fn
button_clicked
(
	button: &Button
)
{
	button.set_label("Hello World!");
}
