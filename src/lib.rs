mod audio;
mod gui;
mod storage;

use std::borrow::Cow;

use iced::{Task, window};
use rfd::{AsyncMessageDialog, MessageDialogResult};

pub use gui::SEMIBOLD;
use gui::{WINDOW_HEIGHT, WINDOW_WIDTH};
use storage::{BLANK_STAGE, Stage, TimerConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    ToMain,
    ToMenu,
    Back,
    Play,
    Skip,
    Zero,
    Reset,
    OpenFile,
    SaveFile,
    SaveFileAs,
    CloseRequest(window::Id),
    CloseRequestResponse(window::Id, MessageDialogResult),
}

enum Screen {
    Main,
    Menu,
}

struct Clock {
    // this is temporary
    value: String,
}

#[derive(Default, Clone)]
pub(crate) struct ProgressTracker {
    pub current: usize,
    total: usize,
}
impl ProgressTracker {
    fn new(limit: usize) -> Self {
        Self {
            current: 0,
            total: limit,
        }
    }

    fn reset(&mut self) {
        self.current = 0;
    }

    fn update(&mut self) {
        self.current = if self.current >= self.total {
            1
        } else {
            self.current + 1
        }
    }

    fn update_total(&mut self, value: usize) {
        self.total = value
    }

    pub(crate) fn show(&self) -> String {
        format!("{} / {}", self.current + 1, self.total)
    }
}

pub struct Lirrel {
    clock: Clock,
    config: TimerConfig,
    progress: ProgressTracker,
    cur_screen: Screen,

    confirm_before_closing: bool,
}

impl Lirrel {
    // iced functions

    pub fn boot() -> Self {
        Self {
            clock: Clock {
                value: String::from("12:30"),
            },
            config: TimerConfig::default(),
            progress: ProgressTracker::new(TimerConfig::DEFAULT_STAGES_NUMBER),
            cur_screen: Screen::Main,

            confirm_before_closing: true,
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        if let Screen::Menu = self.cur_screen {
            return gui::view_menu().into();
        }
        let cur_stage = self.get_current_stage().unwrap_or(&*BLANK_STAGE);
        gui::view_main(&self.clock.value, &cur_stage, &self.progress).into()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::ToMain => self.cur_screen = Screen::Main,
            Message::ToMenu => self.cur_screen = Screen::Menu,
            Message::CloseRequest(id) => {
                if !self.confirm_before_closing {
                    return window::close(id);
                }

                let dialog = rfd::AsyncMessageDialog::new()
                    .set_title("Quer sair agora?")
                    .set_description("Seu timer ainda está rodando!\n\nDeseja realmente sair?")
                    .set_level(rfd::MessageLevel::Warning)
                    .set_buttons(rfd::MessageButtons::YesNo)
                    .show();
                return Task::perform(dialog, move |result| {
                    Message::CloseRequestResponse(id, result)
                });
            }
            Message::CloseRequestResponse(id, result) => match result {
                MessageDialogResult::Yes | MessageDialogResult::Ok => {
                    return window::close(id);
                }
                _ => return Task::none(),
            },
            m => println!("{:?}", m),
        };

        Task::none()
    }

    pub fn window_settings() -> iced::window::Settings {
        let fixed_size = iced::Size::new(WINDOW_WIDTH, WINDOW_HEIGHT);

        iced::window::Settings {
            size: fixed_size,
            resizable: false,
            ..Default::default()
        }
    }

    pub fn theme() -> iced::Theme {
        gui::theme()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch([window::close_requests().map(|id| Message::CloseRequest(id))])
    }
}

impl Lirrel {
    // general functions

    /// Loads a stages file, updating the internal configuration.
    fn load_cycle(&mut self, filename: &str) {
        self.config = match TimerConfig::from_file(filename) {
            Ok(tc) => tc,
            Err(e) => {
                eprintln!("Error: {e}");
                return;
            }
        };
        self.progress.reset();
    }

    fn get_current_stage(&self) -> Option<&Stage> {
        self.config.stages.get(self.progress.current)
    }
}
