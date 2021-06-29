use crate::binding::color::{RColor, RColorPair};
use crate::binding::RFont;
use crate::client::color::{Colors, WorldColor};
use crate::enums::Enum;
use crate::script::{PluginId, PluginMetadata};
use crate::Version;
use chrono::{DateTime, Local};
use hashbrown::HashMap;
use qt_core::{GlobalColor, Key};
use qt_gui::q_font_database::SystemFont;
use qt_gui::q_palette::ColorRole;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Enum)]
pub enum ProxyType {
    Socks4,
    Socks5,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Enum)]
pub enum AutoConnect {
    MUSH,
    Diku,
    MXP,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Enum)]
pub enum UseMxp {
    Command,
    Query,
    Always,
    Never,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Enum)]
pub enum ScriptRecompile {
    Confirm,
    Always,
    Never,
}
fn default_foreground() -> RColor {
    RColor::from(ColorRole::Text)
}

mod keypad_serde {
    use hashbrown::HashMap;
    use qt_core::Key;
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::{Serialize, Serializer};
    use std::mem;
    use std::os::raw::c_int;

    type Shim<V> = HashMap<c_int, V>;

    pub fn serialize<S, V>(map: &HashMap<Key, V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        V: Serialize,
    {
        // SAFETY: Key is repr(transparent) as c_int
        unsafe { mem::transmute::<_, &Shim<V>>(map) }.serialize(serializer)
    }

    pub fn deserialize<'de, D, V>(deserializer: D) -> Result<HashMap<Key, V>, D::Error>
    where
        D: Deserializer<'de>,
        V: Deserialize<'de>,
    {
        // SAFETY: Key is repr(transparent) as c_int
        Shim::<V>::deserialize(deserializer).map(|shim| unsafe { mem::transmute(shim) })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct World {
    // IP address
    pub name: String,
    pub site: String,
    pub port: u16,
    pub proxy_type: Option<ProxyType>,
    pub proxy_server: String,
    pub proxy_port: u16,
    pub proxy_username: String,
    pub proxy_password: String,
    pub proxy_password_base64: bool,
    pub save_world_automatically: bool,

    // Connecting
    pub player: String,
    pub password: String,
    pub connect_method: Option<AutoConnect>,
    pub connect_text: String,

    // Logging
    pub log_file_preamble: String,
    pub log_file_postamble: String,
    pub log_output: bool,
    pub log_input: bool,
    pub log_notes: bool,
    pub log_html: bool,
    pub log_in_color: bool,
    pub log_raw: bool,
    pub write_world_name_to_log: bool,
    pub auto_log_file_name: String,
    pub log_line_preamble_output: String,
    pub log_line_preamble_input: String,
    pub log_line_preamble_notes: String,
    pub log_line_postamble_output: String,
    pub log_line_postamble_input: String,
    pub log_line_postamble_notes: String,

    // Timers
    pub enable_timers: bool,
    pub treeview_timers: bool,

    // Chat
    pub chat_name: String,
    pub auto_allow_snooping: bool,
    pub accept_chat_connections: bool,
    pub chat_port: u16,
    pub validate_incoming_chat_calls: bool,
    pub chat_colors: RColorPair,
    pub ignore_chat_colors: bool,
    pub chat_message_prefix: String,
    pub chat_max_lines_per_message: usize,
    pub chat_max_bytes_per_message: usize,
    pub auto_allow_files: bool,
    pub chat_file_save_directory: String,

    // Notes
    pub notes: String,

    // Output
    pub beep_sound: String,
    pub pixel_offset: i16,
    pub line_spacing: f32,
    pub output_font: RFont,
    pub use_default_output_font: bool,
    pub show_bold: bool,
    pub show_italic: bool,
    pub show_underline: bool,
    pub new_activity_sound: String,
    pub max_output_lines: usize,
    pub wrap_column: u16,

    pub line_information: bool,
    pub start_paused: bool,
    pub auto_pause: bool,
    pub unpause_on_send: bool,
    pub flash_taskbar_icon: bool,
    pub disable_compression: bool,
    pub indent_paras: bool,
    pub naws: bool,
    pub carriage_return_clears_line: bool,
    pub utf_8: bool,
    pub auto_wrap_window_width: bool,
    pub show_connect_disconnect: bool,
    pub copy_selection_to_clipboard: bool,
    pub auto_copy_to_clipboard_in_html: bool,
    pub convert_ga_to_newline: bool,
    pub terminal_identification: String,

    // MXP / Pueblo
    pub use_mxp: UseMxp,
    pub detect_pueblo: bool,
    pub hyperlink_color: RColor,
    pub use_custom_link_color: bool,
    pub mud_can_change_link_color: bool,
    pub underline_hyperlinks: bool,
    pub mud_can_remove_underline: bool,
    pub hyperlink_adds_to_command_history: bool,
    pub echo_hyperlink_in_output_window: bool,
    pub ignore_mxp_color_changes: bool,
    pub send_mxp_afk_response: bool,
    pub mud_can_change_options: bool,

    // ANSI Color
    pub use_default_colors: bool,
    pub ansi_colors: [RColor; 16],

    // Custom Color
    pub custom_names: [String; 16],
    pub custom_colors: [RColorPair; 16],

    // Triggers
    pub enable_triggers: bool,
    pub enable_trigger_sounds: bool,
    pub treeview_triggers: bool,

    // Commands
    pub display_my_input: bool,
    pub echo_colors: RColorPair,
    pub enable_speed_walk: bool,
    pub speed_walk_prefix: String,
    pub speed_walk_filler: String,
    pub speed_walk_delay: u32,
    pub enable_command_stack: bool,
    pub command_stack_character: String,
    pub input_colors: RColorPair,
    pub input_font: RFont,
    pub use_default_input_font: bool,
    pub enable_spam_prevention: bool,
    pub spam_line_count: usize,
    pub spam_message: String,

    pub auto_repeat: bool,
    pub lower_case_tab_completion: bool,
    pub translate_german: bool,
    pub translate_backslash_sequences: bool,
    pub keep_commands_on_same_line: bool,
    pub no_echo_off: bool,
    pub tab_completion_lines: usize,
    pub tab_completion_space: bool,

    pub double_click_inserts: bool,
    pub double_click_sends: bool,
    pub escape_deletes_input: bool,
    pub save_deleted_command: bool,
    pub confirm_before_replacing_typing: bool,
    pub arrow_keys_wrap: bool,
    pub arrows_change_history: bool,
    pub arrow_recalls_partial: bool,
    pub alt_arrow_recalls_partial: bool,
    pub ctrl_z_goes_to_end_of_buffer: bool,
    pub ctrl_p_goes_to_previous_command: bool,
    pub ctrl_n_goes_to_next_command: bool,
    pub history_lines: usize,

    // Aliases
    pub enable_aliases: bool,
    pub treeview_aliases: bool,

    // Keypad
    pub keypad_enable: bool,
    #[serde(with = "keypad_serde")]
    pub keypad_shortcuts: HashMap<Key, String>,

    // Auto Say
    pub enable_auto_say: bool,
    pub autosay_exclude_non_alpha: bool,
    pub autosay_exclude_macros: bool,
    pub auto_say_override_prefix: String,
    pub auto_say_string: String,
    pub re_evaluate_auto_say: bool,

    // Paste
    pub paste_line_preamble: String,
    pub paste_line_postamble: String,
    pub paste_delay: u32,
    pub paste_delay_per_lines: u32,
    pub paste_commented_softcode: bool,
    pub paste_echo: bool,
    pub confirm_on_paste: bool,

    // Send
    pub send_line_preamble: String,
    pub send_line_postamble: String,
    pub send_delay: u32,
    pub send_delay_per_lines: u32,
    pub send_commented_softcode: bool,
    pub send_echo: bool,
    pub confirm_on_send: bool,

    // Scripts
    pub script_prefix: String,
    pub enable_scripts: bool,
    pub warn_if_scripting_inactive: bool,
    pub script_filename: String,
    pub edit_script_with_notepad: bool,
    pub script_editor: String,
    pub script_reload_option: ScriptRecompile,
    pub script_errors_to_output_window: bool,
    pub note_text_color: RColor,
}

impl World {
    pub fn color<'a, 'b>(&'a self, col: &'b WorldColor) -> &'b RColor
    where
        'a: 'b,
    {
        match col {
            WorldColor::Ansi(i) => &self.ansi_colors[*i],
            WorldColor::CustomFg(i) => &self.custom_colors[*i].foreground,
            WorldColor::CustomBg(i) => &self.custom_colors[*i].background,
            WorldColor::Xterm(i) => Colors::xterm(*i),
            WorldColor::Plain(c) => &c,
        }
    }

    pub fn make_plugin(&self) -> io::Result<Option<PluginMetadata>> {
        if self.script_filename.is_empty() {
            return Ok(None);
        }
        let source = PathBuf::from(&self.script_filename);
        let mut file = File::open(&source)?;
        let metadata = file.metadata()?;
        let mut script = String::new();
        file.read_to_string(&mut script)?;
        Ok(Some(PluginMetadata {
            name: format!("World Script: {}", self.name),
            author: "User".to_owned(),
            purpose: "User-provided script file".to_owned(),
            description: "Executes functions provided by the user in World Preferences".to_owned(),
            script,
            source,
            id: PluginId::nil(),
            written: DateTime::from(metadata.created()?).date(),
            modified: DateTime::from(metadata.modified()?).date(),
            version: Version(0),
            client_required: Version(0),
            installed: Local::today(),
            sequence: i16::MIN,
        }))
    }

    // Each plugin has one of these.
    pub fn custom_color_map(&self) -> HashMap<String, RColorPair> {
        let custom_names = self.custom_names.iter().map(ToOwned::to_owned);
        let custom_colors = self.custom_colors.iter().map(ToOwned::to_owned);
        custom_names.zip(custom_colors).collect()
    }

    pub fn new() -> Self {
        let custom_names: [String; 16] = (1..=16)
            .map(|i| format!("Custom{}", i))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let custom_colors: [RColorPair; 16] = Colors::ansi16()
            .to_vec()
            .into_iter()
            .map(|foreground| RColorPair {
                foreground,
                background: RColor::rgb(0, 0, 0),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self {
            // IP address
            name: String::new(),
            site: String::new(),
            port: 4000,
            proxy_type: None,
            proxy_server: String::new(),
            proxy_port: 1080,
            proxy_username: String::new(),
            proxy_password: String::new(),
            proxy_password_base64: true,
            save_world_automatically: false,

            // Connecting
            player: String::new(),
            password: String::new(),
            connect_method: None,
            connect_text: String::new(),

            // Logging
            log_file_preamble: String::new(),
            log_file_postamble: String::new(),
            log_output: true,
            log_input: false,
            log_notes: false,
            log_html: false,
            log_in_color: false,
            log_raw: false,
            write_world_name_to_log: false,
            auto_log_file_name: String::new(),
            log_line_preamble_output: String::new(),
            log_line_preamble_input: String::new(),
            log_line_preamble_notes: String::new(),
            log_line_postamble_output: String::new(),
            log_line_postamble_input: String::new(),
            log_line_postamble_notes: String::new(),

            // Timers
            enable_timers: true,
            treeview_timers: true,

            // Chat
            chat_name: String::new(),
            auto_allow_snooping: false,
            accept_chat_connections: false,
            chat_port: 4050,
            validate_incoming_chat_calls: false,
            chat_colors: RColorPair::new(GlobalColor::Red, GlobalColor::Transparent),
            ignore_chat_colors: false,
            chat_message_prefix: String::new(),
            chat_max_lines_per_message: 0,
            chat_max_bytes_per_message: 0,
            auto_allow_files: false,
            chat_file_save_directory: String::new(),

            // Notes
            notes: String::new(),

            // Output
            beep_sound: String::new(),
            pixel_offset: 0,
            line_spacing: 0.0,
            output_font: RFont::global(),
            use_default_output_font: true,
            show_bold: true,
            show_italic: true,
            show_underline: true,
            new_activity_sound: String::new(),
            max_output_lines: 5000,
            wrap_column: 80,

            line_information: true,
            start_paused: false,
            auto_pause: true,
            unpause_on_send: true,
            flash_taskbar_icon: false,
            disable_compression: false,
            indent_paras: true,
            naws: false,
            carriage_return_clears_line: false,
            utf_8: false,
            auto_wrap_window_width: false,
            show_connect_disconnect: true,
            copy_selection_to_clipboard: false,
            auto_copy_to_clipboard_in_html: false,
            convert_ga_to_newline: false,
            terminal_identification: "mushclient".to_owned(),

            // MXP / Pueblo
            use_mxp: UseMxp::Command,
            detect_pueblo: true,
            hyperlink_color: RColor::from(ColorRole::Link),
            use_custom_link_color: true,
            mud_can_change_link_color: true,
            underline_hyperlinks: true,
            mud_can_remove_underline: false,
            hyperlink_adds_to_command_history: true,
            echo_hyperlink_in_output_window: true,
            ignore_mxp_color_changes: false,
            send_mxp_afk_response: true,
            mud_can_change_options: true,

            // ANSI Color
            use_default_colors: true,
            ansi_colors: Colors::ansi16(),
            custom_names,
            custom_colors,

            // Triggers
            enable_triggers: true,
            enable_trigger_sounds: true,
            treeview_triggers: true,

            // Commands
            display_my_input: true,
            echo_colors: RColorPair::new(ColorRole::Text, GlobalColor::Transparent),
            enable_speed_walk: false,
            speed_walk_prefix: "#".to_owned(),
            speed_walk_filler: "a".to_owned(),
            speed_walk_delay: 20,
            enable_command_stack: false,
            command_stack_character: "#".to_owned(),
            input_colors: RColorPair::new(ColorRole::Text, ColorRole::Base),
            input_font: RFont::from(SystemFont::FixedFont),
            use_default_input_font: true,
            enable_spam_prevention: false,
            spam_line_count: 20,
            spam_message: "look".to_owned(),

            auto_repeat: false,
            lower_case_tab_completion: false,
            translate_german: false,
            translate_backslash_sequences: false,
            keep_commands_on_same_line: false,
            no_echo_off: false,
            tab_completion_lines: 200,
            tab_completion_space: false,

            double_click_inserts: false,
            double_click_sends: false,
            escape_deletes_input: false,
            save_deleted_command: false,
            confirm_before_replacing_typing: true,
            arrow_keys_wrap: false,
            arrows_change_history: true,
            arrow_recalls_partial: false,
            alt_arrow_recalls_partial: false,
            ctrl_z_goes_to_end_of_buffer: false,
            ctrl_p_goes_to_previous_command: false,
            ctrl_n_goes_to_next_command: false,
            history_lines: 1000,

            // Aliases
            enable_aliases: true,
            treeview_aliases: true,

            // Keypad
            keypad_enable: true,
            keypad_shortcuts: HashMap::new(),

            // Auto Say
            enable_auto_say: false,
            autosay_exclude_non_alpha: false,
            autosay_exclude_macros: false,
            auto_say_override_prefix: "-".to_owned(),
            auto_say_string: String::new(),
            re_evaluate_auto_say: false,

            // Paste
            paste_line_preamble: String::new(),
            paste_line_postamble: String::new(),
            paste_delay: 0,
            paste_delay_per_lines: 1,
            paste_commented_softcode: false,
            paste_echo: false,
            confirm_on_paste: true,

            // Send
            send_line_preamble: String::new(),
            send_line_postamble: String::new(),
            send_delay: 0,
            send_delay_per_lines: 1,
            send_commented_softcode: false,
            send_echo: false,
            confirm_on_send: true,

            // Scripts
            script_prefix: String::new(),
            enable_scripts: true,
            warn_if_scripting_inactive: true,
            script_filename: String::new(),
            edit_script_with_notepad: true,
            script_editor: "notepad".to_owned(),
            script_reload_option: ScriptRecompile::Confirm,
            script_errors_to_output_window: false,
            note_text_color: RColor::rgb(0, 128, 255),
        }
    }
}