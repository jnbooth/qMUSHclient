use super::uic;
use super::worldprefs::WorldPrefs;
use super::worldtab::{SelectionMode, WorldTab};
use crate::binding::{RDialog, RSettings, RWidget};
use crate::persist;
use crate::tr::TrContext;
use crate::world::World;
use cpp_core::{CppBox, NullPtr};
use qt_core::{slot, FocusReason, QCoreApplication, QPtr, QString, SlotNoArgs, SlotOfInt};
use qt_widgets::q_dialog::DialogCode;
use qt_widgets::q_message_box::Icon;
use qt_widgets::*;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::os::raw::c_int;
use std::path::Path;
use std::rc::Rc;

const SAVE_DIR: &str = "/home/j/Downloads";

const MAX_RECENT: usize = 5;
const KEY_RECENT: &str = "recent";
fn only_filename(s: &str) -> &str {
    Path::new(s)
        .file_name()
        .and_then(|x| x.to_str())
        .unwrap_or("")
}

#[derive(RWidget, TrContext)]
pub struct App {
    ui: uic::App,
    save_dir: CppBox<QString>,
    save_filter: CppBox<QString>,
    recent: RefCell<VecDeque<String>>,
    world_tabs: RefCell<Vec<Rc<WorldTab>>>,
    current: Cell<Option<usize>>,
    current_input: RefCell<QPtr<QLineEdit>>,
    settings: RSettings,
}

impl App {
    pub fn new(organization: &str, name: &str) -> Rc<Self> {
        let settings = RSettings::new(organization, name);
        let recent = match settings.get_list(KEY_RECENT) {
            Ok(list) => list.collect(),
            Err(_) => VecDeque::new(),
        };

        let this = Rc::new(App {
            ui: uic::App::load(NullPtr),
            save_dir: QString::from_std_str(SAVE_DIR),
            save_filter: tr!("World files (*.qmc);;All Files (*.*)"),
            recent: RefCell::new(recent),
            world_tabs: RefCell::new(Vec::new()),
            current: Cell::new(None),
            current_input: RefCell::new(unsafe { QPtr::null() }),
            settings,
        });
        this.init();
        this
    }

    #[rustfmt::skip]
    fn init(self: &Rc<Self>) {
        unsafe {
            self.ui.world_tabs.current_changed().connect(&self.slot_current_changed());
            self.ui.world_tabs.tab_close_requested().connect(&self.slot_close_world());
            self.ui.action_new.triggered().connect(&self.slot_new_world());
            self.ui.action_open_world.triggered().connect(&self.slot_open_world());
            self.ui.action_close_world.triggered().connect(&self.slot_close_current_world());
            self.ui.action_save_world_details.triggered().connect(&self.slot_save_world());
            self.ui.action_save_world_details_as.triggered().connect(&self.slot_save_world_as());
            self.ui.action_world_properties.triggered().connect(&self.slot_open_preferences());
            self.ui.action_exit.triggered().connect(&self.slot_exit());

            self.ui.action_undo.triggered().connect(&self.slot_undo());
            self.ui.action_redo.triggered().connect(&self.slot_redo());
            self.ui.action_cut.triggered().connect(&self.slot_cut());
            self.ui.action_copy.triggered().connect(&self.slot_copy());
            self.ui.action_paste.triggered().connect(&self.slot_paste());
            self.ui.action_select_all.triggered().connect(&self.slot_select_all());

            self.recent.borrow_mut().truncate(MAX_RECENT);
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
        for (i, (button, path)) in [
            &self.ui.action_rec_1,
            &self.ui.action_rec_2,
            &self.ui.action_rec_3,
            &self.ui.action_rec_4,
            &self.ui.action_rec_5,
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
        self.recent.borrow_mut().truncate(MAX_RECENT);
        unsafe {
            self.settings
                .set_list(KEY_RECENT, self.recent.borrow().iter());
        }
        self.setup_recents();
    }

    fn open_world_file(self: &Rc<Self>, path: &String) {
        self.recent.borrow_mut().retain(|x| x != path);
        match persist::load_world(path) {
            Ok(world) => {
                self.title_from_file(path);
                self.recent.borrow_mut().push_front(path.to_owned());
                self.start_world(world, Some(path.to_owned()));
            }
            Err(e) => self.alert(Icon::Critical, tr!("Cannot open file"), Some(e.to_string())),
        }

        self.save_recents();
    }

    fn set_world_menus_enabled(&self, enabled: bool) {
        for &action in &[
            &self.ui.action_close_world,
            &self.ui.action_import,
            &self.ui.action_plugins,
            &self.ui.action_plugin_wizard,
            &self.ui.action_save_world_details,
            &self.ui.action_save_world_details_as,
            &self.ui.action_print,
            &self.ui.action_log_session,
            &self.ui.action_reload_defaults,
            &self.ui.action_world_properties,
            &self.ui.action_undo,
            &self.ui.action_paste,
            &self.ui.action_paste_to_world,
            &self.ui.action_recall_last_word,
            &self.ui.action_select_all,
            &self.ui.action_debug_packets,
            &self.ui.action_go_to_matching_brace,
            &self.ui.action_select_to_matching_brace,
        ] {
            unsafe {
                action.set_enabled(enabled);
            }
        }
    }

    fn start_world(self: &Rc<Self>, world: World, filename: Option<String>) {
        let tabname = QString::from_std_str(world.name.replace('&', ""));
        let should_open = world.connect_method.is_some();
        let worldtab = WorldTab::new(self.widget(), world, filename);
        unsafe {
            self.ui.world_tabs.add_tab_2a(worldtab.widget(), &tabname);
            self.ui
                .world_tabs
                .set_current_index(self.ui.world_tabs.count() - 1);
            self.set_world_menus_enabled(true);

            worldtab
                .ui
                .input
                .set_focus_1a(FocusReason::ActiveWindowFocusReason);
            if should_open {
                worldtab.open_connection();
            }
        }
        let this = Rc::downgrade(self);
        worldtab.connect_selection_changed(move |mode| {
            if let Some(this) = this.upgrade() {
                this.selection_changed(mode);
            }
        });
        self.world_tabs.borrow_mut().push(worldtab);
    }
    fn current_tab(&self) -> Option<Rc<WorldTab>> {
        let current = self.current.get()?;
        self.world_tabs.borrow().get(current).map(Clone::clone)
    }

    fn save_world_file(self: &Rc<Self>, force_different: bool) {
        let tab = match self.current_tab() {
            Some(tab) => tab,
            None => return,
        };
        let world = tab.borrow_world();
        let save_as = match tab.saved.borrow().as_ref() {
            Some(save_as) if !force_different => save_as.to_owned(),
            _ => unsafe {
                QFileDialog::get_save_file_name_4a(
                    self.widget(),
                    &tr!("Save as"),
                    &QString::from_std_str(&format!("{}/{}.qmc", SAVE_DIR, world.name)),
                    &self.save_filter,
                )
                .to_std_string()
            },
        };
        if save_as.is_empty() {
            return;
        }

        if let Err(e) = persist::save_world(&world, &save_as) {
            self.alert(Icon::Critical, tr!("Cannot save file"), Some(e.to_string()));
            return;
        }

        self.title_from_file(&save_as);
        tab.saved.replace(Some(save_as.clone()));
        {
            let mut recents = self.recent.borrow_mut();
            recents.retain(|x| x != &save_as);
            recents.push_front(save_as);
        }
        self.save_recents();
    }

    unsafe fn with_input<O: Default>(&self, f: unsafe fn(&QLineEdit) -> O) -> O {
        match self.current_input.borrow().as_raw_ref() {
            Some(input) => f(input),
            None => O::default(),
        }
    }

    #[slot(SlotOfInt)]
    fn current_changed(&self, index: c_int) {
        self.current.replace(usize::try_from(index).ok());
        match self.current_tab() {
            None => {
                self.set_world_menus_enabled(false);
                self.current_input.replace(unsafe { QPtr::null() });
                for action in &[
                    &self.ui.action_cut,
                    &self.ui.action_copy,
                    &self.ui.action_copy_as_html,
                    &self.ui.action_redo,
                ] {
                    unsafe {
                        action.set_enabled(false);
                    }
                }
            }
            Some(tab) => {
                self.set_world_menus_enabled(true);
                self.current_input.replace(tab.ui.input.clone());
                unsafe {
                    self.ui
                        .action_redo
                        .set_enabled(tab.ui.input.is_redo_available());
                    self.selection_changed(tab.selection_mode());
                }
            }
        }
    }

    fn selection_changed(&self, mode: SelectionMode) {
        unsafe {
            self.ui.action_cut.set_enabled(mode == SelectionMode::Input);
            self.ui
                .action_copy
                .set_enabled(mode != SelectionMode::Neither);
            self.ui
                .action_copy_as_html
                .set_enabled(mode == SelectionMode::Output);
        }
    }

    #[slot(SlotNoArgs)]
    fn new_world(self: &Rc<Self>) {
        let mut world = World::new();
        world.chat_name = "Name-not-set".to_string();
        let world = Rc::new(RefCell::new(world));
        let prefs = WorldPrefs::new(self.widget(), Rc::downgrade(&world));
        while prefs.exec() == DialogCode::Accepted {
            let (named, sited) = {
                let world = world.borrow();
                (!world.name.is_empty(), !world.site.is_empty())
            };
            if named && sited {
                self.start_world(Rc::try_unwrap(world).unwrap().into_inner(), None);
                return;
            }
            let failure = if named {
                tr!("You must enter the server address.")
            } else if sited {
                tr!("You must enter the world name.")
            } else {
                tr!("You must enter the world name and server address.")
            };

            prefs.browse("IP address");
            self.alert(Icon::Critical, failure, None::<&str>);
        }
    }

    #[slot(SlotNoArgs)]
    fn open_world(self: &Rc<Self>) {
        let filename = unsafe {
            QFileDialog::get_open_file_name_4a(
                self.widget(),
                &tr!("Open world"),
                &self.save_dir,
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
    fn open_world_list(self: &Rc<Self>) {
        if let Ok(list) = self.settings.get_list("startuplist") {
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
        self.close_world(self.current.get().unwrap() as c_int);
    }

    #[slot(SlotOfInt)]
    fn close_world(&self, i: c_int) {
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
        println!("lesgo");
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

    #[slot(SlotNoArgs)]
    fn exit(&self) {
        unsafe { QCoreApplication::quit() }
    }
}
