use super::uic;
use crate::binding::color::HasPalette;
use crate::binding::{Browse, RColor, RDialog, RFont, RWidget};
use crate::client::color::Colors;
use crate::tr::TrContext;
use crate::world::World;
use cpp_core::{CastInto, Ptr, Ref};
use hashbrown::HashMap;
use qt_core::{slot, CheckState, QPtr, QString, SlotNoArgs, SlotOfInt};
use qt_gui::q_palette::ColorRole;
use qt_widgets::q_dialog::DialogCode;
use qt_widgets::q_dialog_button_box::StandardButton;
use qt_widgets::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

mod qform;

use qform::QForm;

trait PrefPage {
    fn get_page(&self) -> QPtr<QWidget>;
    fn upgrade_world(&self) -> Option<Rc<RefCell<World>>>;
}

macro_rules! impl_prefpage {
    ($t:ident) => {
        impl PrefPage for $t {
            fn get_page(&self) -> QPtr<QWidget> {
                unsafe { self.ui.widget.static_upcast() }
            }
            fn upgrade_world(&self) -> Option<Rc<RefCell<World>>> {
                self.world.upgrade()
            }
        }
    };
}

trait PrefPageNew: 'static + PrefPage {
    fn new<P: CastInto<Ptr<QWidget>>>(parent: P, world: Weak<RefCell<World>>) -> Rc<Self>;

    fn connect<T, Q, F>(self: &Rc<Self>, field: QPtr<Q>, getter: F)
    where
        T: 'static,
        Q: 'static + QForm<T>,
        F: 'static + Fn(&mut World) -> &mut T,
    {
        let this = Rc::downgrade(self);
        unsafe {
            QForm::connect_rust(
                field.clone(),
                getter(&mut self.upgrade_world().unwrap().borrow_mut()),
                SlotNoArgs::new(self.get_page(), move || {
                    let this = match Weak::upgrade(&this) {
                        Some(this) => this,
                        None => return,
                    };
                    let world = match this.upgrade_world() {
                        Some(world) => world,
                        None => return,
                    };
                    *getter(&mut world.borrow_mut()) = QForm::get_rust(field.clone());
                }),
            );
        }
    }

    fn connect_font(
        self: &Rc<Self>,
        fontfield: QPtr<QFontComboBox>,
        sizefield: QPtr<QSpinBox>,
        getter: fn(&mut World) -> &mut RFont,
    ) {
        unsafe {
            let this = Rc::downgrade(self);
            QForm::connect_rust(
                fontfield.clone(),
                &getter(&mut self.upgrade_world().unwrap().borrow_mut()),
                SlotNoArgs::new(self.get_page(), move || {
                    let this = match Weak::upgrade(&this) {
                        Some(this) => this,
                        None => return,
                    };
                    let world = match this.upgrade_world() {
                        Some(world) => world,
                        None => return,
                    };
                    getter(&mut world.borrow_mut())
                        .set_family(&QForm::get_rust(fontfield.clone()).family());
                }),
            )
        }
        unsafe {
            let this = Rc::downgrade(self);
            QForm::connect_rust(
                sizefield.clone(),
                &getter(&mut self.upgrade_world().unwrap().borrow_mut()).size(),
                SlotNoArgs::new(self.get_page(), move || {
                    let this = match Weak::upgrade(&this) {
                        Some(this) => this,
                        None => return,
                    };
                    let world = match this.upgrade_world() {
                        Some(world) => world,
                        None => return,
                    };
                    getter(&mut world.borrow_mut()).set_size(QForm::get_rust(sizefield.clone()));
                }),
            )
        }
    }

    fn enable_if<const N: usize>(
        &self,
        checkbox: &QPtr<QCheckBox>,
        enabled: bool,
        fields: [QPtr<QWidget>; N], // already pointers into memory
    ) {
        unsafe {
            let should_enable_initial = checkbox.is_checked() == enabled;
            for field in &fields {
                field.set_enabled(should_enable_initial);
            }
            checkbox
                .state_changed()
                .connect(&SlotOfInt::new(self.get_page(), move |state| {
                    let should_enable = (state == CheckState::Checked.to_int()) == enabled;
                    for field in &fields {
                        field.set_enabled(should_enable);
                    }
                }));
        }
    }
}

macro_rules! impl_prefpagenew {
    ($t:ident) => {
        impl PrefPageNew for $t {
            fn new<P: CastInto<Ptr<QWidget>>>(parent: P, world: Weak<RefCell<World>>) -> Rc<Self> {
                let this = Rc::new(Self {
                    ui: uic::$t::load(parent),
                    world,
                });
                this.init();
                this
            }
        }
    };
}

#[derive(RWidget, TrContext)]
pub struct WorldPrefs {
    ui: uic::WorldPrefs,
    world: Weak<RefCell<World>>,
    pages: HashMap<&'static str, Rc<dyn PrefPage>>,
    current: RefCell<Option<Ref<QWidget>>>,
}
impl RDialog<DialogCode> for WorldPrefs {
    fn exec(&self) -> DialogCode {
        unsafe {
            self.ui
                .settings_tree
                .set_current_item_1a(self.ui.settings_tree.top_level_item(0).child(0));
            self.ui.widget.exec().into()
        }
    }
}

impl WorldPrefs {
    pub fn new<P: CastInto<Ptr<QWidget>>>(parent: P, world: Weak<RefCell<World>>) -> Rc<Self> {
        let mut this = Self {
            ui: uic::WorldPrefs::load(parent),
            world,
            pages: HashMap::new(),
            current: RefCell::new(None),
        };
        this.addpage::<PrefsAddress>("IP address");
        this.addpage::<PrefsConnecting>("Connecting");
        this.addpage::<PrefsLogging>("Logging");
        this.addpage::<PrefsChat>("Chat");
        this.addpage::<PrefsOutput>("Output");
        this.addpage::<PrefsMxp>("MXP/Pueblo");
        this.addpage::<PrefsColor>("ANSI Colour");
        this.addpage::<PrefsCommands>("Commands");
        let this = Rc::new(this);
        this.init();
        this
    }

    fn addpage<P: PrefPageNew>(&mut self, key: &'static str) {
        self.pages
            .insert(key, P::new(&self.ui.widget, self.world.clone()));
    }

    #[rustfmt::skip]
    fn init(self: &Rc<Self>) {
        unsafe {
            for page in self.pages.values() {
                let page = page.get_page();
                page.set_visible(false);
                self.ui.contents.add_widget(page);
            }
            self.ui.settings_tree.expand_all();
            self.ui.settings_tree.current_item_changed().connect(&self.slot_choose_page());
            self.browse("IP address");
        }
    }

    pub fn browse(&self, page: &str) {
        let page: QPtr<QWidget> = match self.pages.get(page) {
            Some(page) => page.get_page(),
            None => return,
        };
        unsafe {
            if let Some(oldpage) = self.current.replace(page.as_ref()) {
                oldpage.set_visible(false);
            }
            page.set_visible(true);
        }
    }

    #[slot(SlotOfQTreeWidgetItem)]
    fn choose_page(&self, item: Ptr<QTreeWidgetItem>) {
        unsafe {
            if item.child_count() == 0 {
                self.browse(&item.text(0).to_std_string());
            }
        }
    }
}

macro_rules! connect_world_one {
    ($self:ident, $field:ident, $fieldname:ident$(, $subfield:ident)?) => {
        $self.connect(
            $self.ui.$fieldname.clone(),
            |world| &mut world.$field$(.$subfield)?
        )
    };
    ($self:ident, $field:ident) => {
        connect_world_one!($self, $field, $field)
    };
}

macro_rules! connect_world {
    ($self:ident, $($field:ident$(.$subfield:ident $fieldname:ident)?),+$(,)?) => {
        $(
            connect_world_one!($self, $field$(, $fieldname, $subfield)?);
        )+
    }
}
/*
macro_rules! connect_force {
    ($self: ident, $field: ident) => {
        connect_world!($self, $field, Force);
    };
}
*/

#[derive(Debug, RWidget, TrContext)]
struct PrefsProxy {
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
struct PrefsAddress {
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
struct PrefsConnecting {
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
struct PrefsLogging {
    ui: uic::PrefsLogging,
    world: Weak<RefCell<World>>,
}
impl_prefpage!(PrefsLogging);
impl_prefpagenew!(PrefsLogging);

const DEFAULT_PREAMBLE: &'static str = r#"<html>
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

const DEFAULT_POSTAMBLE: &'static str = r#"        </font>
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
struct PrefsChat {
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

#[derive(Debug, RWidget, TrContext)]
struct PrefsOutput {
    ui: uic::PrefsOutput,
    world: Weak<RefCell<World>>,
}
impl_prefpage!(PrefsOutput);
impl_prefpagenew!(PrefsOutput);

impl PrefsOutput {
    fn init(self: &Rc<Self>) {
        connect_world!(
            self,
            beep_sound,
            pixel_offset,
            line_spacing,
            use_default_output_font,
            show_bold,
            show_italic,
            show_underline,
            new_activity_sound,
            max_output_lines,
            wrap_column,
            line_information,
            start_paused,
            auto_pause,
            unpause_on_send,
            flash_taskbar_icon,
            disable_compression,
            indent_paras,
            naws,
            carriage_return_clears_line,
            utf_8,
            auto_wrap_window_width,
            show_connect_disconnect,
            copy_selection_to_clipboard,
            auto_copy_to_clipboard_in_html,
            convert_ga_to_newline,
            terminal_identification,
        );
        self.connect_font(
            self.ui.output_font.clone(),
            self.ui.output_font_size.clone(),
            |world| &mut world.output_font,
        );
        unsafe {
            self.enable_if(
                &self.ui.use_default_output_font,
                false,
                [
                    self.ui.output_font.static_upcast(),
                    self.ui.output_font_size.static_upcast(),
                ],
            );
            self.enable_if(
                &self.ui.copy_selection_to_clipboard,
                true,
                [self.ui.auto_copy_to_clipboard_in_html.static_upcast()],
            );
        }
    }
}

#[derive(Debug, RWidget, TrContext)]
struct PrefsMxp {
    ui: uic::PrefsMxp,
    world: Weak<RefCell<World>>,
}
impl_prefpage!(PrefsMxp);
impl_prefpagenew!(PrefsMxp);

impl PrefsMxp {
    fn init(self: &Rc<Self>) {
        connect_world!(
            self,
            use_mxp,
            detect_pueblo,
            hyperlink_color,
            use_custom_link_color,
            mud_can_change_link_color,
            underline_hyperlinks,
            mud_can_remove_underline,
            hyperlink_adds_to_command_history,
            echo_hyperlink_in_output_window,
            ignore_mxp_color_changes,
            send_mxp_afk_response,
            mud_can_change_options,
        );
    }
}

#[derive(Debug, RWidget, TrContext)]
struct PrefsColor {
    ui: uic::PrefsColor,
    world: Weak<RefCell<World>>,
    colorfields: [QPtr<QPushButton>; 16],
}
impl_prefpage!(PrefsColor);

impl PrefPageNew for PrefsColor {
    fn new<P: CastInto<Ptr<QWidget>>>(parent: P, world: Weak<RefCell<World>>) -> Rc<Self> {
        let ui = uic::PrefsColor::load(parent);
        let colorfields = [
            ui.color_0.clone(),
            ui.color_1.clone(),
            ui.color_2.clone(),
            ui.color_3.clone(),
            ui.color_4.clone(),
            ui.color_5.clone(),
            ui.color_6.clone(),
            ui.color_7.clone(),
            ui.color_8.clone(),
            ui.color_9.clone(),
            ui.color_10.clone(),
            ui.color_11.clone(),
            ui.color_12.clone(),
            ui.color_13.clone(),
            ui.color_14.clone(),
            ui.color_15.clone(),
        ];
        let this = Rc::new(Self {
            ui,
            world,
            colorfields,
        });
        this.init();
        this
    }
}

impl PrefsColor {
    #[rustfmt::skip]
    fn init(self: &Rc<Self>) {
        connect_world!(self, use_default_colors);
        self.setcolors(&self.world.upgrade().unwrap().borrow().ansi_colors);
        for (i, field) in self.colorfields.iter().enumerate() {
            self.connect(field.clone(), move |world| &mut world.ansi_colors[i]);
        }
        unsafe {
            self.ui.swap.clicked().connect(&self.slot_swap());
            self.ui.reset.clicked().connect(&self.slot_reset());
        }
    }

    fn setcolors(&self, colors: &[RColor]) {
        for (color, field) in colors.iter().zip(self.colorfields.iter()) {
            field.set_palette_color(ColorRole::Button, color);
            unsafe {
                field.repaint();
            }
        }
    }

    #[slot(SlotNoArgs)]
    fn swap(&self) {
        let world = self.world.upgrade().unwrap();
        let colors = &mut world.borrow_mut().ansi_colors;
        let half = colors.len() / 2;
        for i in 0..half {
            colors.swap(i, i + half);
        }
        self.setcolors(colors);
    }

    #[slot(SlotNoArgs)]
    fn reset(&self) {
        let colors = Colors::ansi16();
        self.setcolors(&colors);
        self.world.upgrade().unwrap().borrow_mut().ansi_colors = colors;
    }
}

#[derive(Debug, RWidget, TrContext)]
struct PrefsCommands {
    ui: uic::PrefsCommands,
    world: Weak<RefCell<World>>,
}
impl_prefpage!(PrefsCommands);
impl_prefpagenew!(PrefsCommands);

impl PrefsCommands {
    fn init(self: &Rc<Self>) {
        connect_world!(
            self,
            display_my_input,
            echo_colors.foreground echo_colors_foreground,
            echo_colors.background echo_colors_background,
            enable_speed_walk,
            speed_walk_prefix,
            speed_walk_filler,
            speed_walk_delay,
            enable_command_stack,
            command_stack_character,
            input_colors.foreground input_colors_foreground,
            input_colors.background input_colors_background,
            use_default_input_font,
            enable_spam_prevention,
            spam_line_count,
            spam_message,
        );
        self.connect_font(
            self.ui.input_font.clone(),
            self.ui.input_font_size.clone(),
            |world| &mut world.input_font,
        );
    }
}
