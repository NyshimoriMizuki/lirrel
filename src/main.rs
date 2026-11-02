use lirrel::Lirrel;

fn main() -> iced::Result {
    iced::application(Lirrel::boot, Lirrel::update, Lirrel::view)
        .title("Lirrel")
        .font(include_bytes!("../fonts/Roboto-VariableFont.ttf").as_slice())
        .default_font(lirrel::SEMIBOLD)
        .window(Lirrel::window_settings())
        .theme(Lirrel::theme())
        .run()
}
