use super::uic;
use crate::binding::color::Colored;
use crate::binding::{QList, RWidget};
use crate::client::color::WorldColor;
use crate::client::Client;
use crate::constants::branding;
use crate::tr::TrContext;
use crate::world::World;
use cpp_core::{CastInto, Ptr};
use qt_core::{slot, FocusReason, QListOfInt, QPtr, QString, QUrl, SlotNoArgs};
use qt_gui::q_palette::ColorRole;
use qt_gui::q_text_cursor::MoveOperation;
use qt_gui::QDesktopServices;
use qt_network::q_abstract_socket::SocketError;
use qt_network::{QTcpSocket, SlotOfSocketError};
use qt_widgets::q_message_box::{ButtonRole, Icon, StandardButton};
use qt_widgets::*;
use std::cell::{self, RefCell};
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    fn init(&self) {
        unsafe {
            self.widget.set_focus_proxy(&self.input);
            self.widget.set_sizes(&QListOfInt::from_array([1, 30]));
            let handle = self.widget.handle(1);
            handle.set_background_role(ColorRole::Button);
            handle.set_auto_fill_background(true);
        }
    }
    fn colorify(&self, world: &World) {
        self.output
            .set_background_color(world.color(&WorldColor::BLACK));
        unsafe {
            self.output.document().set_default_font(&world.input_font);
            // QLineEdit requires a little coaxing to enable transparency
            self.input
                .set_frame(world.input_colors.background.alpha() == 255);
            self.input.set_style_sheet(&world.input_colors.stylesheet());
        }
    }
}

#[derive(RWidget, TrContext)]
pub struct WorldTab {
    pub ui: uic::WorldTab,
    pub client: RefCell<Client>,
    pub saved: RefCell<Option<String>>,
    world: RefCell<Rc<World>>,
}

impl WorldTab {
    pub fn new<P: CastInto<Ptr<QWidget>>>(
        parent: P,
        world: World,
        saved: Option<String>,
    ) -> Rc<Self> {
        let ui = uic::WorldTab::load(parent);
        ui.init();
        ui.colorify(&world);
        let world = Rc::new(world);
        let socketbox = unsafe { QTcpSocket::new_1a(&ui.widget) };
        let socket = unsafe { socketbox.static_upcast() };
        let client = unsafe { Client::new(ui.output.clone(), socketbox, world.clone()) };

        let this = Rc::new(Self {
            client: RefCell::new(client),
            saved: RefCell::new(saved),
            ui,
            world: RefCell::new(world),
        });
        this.init(socket);
        this
    }

    #[rustfmt::skip]
    fn init(self: &Rc<Self>, socket: QPtr<QTcpSocket>) {
        unsafe {
            socket.error_occurred().connect(&self.slot_socket_error());
            socket.ready_read().connect(&self.slot_receive());
            self.ui.output.set_read_only(true);
            self.ui.input.return_pressed().connect(&self.slot_send());
            self.ui.input.editing_finished().connect(&self.slot_deselect());
            self.ui.input.selection_changed().connect(&self.slot_input_selected());
            self.ui.output.selection_changed().connect(&self.slot_output_selected());
        }
    }

    pub fn selection_mode(&self) -> SelectionMode {
        unsafe {
            if self.ui.input.has_selected_text() {
                SelectionMode::Input
            } else if self.ui.output.text_cursor().has_selection() {
                SelectionMode::Output
            } else {
                SelectionMode::Neither
            }
        }
    }

    pub fn connect_selection_changed<F: FnMut(SelectionMode) + 'static>(&self, mut f: F) {
        let input = self.ui.input.clone();
        let output = self.ui.output.clone();
        let mut mode = SelectionMode::get_current(&input, &output);
        unsafe {
            let slot = SlotNoArgs::new(self.widget(), move || {
                let new_mode = SelectionMode::get_current(&input, &output);
                if new_mode != mode {
                    f(new_mode);
                    mode = new_mode;
                }
            });
            self.ui.input.selection_changed().connect(&slot);
            self.ui.output.selection_changed().connect(&slot);
        }
    }

    pub fn set_world(&self, world: World) {
        self.ui.colorify(&world);
        let world = Rc::new(world);
        self.client.borrow_mut().set_world(world.clone());
        self.world.replace(world);
    }
    pub fn borrow_world(&self) -> cell::Ref<Rc<World>> {
        self.world.borrow()
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
        self.client.borrow_mut().println(msg);
    }

    #[slot(SlotNoArgs)]
    fn report_bug(&self) {
        unsafe {
            QDesktopServices::open_url(&QUrl::new_1a(&QString::from_std_str(&format!(
                "{}/issues",
                branding::REPO
            ))));
        }
    }

    #[slot(SlotNoArgs)]
    fn deselect(&self) {
        unsafe {
            self.ui.input.deselect();
        }
    }
    #[slot(SlotNoArgs)]
    fn input_selected(&self) {
        unsafe {
            if self.ui.input.has_selected_text() {
                self.ui.output.move_cursor_1a(MoveOperation::End);
            }
        }
    }
    #[slot(SlotNoArgs)]
    fn output_selected(&self) {
        unsafe {
            if self.ui.output.text_cursor().has_selection() {
                self.ui.input.deselect();
                //self.ui.input.set_focus_policy(FocusPolicy::StrongFocus);
                self.ui.output.set_focus_1a(FocusReason::MouseFocusReason);
            } else {
                self.ui.input.set_focus_1a(FocusReason::MouseFocusReason);
            }
        }
    }

    #[slot(SlotNoArgs)]
    fn receive(&self) {
        self.client.borrow_mut().read();
    }

    #[slot(SlotNoArgs)]
    fn send(&self) {
        let input = unsafe { self.ui.input.text() }.to_std_string();
        unsafe {
            self.ui.input.clear();
        }
        if let Err(e) = self.client.borrow_mut().send_command(input) {
            eprintln!("Failed to send data: {}", e); // will be handled in GUI by socket_error()
        }
    }
}
