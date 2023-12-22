use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::os::raw::c_int;
use std::path::Path;
use std::rc::Rc;

use cpp_core::{CppBox, NullPtr};
use qt_bindings::{RSettings, Widget};
use qt_core::{
    slot, FocusReason, QBox, QCoreApplication, QPtr, QString, SlotNoArgs, SlotOfBool, SlotOfInt,
};
use qt_gui::QTextDocument;
use qt_network::q_abstract_socket::SocketState;
use qt_network::SlotOfSocketState;
use qt_widgets::q_dialog::DialogCode;
use qt_widgets::q_message_box::Icon;
use qt_widgets::*;
use tr::TrContext;

use super::uic;
use super::worldprefs::WorldPrefs;
use super::worldtab::{SelectionMode, WorldTab};
use crate::constants::{config, Paths};
use crate::persist;
use crate::world::World;

const KEY_RECENT: &str = "recent";
fn only_filename(s: &str) -> &str {
    Path::new(s)
        .file_name()
        .and_then(|x| x.to_str())
        .unwrap_or("")
}

#[derive(Widget, TrContext)]
pub struct App {
    ui: uic::App,
    notepad: QBox<QTextEdit>,
    notepad_title: CppBox<QString>,
    blank: QBox<QTextDocument>,
    save_filter: CppBox<QString>,
    recent: RefCell<VecDeque<String>>,
    world_tabs: RefCell<Vec<Rc<WorldTab>>>,
    current: Cell<Option<usize>>,
    current_input: RefCell<QPtr<QLineEdit>>,
    settings: RSettings,
    paths: &'static Paths,
}

impl App {
    pub fn new() -> Rc<Self> {
        let settings = RSettings::default();
        let recent = settings
            .get_list(KEY_RECENT)
            .unwrap_or_else(|_| VecDeque::new());

        let paths = Paths::new(&settings).expect("error locating home directory");
        paths.ensure().expect("error creating working directories");

        let ui = uic::App::load(NullPtr);

        unsafe {
            let this = Rc::new(App {
                blank: QTextDocument::from_q_object(&ui.widget),
                ui,
                notepad: QTextEdit::new(),
                notepad_title: tr!("World Notebook"),
                save_filter: tr!("World files (*.qmc);;All Files (*.*)"),
                recent: RefCell::new(recent),
                world_tabs: RefCell::new(Vec::new()),
                current: Cell::new(None),
                current_input: RefCell::new(QPtr::null()),
                settings,
                paths: Box::leak(Box::new(paths)),
            });
            this.init();
            this
        }
    }

    #[rustfmt::skip]
    fn init(self: &Rc<Self>) {
        unsafe {
            self.notepad.hide();
            self.notepad.set_enabled(false);
            self.notepad.set_window_title(&self.notepad_title);

            let ui = &self.ui;
            ui.world_tabs.current_changed().connect(&self.slot_current_changed());
            ui.world_tabs.tab_close_requested().connect(&self.slot_close_world());
            ui.action_new.triggered().connect(&self.slot_new_world());
            ui.action_open_world.triggered().connect(&self.slot_open_world());
            ui.action_open_worlds_in_startup_list.triggered().connect(&self.slot_open_all());
            ui.action_close_world.triggered().connect(&self.slot_close_current_world());
            ui.action_save_world_details.triggered().connect(&self.slot_save_world());
            ui.action_save_world_details_as.triggered().connect(&self.slot_save_world_as());
            ui.action_world_properties.triggered().connect(&self.slot_open_preferences());
            ui.action_exit.triggered().connect(&self.slot_exit());

            ui.action_undo.triggered().connect(&self.slot_undo());
            ui.action_redo.triggered().connect(&self.slot_redo());
            ui.action_cut.triggered().connect(&self.slot_cut());
            ui.action_copy.triggered().connect(&self.slot_copy());
            ui.action_paste.triggered().connect(&self.slot_paste());
            ui.action_select_all.triggered().connect(&self.slot_select_all());

            ui.action_world_notepad.toggled().connect(&self.slot_toggle_notepad());

            ui.action_connect.triggered().connect(&self.slot_connect());
            ui.action_disconnect.triggered().connect(&self.slot_disconnect());

            self.recent.borrow_mut().truncate(config::MAX_RECENT);
            self.setup_recents();
        }
    }

    fn title_from_file(&self, path: &str) {
        unsafe {
            self.ui
                .widget
                .set_window_title(&QString::from_std_str(only_filename(path)));
        }
    }

    fn setup_recents(self: &Rc<Self>) {
        let ui = &self.ui;
        for (i, (button, path)) in [
            &ui.action_rec_1,
            &ui.action_rec_2,
            &ui.action_rec_3,
            &ui.action_rec_4,
            &ui.action_rec_5,
        ]
        .iter()
        .zip(self.recent.borrow().iter().map(ToOwned::to_owned))
        .enumerate()
        {
            unsafe {
                button.set_enabled(true);
                let filename = only_filename(&path);
                button.set_text(&QString::from_std_str(format!("&{} {}", i + 1, filename)));
                button.set_visible(true);
                let this = Rc::downgrade(self);
                button.disconnect();
                button
                    .triggered()
                    .connect(&SlotNoArgs::new(self.widget(), move || {
                        if let Some(this) = this.upgrade() {
                            this.open_world_file(&path);
                        }
                    }));
            }
        }
    }

    fn save_recents(self: &Rc<Self>) {
        self.recent.borrow_mut().truncate(config::MAX_RECENT);
        self.settings.set_list(KEY_RECENT, &*self.recent.borrow());
        self.setup_recents();
    }

    fn open_world_file(self: &Rc<Self>, path: &str) {
        self.recent.borrow_mut().retain(|x| x != path);
        use persist::Error;
        match persist::load_world(path) {
            Ok(world) => {
                self.title_from_file(path);
                self.recent.borrow_mut().push_front(path.to_owned());
                self.start_world(world, Some(path.to_owned()));
            }
            Err(Error::File(e)) => self.alert(Icon::Critical, tr!("Couldn't open the file"), &e),
            Err(Error::Serial(e)) => {
                self.alert(Icon::Critical, tr!("Failed to decode the file"), &e)
            }
            Err(Error::NotSave) => {
                self.alert(Icon::Critical, tr!("Incorrect file type"), &Error::NotSave)
            }
        }

        self.save_recents();
    }

    fn set_world_menus_enabled(&self, enabled: bool) {
        let ui = &self.ui;
        for &action in &[
            &ui.action_close_world,
            &ui.action_import,
            &ui.action_plugins,
            &ui.action_plugin_wizard,
            &ui.action_save_world_details,
            &ui.action_save_world_details_as,
            &ui.action_print,
            &ui.action_log_session,
            &ui.action_reload_defaults,
            &ui.action_world_properties,
            &ui.action_undo,
            &ui.action_paste,
            &ui.action_paste_to_world,
            &ui.action_recall_last_word,
            &ui.action_select_all,
            &ui.action_debug_packets,
            &ui.action_go_to_matching_brace,
            &ui.action_select_to_matching_brace,
        ] {
            unsafe {
                action.set_enabled(enabled);
            }
        }
    }

    fn start_world(self: &Rc<Self>, world: World, filename: Option<String>) {
        let ui = &self.ui;
        let tabname = QString::from_std_str(world.name.replace('&', ""));
        let should_open = world.connect_method.is_some();
        let worldtab = WorldTab::new(self.widget(), world, filename, self.paths);
        if should_open {
            worldtab.client.borrow_mut().connect();
        }
        let new_index = self.world_tabs.borrow().len();
        unsafe {
            self.update_socket_state(worldtab.socket.state());
            let this = Rc::downgrade(self);
            worldtab
                .socket
                .state_changed()
                .connect(&SlotOfSocketState::new(&ui.widget, move |state| {
                    if let Some(this) = this.upgrade() {
                        if this.current.get() == Some(new_index) {
                            this.update_socket_state(state);
                        }
                    }
                }));
        }
        let tab = worldtab.widget();
        let inputfield = worldtab.ui.input.clone();
        let this = Rc::downgrade(self);
        worldtab.connect_selection_changed(move |mode| {
            if let Some(this) = this.upgrade() {
                this.selection_changed(mode);
            }
        });
        self.world_tabs.borrow_mut().push(worldtab);
        unsafe {
            ui.world_tabs.add_tab_2a(tab, &tabname);
            ui.world_tabs.set_current_index(new_index as c_int);
            inputfield.set_focus_1a(FocusReason::ActiveWindowFocusReason);
        }
    }

    fn current_tab(&self) -> Option<Rc<WorldTab>> {
        let current = self.current.get()?;
        Some(self.world_tabs.borrow().get(current)?.clone())
    }

    fn save_world_file(self: &Rc<Self>, force_different: bool) {
        let tab = match self.current_tab() {
            Some(tab) => tab,
            None => return,
        };
        let world = tab.borrow_world();
        let save_as = match &*tab.saved.borrow() {
            Some(save_as) if !force_different => save_as.to_owned(),
            _ => unsafe {
                QFileDialog::get_save_file_name_4a(
                    self.widget(),
                    &tr!("Save as"),
                    &QString::from_std_str(self.paths.worlds.join(&world.name).to_string_lossy()),
                    &self.save_filter,
                )
                .to_std_string()
            },
        };
        if save_as.is_empty() {
            return;
        }

        use persist::Error;
        match persist::save_world(&world, &save_as) {
            Err(Error::File(e)) => {
                self.alert(Icon::Critical, tr!("Couldn't save the file"), &e);
                return;
            }
            Err(Error::Serial(e)) => {
                self.alert(Icon::Critical, tr!("Failed to encode the file"), &e);
                return;
            }
            _ => (),
        }

        self.title_from_file(&save_as);
        tab.saved.replace(Some(save_as.clone()));
        {
            let mut recents = self.recent.borrow_mut();
            recents.retain(|x| x != &save_as);
            recents.push_front(save_as);
        }
        self.save_recents();
        tab.client.borrow_mut().on_save();
    }

    unsafe fn with_input<O: Default>(&self, f: unsafe fn(&QLineEdit) -> O) -> O {
        unsafe {
            match self.current_input.borrow().as_raw_ref() {
                Some(input) => f(input),
                None => O::default(),
            }
        }
    }

    fn update_socket_state(&self, state: SocketState) {
        let ui = &self.ui;
        unsafe {
            ui.action_connect
                .set_enabled(state == SocketState::UnconnectedState);
            ui.action_disconnect
                .set_enabled(state == SocketState::ConnectedState);
        }
    }

    fn selection_changed(&self, mode: SelectionMode) {
        let ui = &self.ui;
        unsafe {
            ui.action_cut.set_enabled(mode == SelectionMode::Input);
            ui.action_copy.set_enabled(mode != SelectionMode::Neither);
            ui.action_copy_as_html
                .set_enabled(mode == SelectionMode::Output);
        }
    }

    #[slot(SlotOfInt)]
    fn current_changed(&self, index: c_int) {
        self.current.replace(usize::try_from(index).ok());
        let ui = &self.ui;
        match self.current_tab() {
            None => {
                self.set_world_menus_enabled(false);
                unsafe {
                    self.notepad.set_document(&self.blank);
                    self.notepad.set_enabled(false);
                    self.notepad.set_window_title(&self.notepad_title);
                    self.current_input.replace(QPtr::null());
                    for action in &[
                        &ui.action_cut,
                        &ui.action_copy,
                        &ui.action_copy_as_html,
                        &ui.action_redo,
                        &ui.action_connect,
                        &ui.action_disconnect,
                    ] {
                        action.set_enabled(false);
                    }
                }
            }
            Some(tab) => {
                self.set_world_menus_enabled(true);
                self.current_input.replace(tab.ui.input.clone());
                unsafe {
                    ui.action_redo.set_enabled(tab.ui.input.is_redo_available());
                    self.update_socket_state(tab.socket.state());
                    self.selection_changed(tab.selection_mode());
                    self.notepad.set_document(&tab.notepad);
                    self.notepad.set_enabled(true);
                    self.notepad.set_window_title(&tab.notepad_title);
                }
            }
        }
    }

    #[slot(SlotNoArgs)]
    fn connect(&self) {
        if let Some(current) = self.current_tab() {
            current.client.borrow_mut().connect();
        }
    }

    #[slot(SlotNoArgs)]
    fn disconnect(&self) {
        if let Some(current) = self.current_tab() {
            current.client.borrow_mut().disconnect();
        }
    }

    #[slot(SlotNoArgs)]
    fn new_world(self: &Rc<Self>) {
        let mut world = World::new();
        world.chat_name = "Name-not-set".to_string();
        let world = Rc::new(RefCell::new(world));
        let prefs = WorldPrefs::new(self.widget(), Rc::downgrade(&world));
        if prefs.exec() == DialogCode::Accepted {
            self.start_world(Rc::try_unwrap(world).unwrap().into_inner(), None);
        }
    }

    #[slot(SlotNoArgs)]
    fn open_world(self: &Rc<Self>) {
        let filename = unsafe {
            QFileDialog::get_open_file_name_4a(
                self.widget(),
                &tr!("Open world"),
                &QString::from_std_str(self.paths.worlds.to_string_lossy()),
                &self.save_filter,
            )
            .to_std_string()
        };
        if filename.is_empty() {
            return;
        }
        self.open_world_file(&filename)
    }

    #[slot(SlotNoArgs)]
    fn open_all(self: &Rc<Self>) {
        if let Ok(list) = self.settings.get_list::<Vec<String>>("startuplist") {
            for filename in list {
                self.open_world_file(&filename);
            }
        }
    }

    #[slot(SlotNoArgs)]
    fn save_world(self: &Rc<Self>) {
        self.save_world_file(false);
    }

    #[slot(SlotNoArgs)]
    fn save_world_as(self: &Rc<Self>) {
        self.save_world_file(true);
    }

    #[slot(SlotNoArgs)]
    fn close_current_world(&self) {
        unsafe { self.notepad.set_document(&self.blank) };
        self.close_world(self.current.get().unwrap() as c_int);
    }

    #[slot(SlotOfInt)]
    fn close_world(&self, i: c_int) {
        unsafe { self.notepad.set_document(&self.blank) };
        let tab = self.world_tabs.borrow_mut().remove(i as usize);
        tab.client.borrow_mut().disconnect();
    }

    #[slot(SlotNoArgs)]
    fn open_preferences(&self) {
        let tab = match self.current_tab() {
            Some(tab) => tab,
            None => return,
        };
        // We need to do a bit of dancing between Rcs and RefCells because the tab and its
        // descendants use an immutable World, but the preference pages require a mutable copy.
        // Furthermore, the tab is a Qt object, so it only tolerates interior mutability, which
        // means it needs to store its World in a RefCell. This limitation does not extend to its
        // children, i.e. the Client and the Api instances for each Plugin, which use bare Rcs.
        // All this indirection could be avoided by giving each entity its own copy of World, but
        // that would involve a lot of unnecessary cloning.
        let world: World = tab.borrow_world().as_ref().clone();
        let worldcell = Rc::new(RefCell::new(world));
        let prefs = WorldPrefs::new(self.widget(), Rc::downgrade(&worldcell));
        if prefs.exec() == DialogCode::Accepted {
            tab.set_world(Rc::try_unwrap(worldcell).unwrap().into_inner());
            unsafe {
                self.notepad.set_window_title(&tab.notepad_title);
            }
        }
    }

    #[slot(SlotNoArgs)]
    fn undo(&self) {
        unsafe {
            let can_redo = self.with_input(|input| {
                input.undo();
                input.is_redo_available()
            });
            self.ui.action_redo.set_enabled(can_redo);
        }
    }
    #[slot(SlotNoArgs)]
    fn redo(&self) {
        unsafe {
            let can_redo = self.with_input(|input| {
                input.redo();
                input.is_redo_available()
            });
            self.ui.action_redo.set_enabled(can_redo);
        }
    }
    #[slot(SlotNoArgs)]
    fn cut(&self) {
        unsafe { self.with_input(QLineEdit::cut) }
    }
    #[slot(SlotNoArgs)]
    fn copy(&self) {
        unsafe {
            match self.current_input.borrow().as_ref() {
                Some(input) if input.has_selected_text() => input.copy(),
                _ => self.current_tab().unwrap().ui.output.copy(),
            }
        }
    }
    #[slot(SlotNoArgs)]
    fn paste(&self) {
        unsafe { self.with_input(QLineEdit::paste) }
    }
    #[slot(SlotNoArgs)]
    fn select_all(&self) {
        unsafe { self.with_input(QLineEdit::select_all) }
    }

    #[slot(SlotOfBool)]
    fn toggle_notepad(&self, checked: bool) {
        unsafe {
            self.notepad.set_visible(checked);
        }
    }

    #[slot(SlotNoArgs)]
    fn exit(&self) {
        unsafe { QCoreApplication::quit() }
    }
}
