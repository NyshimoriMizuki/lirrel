mod audio;
mod gui;
mod storage;

use iced::time::{self, Duration, Instant};
use iced::{Subscription, Task, window};
use rfd::{AsyncMessageDialog, MessageDialogResult};
use std::borrow::Cow;

pub use gui::SEMIBOLD;
use gui::{WINDOW_HEIGHT, WINDOW_WIDTH};
use storage::{BLANK_STAGE, Stage, TimerConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    ToMain,
    ToMenu,
    Back,
    ToggleTimer,
    Skip,
    Zero,
    Reset,
    OpenFile,
    SaveFile,
    SaveFileAs,
    Tick(Instant),
    CloseRequest(window::Id),
    CloseRequestResponse(window::Id, MessageDialogResult),
}

enum Screen {
    Main,
    Menu,
}

#[derive(Debug, PartialEq, Eq)]
enum TimerStatus {
    // Stopped,
    Completed,
    InProgress,
}
#[derive(Debug, Default)]
struct Clock {
    remaning: Duration,
    deadline: Option<Instant>,
}
impl Clock {
    fn new(duration: Duration) -> Self {
        Self {
            remaning: duration,
            deadline: None,
        }
    }

    fn is_running(&self) -> bool {
        self.deadline.is_some()
    }

    fn run(&mut self) {
        self.deadline = Some(Instant::now() + self.remaning)
    }

    fn pause(&mut self) {
        self.remaning = self
            .deadline
            .take()
            .map(|dl| dl.saturating_duration_since(Instant::now()))
            .unwrap_or(self.remaning);
    }

    fn update(&mut self, now: Instant) -> TimerStatus {
        let Some(deadline) = self.deadline else {
            // return TimerStatus::Stopped;
            return TimerStatus::Completed;
        };
        self.remaning = deadline.saturating_duration_since(now);
        if self.remaning.is_zero() {
            self.deadline = None;
            return TimerStatus::Completed;
        }

        TimerStatus::InProgress
    }
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

    /// it returns `true` if it overflows and reset the counting.
    fn proceed(&mut self) -> bool {
        if self.current + 1 < self.total {
            self.current += 1;
            false
        } else {
            self.current = 0;
            true
        }
    }

    /// it returns `true` if it overflows and go to the last stage.
    fn regress(&mut self) -> bool {
        if self.current == 0 {
            self.current = self.total - 1;
            true
        } else {
            self.current -= 1;
            false
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
        let config = TimerConfig::default();
        Self {
            clock: Clock::new(config.stages.get(0).unwrap().duration),
            config,
            progress: ProgressTracker::new(TimerConfig::DEFAULT_STAGES_NUMBER),
            cur_screen: Screen::Main,

            confirm_before_closing: false,
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        if let Screen::Menu = self.cur_screen {
            return gui::view_menu().into();
        }
        let cur_stage = self.get_current_stage();
        gui::view_main(&self.clock.remaning, &cur_stage, &self.progress).into()
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
            Message::ToggleTimer => {
                if self.clock.is_running() {
                    self.clock.pause();
                    self.confirm_before_closing = false;
                } else {
                    self.clock.run();
                    self.confirm_before_closing = true;
                }
            }
            Message::Tick(now) => {
                let status = self.clock.update(now);
                if status == TimerStatus::Completed {
                    let overflowed = self.load_next_stage();

                    if overflowed && !self.config.cycling {
                        return Task::none();
                    }
                    self.clock.run();
                }
            }
            Message::Reset => todo!(),
            Message::Back => self.load_previus_stage(),
            Message::Skip => {
                self.load_next_stage();
            }
            Message::Zero => todo!(),
            Message::OpenFile => todo!(),
            Message::SaveFile => todo!(),
            Message::SaveFileAs => todo!(),
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

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            match self.clock.is_running() {
                true => time::every(Duration::from_millis(10)).map(Message::Tick),
                false => Subscription::none(),
            },
            window::close_requests().map(|id| Message::CloseRequest(id)),
        ])
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
        self.progress.update_total(self.config.stages.len());
        self.progress.reset();
    }

    fn get_current_stage(&self) -> &Stage {
        self.config
            .stages
            .get(self.progress.current)
            .unwrap_or(&*BLANK_STAGE)
    }

    // returns `true` if overflowed
    fn load_next_stage(&mut self) -> bool {
        let overflowed = self.progress.proceed();
        self.clock = Clock::new(self.get_current_stage().duration);
        overflowed
    }

    fn load_previus_stage(&mut self) {
        let _ = self.progress.regress();
        self.clock = Clock::new(self.get_current_stage().duration);
    }
}
