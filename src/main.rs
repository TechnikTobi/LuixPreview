use gtk::ffi::GTK_RESPONSE_CANCEL;
use gtk::prelude::*;
use gtk::AboutDialog;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Button;

use std::path::PathBuf;
use gtk::FileChooserDialog;
use gtk::FileFilter;

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

    let open_dialog = FileChooserDialog::builder()
        .application(app)
        .decorated(true)
        .deletable(true)
        .focus_visible(true)
        .visible(false)
        .title("Testing the File Dialog")
        .build();

	// Create a new window & set its title
	let window = ApplicationWindow::builder()
		.application(app)
		.title("LuixPreview")
		.child(&about_dialog)
		.child(&button)
        .child(&open_dialog)
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


fn 
show_open_dialog
(
    parent: &ApplicationWindow
)
-> Option<PathBuf>
{
    // let mut file = None;
    let file_filter = FileFilter::new();
    file_filter.set_name(Some("Image file"));

    let dialog = FileChooserDialog::new(
        Some("Select file"),
        Some(parent),
        gtk::FileChooserAction::Open,
        &[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Accept", gtk::ResponseType::Accept)
        ]
    );
    dialog.add_filter(&file_filter);

    

    /* 
    dialog.connect_response(clone!(@weak text_view => move |file_chooser, response| {
        if response == gtk::ResponseType::Ok {
            let filename = file_chooser.get_filename().expect("Couldn't get filename");
            let file = File::open(&filename).expect("Couldn't open file");

            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            let _ = reader.read_to_string(&mut contents);

            text_view
                .get_buffer()
                .expect("Couldn't get window")
                .set_text(&contents);
        }
        file_chooser.close();
    }));
    */

    return None;
}