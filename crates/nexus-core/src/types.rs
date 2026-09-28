use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Capability {
    Input,
    Screen,
    Network,
    FileRead,
    FileWrite,
    Process,
    Device,
    KbRead,
    KbWrite,
    Telemetry,
}

impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Capability::Input => write!(f, "input"),
            Capability::Screen => write!(f, "screen"),
            Capability::Network => write!(f, "network"),
            Capability::FileRead => write!(f, "file:read"),
            Capability::FileWrite => write!(f, "file:write"),
            Capability::Process => write!(f, "process"),
            Capability::Device => write!(f, "device"),
            Capability::KbRead => write!(f, "kb:read"),
            Capability::KbWrite => write!(f, "kb:write"),
            Capability::Telemetry => write!(f, "telemetry"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Mode {
    Manual,
    Assisted,
    Autonomous,
    Scheduled,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Manual => write!(f, "Manual"),
            Mode::Assisted => write!(f, "Assisted"),
            Mode::Autonomous => write!(f, "Autonomous"),
            Mode::Scheduled => write!(f, "Scheduled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub mode: Mode,
    pub kill_switch: Vec<String>,
    pub input_profile: InputProfile,
    pub caps: Vec<Capability>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputProfile {
    pub mouse_sensitivity: f64,
    pub accel: bool,
    pub key_delay_ms: u64,
    pub deadzone: f64,
    pub tokens_per_sec: u64,
}

impl Default for InputProfile {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 1.0,
            accel: false,
            key_delay_ms: 8,
            deadzone: 0.12,
            tokens_per_sec: 300,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: Mode::Manual,
            kill_switch: vec!["Ctrl".into(), "Alt".into(), "K".into()],
            input_profile: InputProfile::default(),
            caps: vec![Capability::Input, Capability::Screen],
        }
    }
}
