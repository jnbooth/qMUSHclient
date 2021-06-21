use super::uic;
use super::{WorldPrefs, WorldTab};
use crate::binding::{RDialog, RSettings, RWidget};
use crate::persist;
use crate::tr::TrContext;
use crate::world::World;
use cpp_core::{CppBox, NullPtr, Ptr};
use qt_core::{slot, FocusReason, QCoreApplication, QString, SlotOfInt, SlotNoArgs};
use qt_widgets::q_dialog::DialogCode;
use qt_widgets::q_message_box::Icon;
use qt_widgets::*;
use std::cell::RefCell;
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
    settings: RSettings,
}

impl App {
    pub fn new(organization: &str, name: &str) -> Rc<Self> {
        let settings = RSettings::new(organization, name);
        let recent = settings.get_list(KEY_RECENT).unwrap_or(VecDeque::new());

        let this = Rc::new(App {
            ui: uic::App::load(NullPtr),
            save_dir: QString::from_std_str(SAVE_DIR),
            save_filter: tr!("World files (*.qmc);;All Files (*.*)"),
            recent: RefCell::new(recent),
            world_tabs: RefCell::new(Vec::new()),
            settings,
        });
        this.init();
        this
    }

    fn init(self: &Rc<Self>) {
        unsafe {
            self.ui
                .world_tabs
                .tab_close_requested()
                .connect(&self.slot_close_world());
            self.ui
                .action_new
                .triggered()
                .connect(&self.slot_new_world());
            self.ui
                .action_open_world
                .triggered()
                .connect(&self.slot_open_world());
            self.ui
                .action_close_world
                .triggered()
                .connect(&self.slot_close_current_world());
            self.ui
                .action_save_world_details
                .triggered()
                .connect(&self.slot_save_world());
            self.ui
                .action_save_world_details_as
                .triggered()
                .connect(&self.slot_save_world_as());
            self.ui
                .action_world_properties
                .triggered()
                .connect(&self.slot_open_preferences());
            self.ui
                .action_exit
                .triggered()
                .connect(&SlotNoArgs::new(Ptr::from_raw(&**self), || {
                    QCoreApplication::quit()
                }));

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

    fn start_world(&self, world: World, filename: Option<String>) {
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
                .widget()
                .set_focus_1a(FocusReason::ActiveWindowFocusReason);
            if should_open {
                worldtab.open_connection();
            }
        }
        self.world_tabs.borrow_mut().push(worldtab);
    }
    fn current_tab(&self) -> Option<Rc<WorldTab>> {
        let c_current = unsafe { self.ui.world_tabs.current_index() };
        let current = usize::try_from(c_current).ok()?;
        Some(self.world_tabs.borrow()[current].clone())
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

    #[slot(SlotNoArgs)]
    fn new_world(&self) {
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
    fn save_world(self: &Rc<Self>) {
        self.save_world_file(false);
    }

    #[slot(SlotNoArgs)]
    fn save_world_as(self: &Rc<Self>) {
        self.save_world_file(true);
    }

    #[slot(SlotNoArgs)]
    fn close_current_world(&self) {
        self.close_world(unsafe { self.ui.world_tabs.current_index() });
    }

    #[slot(SlotOfInt)]
    fn close_world(&self, i: c_int) {
        self.world_tabs.borrow_mut().remove(i as usize).client.borrow_mut().disconnect();
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
}
