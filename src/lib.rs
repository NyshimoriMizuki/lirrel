use iced::widget::{column, container, text};
use iced::{Center, Element, Length::Fill};

pub enum Message {
    Play,
    Stop,
    ResetStage,
    ResetAll,
    OpenSettings,
    CloseSettings,
}

pub struct Lirrel;
impl Lirrel {
    pub fn boot() -> Self {
        Lirrel
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(column![text("hello").size(40)])
            .align_x(Center)
            .center_x(Fill)
            .into()
    }

    pub fn update(&mut self, _msg: Message) {
        todo!()
    }
}
