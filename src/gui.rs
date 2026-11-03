use iced::alignment::Vertical;
use iced::font::Weight;
use iced::widget::{
    Container, button, column, container, row, rule::horizontal as horizontal_rule, space, text,
};
use iced::{Background, Border, Color, Font, Length, Padding};

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
        button(text("⚙️").size(20).align_y(Vertical::Center))
            .style(|_, _| button::Style {
                text_color: WHITE,
                ..Default::default()
            })
            .on_press(Message::ToMenu),
    ]
    .width(Length::Fill)
    .height(20)
    .align_y(Vertical::Center);

    let timer = container(simple_text!(time_text, 68, BLACK, WHITE))
        .center_x(Length::Fill)
        .padding(Padding::from(32));

    let primary_controls = row![
        make_default_button(("⏮", CYAN), "Voltar", Message::Back),
        make_button(
            ("▶", WHITE),
            "Play",
            1.,
            (PLAY, PLAY_HOVER),
            (Length::Fill, Length::Fixed(45.)),
            Message::Play
        ),
        make_default_button(("⏭", CYAN), "Pular", Message::Skip),
    ]
    .spacing(12)
    .width(Length::Fill)
    .height(45);

    let secondary_controls = row![
        make_default_button(("🔁", CYAN), "Zerar", Message::Zero),
        make_default_button(("❌", RED), "Resetar", Message::Reset),
    ]
    .spacing(12)
    .width(Length::Fill)
    .height(45);

    container(column![
        header, //
        timer,
        primary_controls,
        space::vertical().height(12.),
        secondary_controls,
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(Padding::from(32))
    .style(|_| container::Style {
        background: Some(Background::Color(PANEL)),
        border: Border::default().rounded(16),
        text_color: Some(WHITE),
        ..Default::default()
    })
}

pub(crate) fn view_menu<'a>() -> Container<'a, Message> {
    let header = row![
        simple_text!("Configurações", 16, Font::DEFAULT, MUTED),
        space::horizontal(),
        button(text("❌").size(20).align_y(Vertical::Center))
            .style(|_, _| button::Style {
                text_color: WHITE,
                ..Default::default()
            })
            .on_press(Message::ToMain)
    ]
    .width(Length::Fill)
    .height(20)
    .align_y(Vertical::Top);

    let file_line = row![
        column![
            make_button(
                ("", MUTED),
                "salvar",
                0.75,
                (BUTTON, BUTTON_HOVER),
                (Length::Fixed(100.), Length::Fixed(45. / 2.)),
                Message::SaveFile
            ),
            make_button(
                ("", MUTED),
                "salvar como",
                0.75,
                (BUTTON, BUTTON_HOVER),
                (Length::Fixed(100.), Length::Fixed(45. / 2.)),
                Message::SaveFileAs
            ),
        ],
        space::horizontal(),
        simple_text!("arquivo aberto", 32, SEMIBOLD, WHITE),
        space::horizontal(),
        make_button(
            ("", MUTED),
            "abrir",
            1.,
            (BUTTON, BUTTON_HOVER),
            (Length::Fixed(100.), Length::Fixed(45.)),
            Message::OpenFile
        ),
    ]
    .spacing(8)
    .padding(Padding::from([8, 0]))
    .width(Length::Fill);

    container(column![header, file_line, horizontal_rule(2)])
        .width(Length::Fill)
        .height(Length::Fill)
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
    make_button(
        glyph,
        label,
        1.,
        (BUTTON, BUTTON_HOVER),
        (Length::Fill, Length::Fixed(45.)),
        message,
    )
}

fn make_button(
    glyph: (&'static str, Color),
    label: &'static str,
    font_scale: f32,
    bg_colors: (Color, Color),
    size: (Length, Length),
    message: Message,
) -> button::Button<'static, Message> {
    button(
        container(
            row![
                simple_text!(glyph.0, 18. * font_scale, SEMIBOLD, glyph.1),
                simple_text!(label, 16. * font_scale, SEMIBOLD, WHITE)
            ]
            .spacing(7),
        )
        .center_x(Length::Fill)
        .align_y(Vertical::Center),
    )
    .width(size.0)
    .height(size.1)
    .style(move |_, status| {
        let color = if matches!(status, button::Status::Hovered) {
            bg_colors.0
        } else {
            bg_colors.1
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
