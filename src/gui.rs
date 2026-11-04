use iced::alignment::Vertical;
use iced::font::Weight;
use iced::widget::{
    Container, Text, button, column, container, rich_text, row,
    rule::horizontal as horizontal_rule, space, span, text,
};
use iced::{Background, Border, Color, Font, Length, Padding};

use std::sync::LazyLock;

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

const CHAR_SIZE: u32 = 16;
const SMALL_CHAR_SIZE: u32 = CHAR_SIZE * 3 / 4;
const ICON_SIZE: u32 = CHAR_SIZE * 4 / 3;

pub static BLANK_TEXT: LazyLock<UIText> = LazyLock::new(UIText::default);

pub(crate) fn view_main<'a>(
    time_text: &'a str,
    stage: &'a Stage,
    indicator: &'a ProgressTracker,
) -> Container<'a, Message> {
    let header = row![
        simple_text!(indicator.show(), CHAR_SIZE, Font::DEFAULT, MUTED),
        space::horizontal(),
        simple_text!(&stage.name, CHAR_SIZE * 3 / 2, BOLD, CYAN),
        space::horizontal(),
        UIButton::new(Message::ToMenu)
            .glyph_sized("⚙️", WHITE, ICON_SIZE)
            .create(),
    ]
    .height(20)
    .width(Length::Fill)
    .align_y(Vertical::Center);

    let timer = container(simple_text!(time_text, CHAR_SIZE * 4, BLACK, WHITE))
        .center_x(Length::Fill)
        .padding(Padding::from((CHAR_SIZE * 2) as f32));

    let primary_controls = row![
        make_default_button(("⏮", CYAN), "Voltar", Message::Back),
        UIButton::new(Message::Play)
            .glyph("▶", WHITE)
            .text("Play")
            .base(PLAY, PLAY_HOVER, 8.into())
            .create(),
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
        space::vertical().height(SMALL_CHAR_SIZE as f32),
        secondary_controls,
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(Padding::from(32))
    .style(|_| container::Style {
        background: Some(Background::Color(PANEL)),
        border: Border::default().rounded(8),
        text_color: Some(WHITE),
        ..Default::default()
    })
}

pub(crate) fn view_menu<'a>() -> Container<'a, Message> {
    let header = row![
        UIText::new("Configurações", CHAR_SIZE)
            .font(Font::DEFAULT)
            .color(MUTED)
            .create(),
        space::horizontal(),
        UIButton::new(Message::ToMain)
            .glyph_sized("❌", WHITE, ICON_SIZE)
            .create(),
    ]
    .width(Length::Fill)
    .height(20)
    .align_y(Vertical::Top);

    let file_line = row![
        column![
            UIButton::new(Message::SaveFile)
                .text_sized("salvar", SMALL_CHAR_SIZE)
                .size(Size2D::new(100., 45. / 2.))
                .create(),
            UIButton::new(Message::SaveFileAs)
                .text_sized("salvar como", SMALL_CHAR_SIZE)
                .size(Size2D::new(100., 45. / 2.))
                .create(),
        ]
        .spacing(1),
        space::horizontal(),
        UIText::new("arquivo aberto", CHAR_SIZE * 2).create(),
        space::horizontal(),
        UIButton::new(Message::SaveFileAs)
            .text("abrir")
            .size(Size2D::new(100., 45.))
            .create(),
    ]
    .spacing(8)
    .padding(Padding::from([8, 0]))
    .width(Length::Fill);

    container(column![header, file_line, horizontal_rule(2)])
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding::from((CHAR_SIZE * 2) as f32))
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
    UIButton::new(message)
        .glyph(glyph.0, glyph.1)
        .text(label)
        .create()
}

pub fn theme() -> iced::theme::Theme {
    iced::theme::Theme::Dark
}

#[derive(Clone, Copy)]
struct UIButton<'a> {
    glyph: Option<UIText<'a>>,
    text: Option<UIText<'a>>,
    base: ButtonBase,
    action: Message,
    size: Size2D,
}
impl<'a> UIButton<'a> {
    /// Starts the building of a button.
    ///
    /// ```
    /// // Default configuration
    /// UIButton {
    ///     action, //obligatory parameter
    ///     glyph: None,
    ///     text: None,
    ///     base: ButtonBase::new(BUTTON, BUTTON_HOVER, 8.0.into()),
    ///     size: Size2D { width: Fill, height: 45.0 },
    /// }
    /// ```
    fn new(action: Message) -> Self {
        let size = Size2D::new(0., 45.).fill_x();
        Self {
            glyph: None,
            text: None,
            base: ButtonBase::new(BUTTON, BUTTON_HOVER, 8.0.into()),
            action,
            size,
        }
    }

    fn glyph(mut self, content: &'a str, color: Color) -> Self {
        self.glyph = Some(UIText::new(content, CHAR_SIZE).color(color));
        self
    }

    fn glyph_sized(mut self, content: &'a str, color: Color, size: u32) -> Self {
        self.glyph = Some(UIText::new(content, size).color(color));
        self
    }

    fn text(mut self, content: &'a str) -> Self {
        self.text = Some(UIText::new(content, CHAR_SIZE));
        self
    }

    fn text_sized(mut self, content: &'a str, size: u32) -> Self {
        self.text = Some(UIText::new(content, size));
        self
    }

    fn transparent(mut self) -> Self {
        self.base = ButtonBase::transparent();
        self
    }

    fn base(mut self, active: Color, hovered: Color, radius: iced::border::Radius) -> Self {
        self.base = ButtonBase::new(active, hovered, radius);
        self
    }

    fn size(mut self, size: Size2D) -> Self {
        self.size = size;
        self
    }

    fn render_content(self) -> Container<'a, Message> {
        match (&self.glyph, &self.text) {
            (None, None) => container(BLANK_TEXT.create()),
            (Some(e), None) | (None, Some(e)) => container(e.create()),
            (Some(g), Some(t)) => {
                container(rich_text![g.create_span(), span(" "), t.create_span()])
            }
        }
        .center_x(Length::Fill)
        .align_y(Vertical::Center)
    }

    /// Finishes the construction and builds it.
    /// > Warning: this consumes the `Button`
    fn create(self) -> button::Button<'a, Message> {
        let content = self.render_content();

        button(content)
            .width(self.size.width)
            .height(self.size.height)
            .style(move |_, status| button::Style {
                text_color: WHITE,
                background: self.base.create(status),
                border: self.base.border,
                ..Default::default()
            })
            .on_press(self.action)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct UIText<'a> {
    content: &'a str,
    color: Color,
    size: u32,
    font: Font,
}

impl<'a> Default for UIText<'a> {
    fn default() -> Self {
        Self {
            content: "",
            color: WHITE,
            size: CHAR_SIZE,
            font: SEMIBOLD,
        }
    }
}

impl<'a> UIText<'a> {
    /// Starts the building of a text.
    ///
    /// ```
    /// // Default configuration
    /// UIText {
    ///     size, // obligatory parameter
    ///     content, // obligatory parameter
    ///     color: WHITE,
    ///     font: SEMIBOLD,
    /// }
    /// ```
    pub(crate) fn new(content: &'a str, size: u32) -> Self {
        Self {
            size,
            content,
            color: WHITE,
            font: SEMIBOLD,
        }
    }

    pub(crate) fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub(crate) fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    /// Finishes the construction and builds it.
    /// > Warning: this consumes the `UIText`
    pub(crate) fn create(self) -> Text<'a> {
        text(self.content)
            .size(self.size)
            .line_height(iced::Pixels(self.size as f32))
            .font(self.font)
            .color(self.color)
    }
    /// Finishes the construction and builds it.
    /// > Warning: this consumes the `UIText`
    pub(crate) fn create_span(self) -> text::Span<'a> {
        span(self.content)
            .size(self.size)
            .line_height(iced::Pixels(self.size as f32))
            .font(self.font)
            .color(self.color)
    }
}

#[derive(Clone, Copy)]
struct ButtonBase {
    is_none: bool,
    active: Color,
    hovered: Color,
    border: Border,
}
impl ButtonBase {
    fn transparent() -> Self {
        ButtonBase {
            is_none: true,
            active: Color::TRANSPARENT,
            hovered: Color::TRANSPARENT,
            border: Border::default(),
        }
    }

    fn new(active: Color, hovered: Color, radius: iced::border::Radius) -> Self {
        let border = Border::default().rounded(radius);
        Self {
            is_none: false,
            active,
            hovered,
            border,
        }
    }

    fn gen_state_color(&self, status: button::Status) -> Color {
        if matches!(status, button::Status::Hovered) {
            self.hovered
        } else {
            self.active
        }
    }

    fn create(&self, status: button::Status) -> Option<Background> {
        if self.is_none {
            return None;
        }
        Some(Background::Color(self.gen_state_color(status)))
    }
}

#[derive(Clone, Copy)]
struct Size2D {
    width: Length,
    height: Length,
}
impl Size2D {
    fn new(x: f32, y: f32) -> Self {
        Size2D {
            width: Length::Fixed(x),
            height: Length::Fixed(y),
        }
    }

    fn fill_x(mut self) -> Self {
        self.width = Length::Fill;
        self
    }
    fn fill_y(mut self) -> Self {
        self.height = Length::Fill;
        self
    }
}
