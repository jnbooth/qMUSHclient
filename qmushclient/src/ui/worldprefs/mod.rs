use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use cpp_core::{CastInto, Ptr, Ref};
use qt_bindings::{RFont, RForm, Widget};
use qt_core::{slot, QPtr};
use qt_widgets::q_dialog::DialogCode;
use qt_widgets::*;
use tr::TrContext;

use super::uic;
use crate::ui::worldprefs::tables::PrefsTimers;
use crate::world::World;

macro_rules! impl_prefpage {
    ($t:ident) => {
        impl crate::ui::worldprefs::PrefPage for $t {
            fn get_page(&self) -> qt_core::QPtr<qt_widgets::QWidget> {
                unsafe { self.ui.widget.static_upcast() }
            }
            fn get_world(&self) -> std::rc::Weak<std::cell::RefCell<crate::world::World>> {
                self.world.clone()
            }
        }
    };
}

macro_rules! impl_prefpageext {
    ($t:ident) => {
        impl super::PrefPageExt for $t {
            fn new<P: cpp_core::CastInto<cpp_core::Ptr<qt_widgets::QWidget>>>(
                parent: P,
                world: std::rc::Weak<std::cell::RefCell<crate::world::World>>,
            ) -> std::rc::Rc<Self> {
                let this = std::rc::Rc::new(Self {
                    ui: uic::$t::load(parent),
                    world,
                });
                this.init();
                this
            }
        }
    };
}

macro_rules! connect_world_one {
    ($ui:ident, $world:ident, $self:ident, $field:ident, $fieldname:ident$(, $subfield:ident)?) => {
        $self.connect(&mut *$world, &$ui.$fieldname, |world| &mut world.$field$(.$subfield)?)
    };
    ($ui:ident, $world:ident, $self:ident, $field:ident) => {
        connect_world_one!($ui, $world, $self, $field, $field)
    };
}

macro_rules! connect_world {
    ($self:ident, $($field:ident$(.$subfield:ident $fieldname:ident)?),+$(,)?) => {
        let worldrc = $self.world.upgrade().unwrap();
        let mut world = worldrc.borrow_mut();
        let ui = &$self.ui;
        $(
            connect_world_one!(ui, world, $self, $field$(, $fieldname, $subfield)?);
        )+
        std::mem::drop(world);
        std::mem::drop(worldrc);
    }
}

mod general;
use general::{PrefsAddress, PrefsChat, PrefsConnecting, PrefsLogging};
mod appearance;
use appearance::{PrefsColor, PrefsCustomColor, PrefsMxp, PrefsOutput};
mod input;
use input::PrefsCommands;
mod tables;
use tables::{PrefsAliases, PrefsTriggers};

trait PrefPage {
    fn get_page(&self) -> QPtr<QWidget>;
    fn get_world(&self) -> Weak<RefCell<World>>;
}

/// These methods cannot be part of `PrefPage`, because they would prevent creating `dyn PrefPage`.
trait PrefPageExt: 'static + PrefPage + Widget {
    fn new<P: CastInto<Ptr<QWidget>>>(parent: P, world: Weak<RefCell<World>>) -> Rc<Self>;

    /// # Safety
    ///
    /// `field` must be valid.
    unsafe fn connect<T, Q, F>(self: &Rc<Self>, world: &mut World, field: &Q, getter: F)
    where
        T: 'static,
        Q: 'static + RForm<T>,
        F: 'static + Clone + Fn(&mut World) -> &mut T,
    {
        // SAFETY: `field` is valid.
        unsafe {
            self.connect_form(field, getter(world), self.get_world(), move |world, val| {
                *getter(&mut world.borrow_mut()) = val;
            });
        }
    }

    /// # Safety
    ///
    /// `fontfield` and `sizefield` must be valid.
    unsafe fn connect_font(
        self: &Rc<Self>,
        fontfield: &QPtr<QFontComboBox>,
        sizefield: &QPtr<QSpinBox>,
        getter: fn(&mut World) -> &mut RFont,
    ) {
        let worldrc = self.get_world().upgrade().unwrap();
        let mut worldref = worldrc.borrow_mut();
        let font = getter(&mut worldref);
        // SAFETY: fields are valid.
        unsafe {
            self.connect_form(
                sizefield,
                &font.size(),
                self.get_world(),
                move |world, size| {
                    getter(&mut world.borrow_mut()).set_size(size);
                },
            );
            self.connect_form(fontfield, font, self.get_world(), move |world, font| {
                getter(&mut world.borrow_mut()).set_family(&font.family());
            });
        }
    }
}

#[derive(Widget, TrContext)]
pub struct WorldPrefs {
    ui: uic::WorldPrefs,
    world: Weak<RefCell<World>>,
    pages: HashMap<&'static str, Rc<dyn PrefPage>>,
    current: RefCell<Option<Ref<QWidget>>>,
}

impl WorldPrefs {
    pub fn new<P: CastInto<Ptr<QWidget>>>(parent: P, world: Weak<RefCell<World>>) -> Rc<Self> {
        let mut this = Self {
            ui: uic::WorldPrefs::load(parent),
            world,
            pages: HashMap::new(),
            current: RefCell::new(None),
        };

        let prefs_address = PrefsAddress::new(&this.ui.widget, this.world.clone());
        prefs_address.connect_ok(&this.ui);
        this.pages.insert("IP address", prefs_address);

        this.addpage::<PrefsConnecting>("Connecting");
        this.addpage::<PrefsLogging>("Logging");
        this.addpage::<PrefsTimers>("Timers");
        this.addpage::<PrefsChat>("Chat");
        this.addpage::<PrefsOutput>("Output");
        this.addpage::<PrefsMxp>("MXP/Pueblo");
        this.addpage::<PrefsColor>("ANSI Colour");
        this.addpage::<PrefsCustomColor>("Custom Colour");
        this.addpage::<PrefsCommands>("Commands");
        this.addpage::<PrefsAliases>("Aliases");
        this.addpage::<PrefsTriggers>("Triggers");
        let this = Rc::new(this);
        this.init();
        this
    }

    fn addpage<P: PrefPageExt>(&mut self, key: &'static str) {
        self.pages
            .insert(key, P::new(&self.ui.widget, self.world.clone()));
    }

    fn init(self: &Rc<Self>) {
        unsafe {
            let ui = &self.ui;
            for page in self.pages.values() {
                let page = page.get_page();
                page.set_visible(false);
                ui.contents.add_widget(page);
            }
            ui.settings_tree.expand_all();
            ui.settings_tree
                .current_item_changed()
                .connect(&self.slot_choose_page());
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

    pub fn exec(&self) -> DialogCode {
        let ui = &self.ui;
        unsafe {
            ui.settings_tree
                .set_current_item_1a(ui.settings_tree.top_level_item(0).child(0));
            DialogCode::from(ui.widget.exec())
        }
    }

    /// # Safety
    ///
    /// `item` must be valid.
    #[slot(SlotOfQTreeWidgetItem)]
    unsafe fn choose_page(&self, item: Ptr<QTreeWidgetItem>) {
        unsafe {
            if item.child_count() == 0 {
                self.browse(&item.text(0).to_std_string());
            }
        }
    }
}
