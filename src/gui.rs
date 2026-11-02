use iced::alignment::Vertical;
use iced::font::Weight;
use iced::widget::{Container, button, column, container, row, space, text};
use iced::{Background, Border, Color, Fill, Font, Padding};

use crate::storage::Stage;
use crate::{Message, ProgressTracker};

macro_rules! simple_text {
    ($content:expr, $size:expr, $font:expr, $color:expr) => {
        text($content).size($size).font($font).color($color)
    };
}

macro_rules! const_hexcolor {
    ($hex:expr => $name:ident) => {
        const $name: Color = Color::from_rgb8(
            (($hex >> 16) & 0xFF) as u8,
            (($hex >> 8) & 0xFF) as u8,
            ($hex & 0xFF) as u8,
        );
    };
}
macro_rules! font {
    ($font:expr, $weight:expr) => {
        Font {
            family: iced::font::Family::Name($font),
            weight: $weight,
            ..Font::DEFAULT
        }
    };
}

const_hexcolor!(0x1F2B3E => PANEL );
const_hexcolor!(0x344358 => BUTTON );
const_hexcolor!(0x3E5067 => BUTTON_HOVER );
const_hexcolor!(0x008BCC => PLAY );
const_hexcolor!(0x0099DE => PLAY_HOVER );
const_hexcolor!(0xF5F7FA => WHITE );
const_hexcolor!(0xA6B8CF => MUTED );
const_hexcolor!(0x23B5F5 => CYAN );
const_hexcolor!(0xFF456F => RED );

pub const WINDOW_WIDTH: f32 = 500.0;
pub const WINDOW_HEIGHT: f32 = 350.0;

const BLACK: Font = font!("Roboto Black", Weight::Black);
const BOLD: Font = font!("Roboto Bold", Weight::Bold);
pub const SEMIBOLD: Font = font!("Roboto SemiBold", Weight::Semibold);

pub(crate) fn view_main<'a>(
    time_text: &'a str,
    stage: &'a Stage,
    indicator: &'a ProgressTracker,
) -> Container<'a, Message> {
    let header = row![
        simple_text!(indicator.show(), 16, Font::DEFAULT, MUTED),
        space::horizontal(),
        simple_text!(&stage.name, 24, BOLD, CYAN),
        space::horizontal(),
        button(text("⚙").size(20).align_y(Vertical::Center))
            .padding(0)
            .style(|_, _| button::Style {
                background: None,
                text_color: WHITE,
                border: Border::default(),
                ..Default::default()
            })
            .on_press(Message::ToMenu),
    ]
    .width(Fill)
    .height(24)
    .align_y(Vertical::Center);

    let timer = container(simple_text!(time_text, 68, BLACK, WHITE))
        .center_x(Fill)
        .padding(Padding::from(32));

    let primary_controls = row![
        make_default_button(("⏮", CYAN), "Voltar", Message::Back),
        make_button(("▶", WHITE), "Play", PLAY, PLAY_HOVER, Message::Play),
        make_default_button(("⏭", CYAN), "Pular", Message::Skip),
    ]
    .spacing(12)
    .width(Fill)
    .height(45);

    let secondary_controls = row![
        make_default_button(("🔁", CYAN), "Zerar", Message::Zero),
        make_default_button(("❌", RED), "Resetar", Message::Reset),
    ]
    .spacing(12)
    .width(Fill)
    .height(45);

    container(column![
        header, //
        timer,
        primary_controls,
        space::vertical().height(12.),
        secondary_controls,
    ])
    .width(Fill)
    .height(Fill)
    .padding(Padding::from(32))
    .style(|_| container::Style {
        background: Some(Background::Color(PANEL)),
        border: Border::default().rounded(16),
        text_color: Some(WHITE),
        ..Default::default()
    })
}

pub(crate) fn view_menu<'a>() -> Container<'a, Message> {
    let back_button = make_default_button(("◀", MUTED), "voltar", Message::ToMain);

    container(column![back_button])
        .width(Fill)
        .height(Fill)
        .padding(Padding::from(32))
        .style(|_| container::Style {
            background: Some(Background::Color(PANEL)),
            border: Border::default().rounded(16),
            text_color: Some(WHITE),
            ..Default::default()
        })
}

fn make_default_button(
    glyph: (&'static str, Color),
    label: &'static str,
    message: Message,
) -> button::Button<'static, Message> {
    make_button(glyph, label, BUTTON, BUTTON_HOVER, message)
}

fn make_button(
    glyph: (&'static str, Color),
    label: &'static str,
    background: Color,
    hover_background: Color,
    message: Message,
) -> button::Button<'static, Message> {
    button(
        container(
            row![
                simple_text!(glyph.0, 18, SEMIBOLD, glyph.1),
                simple_text!(label, 16, SEMIBOLD, WHITE)
            ]
            .spacing(7),
        )
        .center_x(Fill)
        .align_y(Vertical::Center),
    )
    .width(Fill)
    .height(45)
    .style(move |_, status| {
        let color = if matches!(status, button::Status::Hovered) {
            hover_background
        } else {
            background
        };

        button::Style {
            background: Some(Background::Color(color)),
            text_color: WHITE,
            border: Border {
                radius: 8.0.into(),
                ..Border::default()
            },
            ..Default::default()
        }
    })
    .on_press(message)
}

pub fn theme() -> iced::theme::Theme {
    iced::theme::Theme::Dark
}
