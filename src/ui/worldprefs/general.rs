use std::cell::RefCell;
use std::rc::{Rc, Weak};

use cpp_core::{CastInto, Ptr};
use qt_core::{slot, QPtr, QString, SlotNoArgs};
use qt_widgets::q_dialog_button_box::StandardButton;
use qt_widgets::*;

use super::PrefPageNew;
use crate::binding::{Browse, RWidget};
use crate::tr::TrContext;
use crate::ui::uic;
use crate::world::World;

#[derive(Debug, RWidget, TrContext)]
pub struct PrefsProxy {
    ui: uic::PrefsProxy,
    world: Weak<RefCell<World>>,
}
impl_prefpage!(PrefsProxy);
impl_prefpagenew!(PrefsProxy);

impl PrefsProxy {
    #[rustfmt::skip]
    fn init(self: &Rc<Self>) {
        connect_world!(
            self,
            proxy_username,
            proxy_password,
        );
    }
}

#[derive(Debug, RWidget, TrContext)]
pub struct PrefsAddress {
    ui: uic::PrefsAddress,
    world: Weak<RefCell<World>>,
}
impl_prefpage!(PrefsAddress);
impl_prefpagenew!(PrefsAddress);

impl PrefsAddress {
    #[rustfmt::skip]
    fn init(self: &Rc<Self>) {
        connect_world!(
            self,
            name,
            site,
            port,
            proxy_type,
            proxy_server,
            proxy_port,
            save_world_automatically,
        );
        unsafe {
            self.ui.proxy_password_button.clicked().connect(&self.slot_open_proxy_settings());
        }
    }

    #[slot(SlotNoArgs)]
    fn open_proxy_settings(&self) {
        let page = PrefsProxy::new(self.widget(), self.world.clone());
        unsafe {
            let a = page.ui.widget.exec();
            println!("{}", a);
        }
    }
}

#[derive(Debug, RWidget, TrContext)]
pub struct PrefsConnecting {
    ui: uic::PrefsConnecting,
    world: Weak<RefCell<World>>,
    lines_cache: RefCell<usize>,
}
impl_prefpage!(PrefsConnecting);

impl PrefPageNew for PrefsConnecting {
    fn new<P: CastInto<Ptr<QWidget>>>(parent: P, world: Weak<RefCell<World>>) -> Rc<Self> {
        let this = Rc::new(Self {
            ui: uic::PrefsConnecting::load(parent),
            world,
            lines_cache: RefCell::new(0),
        });
        this.init();
        this
    }
}

impl PrefsConnecting {
    #[rustfmt::skip]
    fn init(self: &Rc<Self>) {
        unsafe {
            connect_world!(
                self,
                player,
                password,
                connect_method,
            );
            self.ui.connect_text.text_changed().connect(&self.slot_update_connect_text());
        }
    }

    #[slot(SlotNoArgs)]
    fn update_connect_text(&self) {
        let connect_text = unsafe {
            self.ui
                .connect_text
                .to_plain_text()
                .trimmed()
                .to_std_string()
        };
        let lines = connect_text.lines().count();
        if lines != self.lines_cache.replace(lines) {
            unsafe {
                self.ui
                    .connect_text_lines
                    .set_text(&tr!(lines, "(%n line(s))"));
            }
        }
        if let Some(mut world) = self
            .world
            .upgrade()
            .as_ref()
            .and_then(|x| x.try_borrow_mut().ok())
        {
            world.connect_text = connect_text;
        }
    }
}

#[derive(Debug, RWidget, TrContext)]
pub struct PrefsLogging {
    ui: uic::PrefsLogging,
    world: Weak<RefCell<World>>,
}
impl_prefpage!(PrefsLogging);
impl_prefpagenew!(PrefsLogging);

const DEFAULT_PREAMBLE: &str = r#"<html>
  <head>
    <title>Log of %N session</title>
    <style type="text/css">
      body { background-color: black; }
    </style>
  </head>
  <body>
    <pre>
      <code>
        <font size=2 face="FixedSys, Lucida Console, Courier New, Courier">"#;

const DEFAULT_POSTAMBLE: &str = r#"        </font>
      </code>
    </pre>
  </body>
</html>"#;

impl PrefsLogging {
    #[rustfmt::skip]
    fn init(self: &Rc<Self>) {
        connect_world!(
            self,
            log_file_preamble,
            log_file_postamble,
            log_output,
            log_input,
            log_notes,
            log_html,
            log_in_color,
            log_raw,
            write_world_name_to_log,
            log_line_preamble_output,
            log_line_preamble_input,
            log_line_preamble_notes,
            log_line_postamble_output,
            log_line_postamble_input,
            log_line_postamble_notes,
        );
        self.connect_browse_button(
            Browse::Save,
            &self.ui.auto_log_file_name_browse,
            &self.ui.auto_log_file_name,
            "logs",
            "Text files (*.txt)",
        );
        unsafe {
            self.ui.button_box.help_requested().connect(&self.slot_show_help());
            let reset = self.ui.button_box.button(StandardButton::RestoreDefaults);
            reset.clicked().connect(&self.slot_set_defaults());
            self.enable_if(
                &self.ui.log_html,
                true,
                [self.ui.log_in_color.static_upcast(), reset.static_upcast()],
            );
        }
    }

    #[slot(SlotNoArgs)]
    fn show_help(&self) {
        unsafe {
            uic::SpecialHelp::load(self.widget()).widget.exec();
        }
    }

    #[slot(SlotNoArgs)]
    fn set_defaults(&self) {
        unsafe {
            self.ui
                .log_file_preamble
                .set_plain_text(&QString::from_std_str(DEFAULT_PREAMBLE));
            self.ui
                .log_file_postamble
                .set_plain_text(&QString::from_std_str(DEFAULT_POSTAMBLE));
        }
    }
}

#[derive(Debug, RWidget, TrContext)]
pub struct PrefsChat {
    ui: uic::PrefsChat,
    world: Weak<RefCell<World>>,
}
impl_prefpage!(PrefsChat);
impl_prefpagenew!(PrefsChat);

impl PrefsChat {
    fn init(self: &Rc<Self>) {
        connect_world!(
            self,
            chat_name,
            auto_allow_snooping,
            accept_chat_connections,
            chat_port,
            validate_incoming_chat_calls,
            chat_colors.foreground chat_colors_foreground,
            chat_colors.background chat_colors_background,
            ignore_chat_colors,
            chat_message_prefix,
            chat_max_lines_per_message,
            chat_max_bytes_per_message,
            auto_allow_files,
            chat_file_save_directory,
        );
        self.connect_browse_button(
            Browse::Directory,
            &self.ui.chat_file_save_directory_browse,
            &self.ui.chat_file_save_directory,
            "",
            "",
        );
    }
}
