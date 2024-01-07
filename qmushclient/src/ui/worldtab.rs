use std::cell::{self, RefCell};
use std::os::raw::c_double;
use std::rc::Rc;

use cpp_core::{CastInto, CppBox, Ptr, Ref};
use enumeration::Enum;
use qt::text::RTextCursor;
use qt::widgets::{RLineEdit, RTextBrowser};
use qt::{Printable, QList, RColor, Widget};
use qt_core::{
    slot, FocusReason, GlobalColor, QBox, QListOfInt, QPoint, QPtr, QString, QUrl, SlotNoArgs,
};
use qt_gui::q_palette::ColorRole;
use qt_gui::q_text_block_format::LineHeightTypes;
use qt_gui::q_text_cursor::{MoveMode, MoveOperation};
use qt_gui::{QDesktopServices, QTextBlockFormat, QTextDocument, SlotOfQUrl};
use qt_network::q_abstract_socket::SocketError;
use qt_network::{QTcpSocket, SlotOfSocketError};
use qt_widgets::q_message_box::{ButtonRole, Icon, StandardButton};
use qt_widgets::*;
use tr::TrContext;
use uuid::Uuid;

use super::uic;
use crate::client::color::WorldColor;
use crate::client::Client;
use crate::constants::{branding, Paths};
use crate::mxp::SendTo;
use crate::script::SendRequest;
use crate::world::World;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum SelectionMode {
    Neither,
    Input,
    Output,
}

impl SelectionMode {
    fn get_current(input: &QPtr<QLineEdit>, output: &QPtr<QTextBrowser>) -> Self {
        unsafe {
            if input.has_selected_text() {
                Self::Input
            } else if output.text_cursor().has_selection() {
                Self::Output
            } else {
                Self::Neither
            }
        }
    }
}

impl uic::WorldTab {
    fn style(&self, world: &World) {
        unsafe {
            self.output.set_style_sheet(&QString::from_std_str(format!(
                r"QTextBrowser {{
                    color: {fg};
                    background-color: {bg};
                    font: {font};
                    line-height: {spacing};
                }}",
                fg = world.color(&WorldColor::WHITE),
                bg = world.color(&WorldColor::BLACK),
                font = world.output_font,
                spacing = world.line_spacing,
            )));
            self.input.set_font(&world.input_font);
            // QLineEdit requires a little coaxing to enable transparency
            self.input
                .set_frame(world.input_colors.background.alpha() == 255);
            self.input.set_style_sheet(&world.input_colors.stylesheet());
        }
    }

    fn set_spacing(&self, spacing: c_double) {
        unsafe {
            let cursor = self.output.cursor_for_position(&QPoint::new_0a());
            while cursor.move_position_2a(MoveOperation::NextBlock, MoveMode::KeepAnchor) {}
            let fmt = QTextBlockFormat::new();
            fmt.set_line_height(
                spacing * 100.0,
                LineHeightTypes::ProportionalHeight.to_int(),
            );
            cursor.merge_block_format(&fmt);
        }
    }
}

#[derive(Widget, TrContext)]
pub struct WorldTab {
    pub ui: uic::WorldTab,
    pub client: RefCell<Client>,
    pub saved: RefCell<Option<String>>,
    pub notepad: QBox<QTextDocument>,
    pub notepad_title: CppBox<QString>,
    pub socket: QPtr<QTcpSocket>,
    world: RefCell<Rc<World>>,
    address: RefCell<String>,
    cursor: RTextCursor,
}

impl WorldTab {
    pub fn new<P>(parent: P, world: World, saved: Option<String>, paths: &'static Paths) -> Rc<Self>
    where
        P: CastInto<Ptr<QWidget>>,
    {
        let ui = uic::WorldTab::load(parent);
        ui.style(&world);
        ui.set_spacing(world.line_spacing as c_double);
        let notepad_title = tr!("World Notepad - {}", world.name);
        let world = Rc::new(world);
        unsafe {
            let socketbox = QTcpSocket::new_1a(&ui.widget);
            let socket = socketbox.static_upcast();
            let client = Client::new(
                RTextBrowser::new(ui.output.clone()),
                RLineEdit::new(ui.input.clone()),
                socketbox,
                world.clone(),
                paths,
            );
            let cursor = RTextCursor::get(&ui.output);
            let this = Rc::new(Self {
                client: RefCell::new(client),
                saved: RefCell::new(saved),
                notepad: QTextDocument::from_q_object(&ui.widget),
                notepad_title,
                ui,
                world: RefCell::new(world),
                socket,
                address: RefCell::new(String::new()),
                cursor,
            });
            this.client
                .borrow_mut()
                .plugins
                .set_event_handler(Rc::downgrade(&this));
            this.init();
            this
        }
    }

    /// # Safety:
    ///
    /// `socket` must be valid.
    #[rustfmt::skip]
    unsafe fn init(self: &Rc<Self>) {
        unsafe {
            let ui = &self.ui;
            ui.widget.set_focus_proxy(&ui.input);
            ui.widget.set_sizes(&QListOfInt::from_array([1, 30]));
            let handle = ui.widget.handle(1);
            handle.set_background_role(ColorRole::Button);
            handle.set_auto_fill_background(true);
            ui.output.set_read_only(true);
            ui.output.set_open_links(false);
            ui.output.custom_context_menu_requested().connect(&self.slot_menu_open());
            ui.output.anchor_clicked().connect(&self.slot_anchor_clicked());
            ui.output.selection_changed().connect(&self.slot_output_selected());
            ui.input.selection_changed().connect(&self.slot_input_selected());
            ui.input.return_pressed().connect(&self.slot_send());
            ui.input.editing_finished().connect(&self.slot_deselect());
            self.socket.error_occurred().connect(&self.slot_socket_error());
            self.socket.ready_read().connect(&self.slot_receive());
            self.socket.connected().connect(&self.slot_report_connected());
            self.socket.disconnected().connect(&self.slot_report_disconnected());
        }
    }

    pub fn selection_mode(&self) -> SelectionMode {
        let ui = &self.ui;
        unsafe {
            if ui.input.has_selected_text() {
                SelectionMode::Input
            } else if ui.output.text_cursor().has_selection() {
                SelectionMode::Output
            } else {
                SelectionMode::Neither
            }
        }
    }

    pub fn connect_selection_changed<F: FnMut(SelectionMode) + 'static>(&self, mut f: F) {
        let ui = &self.ui;
        let input = ui.input.clone();
        let output = ui.output.clone();
        let mut mode = SelectionMode::get_current(&input, &output);
        unsafe {
            let slot = SlotNoArgs::new(self.widget(), move || {
                let new_mode = SelectionMode::get_current(&input, &output);
                if new_mode != mode {
                    f(new_mode);
                    mode = new_mode;
                }
            });
            ui.input.selection_changed().connect(&slot);
            ui.output.selection_changed().connect(&slot);
        }
    }

    pub fn set_world(&self, world: World) {
        let newspacing = world.line_spacing as c_double;
        let newtitle = world.name.clone();
        self.ui.style(&world);
        let world = Rc::new(world);
        self.client.borrow_mut().set_world(world.clone());
        let oldworldrc = self.world.replace(world);
        let oldworld = &*oldworldrc;
        if (oldworld.line_spacing as c_double - newspacing).abs() > 0.009 {
            self.ui.set_spacing(newspacing);
            self.client.borrow_mut().set_spacing(newspacing);
        }
        if oldworld.name != newtitle {
            unsafe {
                self.notepad_title
                    .swap(&tr!("World Notepad - {}", newtitle));
            }
            self.client.borrow_mut().retitle(&newtitle);
        }
    }
    pub fn borrow_world(&self) -> cell::Ref<Rc<World>> {
        self.world.borrow()
    }

    pub fn command(&self, cmd: String) {
        if let Err(e) = self.client.borrow_mut().send_command(cmd) {
            eprintln!("Failed to send data: {}", e); // will be handled in GUI by socket_error()
        }
    }

    pub fn trigger_timer(&self, id: Uuid, request: SendRequest) {
        if let Err(e) = self.client.borrow_mut().plugins.trigger_timer(id, request) {
            eprintln!("Timer failed: {}", e);
        }
    }

    pub fn notify<S: Printable, Fg: Into<RColor>>(&self, text: S, fg: Fg) {
        if !self.cursor.at_block_start() {
            self.cursor.insert_block();
        }
        self.cursor.insert_text_colored(text, Some(fg.into()), None);
        self.cursor.insert_block();
    }

    #[slot(SlotOfSocketError)]
    fn socket_error(self: &Rc<Self>, error: SocketError) {
        let msg = match error {
            SocketError::ConnectionRefusedError => {
                tr!("Connection failed: server refused connection.")
            }
            SocketError::RemoteHostClosedError => {
                tr!("Connection closed by server.")
            }
            SocketError::HostNotFoundError => {
                tr!("Connection failed: site address is unreachable.")
            }
            SocketError::SocketAccessError => {
                tr!("Connection failed: you do not have permission to use sockets.")
            }
            SocketError::SocketResourceError => {
                tr!("Connection failed: insufficient system resources.")
            }
            SocketError::SocketTimeoutError => {
                tr!("Connection timed out.")
            }
            SocketError::NetworkError => {
                tr!("Connection failed: network error. Check your Internet connection.")
            }
            SocketError::UnsupportedSocketOperationError => {
                tr!("Connection failed: network operation is unsupported.")
            }
            SocketError::ProxyAuthenticationRequiredError => {
                tr!("Connection failed: proxy server requires authentication.")
            }
            SocketError::ProxyConnectionClosedError => {
                tr!("Connection closed by proxy server.")
            }
            SocketError::ProxyNotFoundError => {
                tr!("Connection failed: proxy address is unreachable.")
            }
            SocketError::ProxyProtocolError => {
                tr!("Received an invalid response from the proxy server.")
            }
            _ => unsafe {
                let msgbox = QMessageBox::from_q_widget(self.widget());
                msgbox.set_icon(Icon::Critical);
                msgbox.set_text(&tr!("Encountered an unexpected network error!"));
                msgbox.set_text(&tr!("Error code: {}", error.to_int()));
                msgbox
                    .add_button_q_string_button_role(&tr!("Report Bug"), ButtonRole::HelpRole)
                    .clicked()
                    .connect(&self.slot_report_bug());
                msgbox.add_button_standard_button(StandardButton::Close);
                msgbox.set_default_button_standard_button(StandardButton::Close);
                msgbox.exec();
                return;
            },
        };
        self.notify(msg, GlobalColor::Red);
    }

    #[slot(SlotNoArgs)]
    fn report_bug(&self) {
        open_link(&QString::from_std_str(format!("{}/issues", branding::REPO)));
    }

    #[slot(SlotNoArgs)]
    fn deselect(&self) {
        unsafe {
            self.ui.input.deselect();
        }
    }
    #[slot(SlotNoArgs)]
    fn input_selected(&self) {
        let ui = &self.ui;
        unsafe {
            if ui.input.has_selected_text() {
                ui.output.move_cursor_1a(MoveOperation::End);
            }
        }
    }
    #[slot(SlotNoArgs)]
    fn output_selected(&self) {
        let ui = &self.ui;
        unsafe {
            if ui.output.text_cursor().has_selection() {
                ui.input.deselect();
                ui.output.set_focus_1a(FocusReason::MouseFocusReason);
            } else {
                ui.input.set_focus_1a(FocusReason::MouseFocusReason);
            }
        }
    }

    #[slot(SlotNoArgs)]
    fn receive(&self) {
        if let Err(e) = self.client.borrow_mut().read() {
            eprintln!("Failed to read data: {}", e); // will be handled in GUI by socket_error()
        }
    }

    #[slot(SlotNoArgs)]
    fn report_connected(&self) {
        let address = unsafe { self.socket.peer_address().to_string().to_std_string() };
        self.notify(tr!("Connected to {}", address), GlobalColor::Gray);
        self.address.replace(address);
    }

    #[slot(SlotNoArgs)]
    fn report_disconnected(&self) {
        self.notify(
            tr!("Disconnected from {}", self.address.borrow()),
            GlobalColor::Gray,
        );
    }

    #[slot(SlotNoArgs)]
    fn send(&self) {
        let ui = &self.ui;
        unsafe {
            let input = ui.input.text().to_std_string();
            ui.input.clear();
            self.command(input);
        }
    }

    #[slot(SlotOfQPoint)]
    fn menu_open(&self, point: Ref<QPoint>) {
        let ui = &self.ui;
        unsafe {
            let format = ui.output.cursor_for_position(point).char_format();
            let point = ui.output.map_to_global(point);
            let anchor_names = format.anchor_names();
            if anchor_names.is_empty() {
                ui.output
                    .create_standard_context_menu_1a(&point)
                    .exec_1a_mut(&point);
                return;
            }
            let menu = QMenu::from_q_widget(&ui.output);
            for anchor_name in anchor_names.iter() {
                menu.add_action_q_string(anchor_name);
            }
            let chosen = menu.exec_1a_mut(&point);
            if chosen.is_null() {
                return;
            }
            if format.is_anchor() {
                open_link(&chosen.text())
            } else {
                ui.input.set_text(&chosen.text())
            };
        }
    }

    #[slot(SlotOfQUrl)]
    fn anchor_clicked(&self, url: Ref<QUrl>) {
        let qtext = unsafe { url.to_string_0a() };
        let text = qtext.to_std_string();
        let (sendto, body) = SendTo::detach(&text);
        match sendto {
            SendTo::Input => unsafe { self.ui.input.set_text(&QString::from_std_str(body)) },
            SendTo::Internet => open_link(&QString::from_std_str(body)),
            SendTo::World => self.command(body.to_owned()),
        }
    }
}

fn open_link(link: &CppBox<QString>) {
    unsafe {
        QDesktopServices::open_url(&QUrl::new_1a(link));
    }
}
