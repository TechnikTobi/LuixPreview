use iced::Settings;
use iced::pure::Sandbox;
use iced::pure::widget::{Button, Text, Column, Container};

const APP_ID: &str = "org.tp.techniktobi.luixpreview";

struct 
Counter
{
	count: i64
}

enum
ECounterMessage
{
	Increment,
	Decrement
}

impl Sandbox for Counter
{
	// A type alias to the Message type we will use
    type Message = ();

	// Initialises the state of the application
    fn new() -> Self {
        todo!()
    }

	// Sets the title of the application
    fn title(&self) -> String {
        todo!()
    }

	// Receives messages of the above defined type and changes the applications state
    fn update(&mut self, message: Self::Message) {
        todo!()
    }

	// Draws the GUI
	// Returns an iced element that occupies the view
    fn view(&self) -> iced::pure::Element<'_, Self::Message> {
        todo!()
    }
}

fn
main
() 
{

	println!("Hello World!");

}	
