mod audio;
mod gui;
mod storage;

pub use gui::SEMIBOLD;
use gui::{WINDOW_HEIGHT, WINDOW_WIDTH};
use storage::{BLANK_STAGE, Stage, TimerConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    ToMenu,
    Back,
    Play,
    Skip,
    Zero,
    Reset,
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
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let cur_stage = self.get_current_stage().unwrap_or(&*BLANK_STAGE);
        gui::view_main(&self.clock.value, &cur_stage, &self.progress).into()
    }

    pub fn update(&mut self, msg: Message) {
        println!("{:?}", msg)
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
