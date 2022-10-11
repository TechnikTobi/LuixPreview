use iced::Settings;
use iced::pure::Sandbox;
use iced::pure::widget::{Button, Text, Column, Container, Image};

// const APP_ID: &str = "org.tp.techniktobi.luixpreview";

struct 
Counter
{
	count: i64
}

#[derive(Debug, Clone, Copy)]
enum
ECounterMessage
{
	Increment,
	Decrement
}

impl Sandbox for Counter
{
	// A type alias to the Message type we will use
    type Message = ECounterMessage;

	// Initialises the state of the application
    fn new() -> Self {
        Counter 
        { 
            count: 0 
        }
    }

	// Sets the title of the application
    fn title(&self) -> String {
        String::from("A Counter Application written in Rust using Iced")
    }

	// Receives messages of the above defined type and changes the applications state
    fn update(&mut self, message: Self::Message) {
        match message
        {
            ECounterMessage::Increment => self.count += 1,
            ECounterMessage::Decrement => self.count -= 1,
        }
    }

	// Draws the GUI
	// Returns an iced element that occupies the view
    fn view(&self) -> iced::pure::Element<'_, Self::Message> {

        // A label for displaying the currrent count
        let label = Text::new(format!("Count: {}", self.count));

        // Buttons for messages
        let button_increment = Button::new("Increment").on_press(ECounterMessage::Increment);
        let button_decrement = Button::new("Decrement").on_press(ECounterMessage::Decrement);

        let image = Image::new("image.png")
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);

        let column = Column::new()
            .push(image)
            .push(label)
            .push(button_increment)
            .push(button_decrement);

        return Container::new(column)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::WHITE
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn should_exit(&self) -> bool {
        false
    }

    fn run(settings: Settings<()>) -> Result<(), iced::Error>
        where
    Self: 'static + Sized,
    {
        <Self as iced::pure::Application>::run(settings)
    }

}

fn
main
() 
{
	Counter::run(Settings::default()).expect("Could not start application");
}	
