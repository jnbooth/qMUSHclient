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
        // SAFETY: fields are valid.
        unsafe {
            connect_world!(
                self,
                proxy_username,
                proxy_password,
            );
        }
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
        unsafe {
            // SAFETY: fields are valid.
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
            self.ui.proxy_password_button.clicked().connect(&self.slot_open_proxy_settings());
        }
    }

    #[slot(SlotNoArgs)]
    fn open_proxy_settings(&self) {
        let page = PrefsProxy::new(self.widget(), self.world.clone());
        unsafe {
            page.ui.widget.exec();
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
        let ui = &self.ui;
        let connect_text = unsafe { ui.connect_text.to_plain_text().trimmed().to_std_string() };
        let lines = connect_text.lines().count();
        if lines != self.lines_cache.replace(lines) {
            unsafe {
                ui.connect_text_lines.set_text(&tr!(lines, "(%n line(s))"));
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
        // SAFETY: fields are valid.
        unsafe {
            connect_world!(
                self,
                log_file,
                log_file_preamble,
                log_file_postamble,
                log_output,
                log_input,
                log_notes,
                log_preamble_output,
                log_preamble_input,
                log_preamble_notes,
                log_postamble_output,
                log_postamble_input,
                log_postamble_notes,
            );

            let ui = &self.ui;
            let world = self.world.clone();
            self.connect_browse_button(
                Browse::Save,
                &ui.log_file_browse,
                &ui.log_file,
                move || QString::from_std_str(
                    &format!("logs/{}.txt", world.upgrade().unwrap().borrow().name)
                ),
                "Text files (*.txt)",
            );
            ui.button_box.help_requested().connect(&self.slot_show_help());
            let reset = ui.button_box.button(StandardButton::RestoreDefaults);
            reset.clicked().connect(&self.slot_set_defaults());

            let worldrc = self.world.upgrade().unwrap();
            let worldref = &mut *worldrc.borrow_mut();
            self.connect(
                worldref,
                &[ui.log_text.clone(), ui.log_html.clone(), ui.log_raw.clone()],
                |world| &mut world.log_format,
            );
            self.connect(worldref,
                &[ui.log_append.clone(), ui.log_overwrite.clone()],
                |world| &mut world.log_mode,
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
        let ui = &self.ui;
        unsafe {
            ui.log_file_preamble
                .set_plain_text(&QString::from_std_str(DEFAULT_PREAMBLE));
            ui.log_file_postamble
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
        // SAFETY: fields are valid.
        unsafe {
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
            let ui = &self.ui;
            self.connect_browse_button(
                Browse::Directory,
                &ui.chat_file_save_directory_browse,
                &ui.chat_file_save_directory,
                || QString::new(),
                "",
            );
        }
    }
}
