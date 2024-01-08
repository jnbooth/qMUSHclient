use tr::TrContext;

use crate::send::Event;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TrContext)]
pub enum Pad {
    Script(String),
    Alias { plugin: String, label: String },
    Timer { plugin: String, event: Event },
    Trigger { plugin: String, label: String },
    PacketDebug,
}

impl Pad {
    pub fn title(&self, pad_name: &str) -> String {
        match self {
            Self::Script(s) => format!("{} - {}", s, pad_name),
            Self::Alias { plugin, label } => {
                tr!("Alias: {} ({}) - {}", label, plugin, pad_name).to_std_string()
            }
            Self::Timer { plugin, event } => {
                tr!("Timer: {} ({}) - {}", event, plugin, pad_name).to_std_string()
            }
            Self::Trigger { plugin, label } => {
                tr!("Trigger: {} ({}) - {}", label, plugin, pad_name).to_std_string()
            }
            Self::PacketDebug => tr!("Packet debug - {}", pad_name).to_std_string(),
        }
    }
}
