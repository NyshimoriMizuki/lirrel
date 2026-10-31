use iced::widget::{column, container, text};
use iced::{Center, Element, Length::Fill};

mod audio;
mod storage;

use storage::Stage;

pub enum Message {
    Play,
    Stop,
    ResetStage,
    ResetAll,
    OpenSettings,
    CloseSettings,
}

pub struct Lirrel {
    cycle_name: String,
    stages: Vec<Stage>,
    current_stage: usize,
}

impl Lirrel {
    // iced functions

    pub fn boot() -> Self {
        Self {
            cycle_name: String::new(),
            stages: Vec::new(),
            current_stage: 0,
        }
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

impl Lirrel {
    // general functions

    /// Loads a stages file, updating the `cycle_name` and the `stages`.
    fn load_cycle(&mut self, filename: &str) {
        todo!()
    }
}
