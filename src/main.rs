use iced::Size;
use iced::window::Settings;

use lirrel::Lirrel;

fn main() -> iced::Result {
    iced::application(Lirrel::boot, Lirrel::update, Lirrel::view)
        .title("Lirrel")
        .window(Settings {
            size: Size::new(300., 200.),
            resizable: false,
            ..Default::default()
        })
        .run()
}
