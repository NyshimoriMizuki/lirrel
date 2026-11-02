mod audio;
mod gui;
mod storage;

pub use gui::SEMIBOLD;
use gui::{WINDOW_HEIGHT, WINDOW_WIDTH};
use storage::{Stage, TimerConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    MenuPressed,
    BackPressed,
    PlayPressed,
    SkipPressed,
    ZeroPressed,
    ResetPressed,
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
    alarm: String,
    stage_name: String,
    stages: Vec<Stage>,
    progress: ProgressTracker,
    cycling: bool,
}

impl Lirrel {
    // iced functions

    pub fn boot() -> Self {
        Self {
            clock: Clock {
                value: String::from("12:30"),
            },
            stage_name: String::from("Usual Cicle"),
            alarm: String::from("beep"),
            stages: vec![Stage::new("Pause", 300), Stage::new("Work", 1500)],
            progress: ProgressTracker::new(2),
            cycling: false,
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        gui::view_main(&self.clock.value, &self.stage_name, &self.progress).into()
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
        let config = match TimerConfig::from_file(filename) {
            Ok(tc) => tc,
            Err(e) => {
                eprintln!("Error: {e}");
                return;
            }
        };
        self.stage_name = config.name;
        self.stages = config.stages;
        self.alarm = config.alarm;
        self.progress.reset();
    }
}
