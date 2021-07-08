use std::cell::RefCell;
use std::rc::{Rc, Weak};

use cpp_core::{CastInto, Ptr, Ref};
use hashbrown::HashMap;
use qt_core::{slot, CheckState, QPtr, SlotNoArgs, SlotOfInt};
use qt_widgets::q_dialog::DialogCode;
use qt_widgets::*;

use super::uic;
use crate::binding::{RDialog, RFont, RWidget};
use crate::tr::TrContext;
use crate::world::World;

macro_rules! impl_prefpage {
    ($t:ident) => {
        impl crate::ui::worldprefs::PrefPage for $t {
            fn get_page(&self) -> QPtr<QWidget> {
                unsafe { self.ui.widget.static_upcast() }
            }
            fn upgrade_world(&self) -> Option<Rc<RefCell<World>>> {
                self.world.upgrade()
            }
        }
    };
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

macro_rules! connect_world_one {
    ($ui:ident, $self:ident, $field:ident, $fieldname:ident$(, $subfield:ident)?) => {
        $self.connect(
            $ui.$fieldname.clone(),
            |world| &mut world.$field$(.$subfield)?
        )
    };
    ($ui:ident, $self:ident, $field:ident) => {
        connect_world_one!($ui, $self, $field, $field)
    };
}

macro_rules! connect_world {
    ($self:ident, $($field:ident$(.$subfield:ident $fieldname:ident)?),+$(,)?) => {
        let ui = &$self.ui;
        $(
            connect_world_one!(ui, $self, $field$(, $fieldname, $subfield)?);
        )+
    }
}

mod general;
use general::{PrefsAddress, PrefsChat, PrefsConnecting, PrefsLogging};
mod appearance;
use appearance::{PrefsColor, PrefsCustomColor, PrefsMxp, PrefsOutput};
mod input;
use input::PrefsCommands;
mod qform;

use qform::QForm;

trait PrefPage {
    fn get_page(&self) -> QPtr<QWidget>;
    fn upgrade_world(&self) -> Option<Rc<RefCell<World>>>;
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

#[derive(RWidget, TrContext)]
pub struct WorldPrefs {
    ui: uic::WorldPrefs,
    world: Weak<RefCell<World>>,
    pages: HashMap<&'static str, Rc<dyn PrefPage>>,
    current: RefCell<Option<Ref<QWidget>>>,
}
impl RDialog<DialogCode> for WorldPrefs {
    fn exec(&self) -> DialogCode {
        let ui = &self.ui;
        unsafe {
            ui.settings_tree
                .set_current_item_1a(ui.settings_tree.top_level_item(0).child(0));
            DialogCode::from(ui.widget.exec())
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
        this.addpage::<PrefsCustomColor>("Custom Colour");
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
            let ui = &self.ui;
            for page in self.pages.values() {
                let page = page.get_page();
                page.set_visible(false);
                ui.contents.add_widget(page);
            }
            ui.settings_tree.expand_all();
            ui.settings_tree.current_item_changed().connect(&self.slot_choose_page());
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
