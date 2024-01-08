use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::os::raw::{c_double, c_int};
use std::path::PathBuf;
use std::ptr;
use std::rc::{Rc, Weak};
use std::time::Duration;

use chrono::{NaiveTime, Timelike};
use cpp_core::{CastInto, CppDeletable, Ptr};
use enumeration::Enum;
use qt::traits::{Browse, HasPalette, Widget};
use qt_core::{slot, QPtr, QString, QTime, SlotNoArgs, SlotOfInt, SlotOfQString};
use qt_gui::q_palette::ColorRole;
use qt_widgets::q_dialog::DialogCode;
use qt_widgets::q_dialog_button_box::StandardButton;
use qt_widgets::q_message_box::Icon;
use qt_widgets::{QPushButton, QTreeWidget, QTreeWidgetItem, QWidget, SlotOfQTreeWidgetItem};
use tr::TrContext;

use super::{uic, PrefPageExt};
use crate::script::{Alias, Event, Reaction, SendTo, Sender, Timer, Trigger};
use crate::world::World;
use crate::DurationExt;

#[derive(Debug, Widget, TrContext)]
struct TriggerEdit {
    ui: uic::TriggerEdit,
}

impl TriggerEdit {
    pub fn new<P: CastInto<Ptr<QWidget>>>(parent: P) -> Self {
        let this = Self {
            ui: uic::TriggerEdit::load(parent),
        };
        this.init();
        this
    }

    /// # Safety
    ///
    /// `button` must be valid.
    unsafe fn connect_color(&self, button: QPtr<QPushButton>) {
        let ui = &self.ui;
        unsafe {
            let parent = ui.widget.as_ptr();
            button.set_maximum_width(button.height());
            button.clicked().connect(&SlotNoArgs::new(parent, move || {
                if let Some(color) = button.palette_color(ColorRole::Button).pick(parent) {
                    button.set_palette_color(ColorRole::Button, &color);
                }
            }));
            let varlabel = ui.variable_label.clone();
            let varfield = ui.variable.clone();
            ui.send_to
                .current_index_changed()
                .connect(&SlotOfInt::new(&ui.widget, move |i| {
                    let on_variable = i == SendTo::Variable.index() as c_int;
                    varlabel.set_visible(on_variable);
                    varfield.set_visible(on_variable);
                }));
        }
    }

    #[rustfmt::skip]
    fn init(&self) {
        let ui = &self.ui;
        // SAFETY: fields are valid.
        unsafe {
            let ok_button = ui.button_box.button(StandardButton::Ok);
            ok_button.set_enabled(false);
            {
                let ok_button = ok_button.clone();
                ui.pattern.text_changed().connect(&SlotOfQString::new(
                    &ui.widget,
                    move |text| ok_button.set_enabled(!text.is_empty()),
                ));
            }
            {
                let at_time = ui.at_time.clone();
                let every_hour = ui.every_hour.clone();
                let every_minute = ui.every_minute.clone();
                let every_second = ui.every_second.clone();
                let enable_timer = SlotNoArgs::new(&ui.widget, move || {
                    ok_button.set_enabled(
                        at_time.is_checked()
                            || every_hour.value() > 0
                            || every_minute.value() > 0
                            || every_second.value() > 0.0,
                    );
                });
                ui.at_time.toggled().connect(&enable_timer);
                ui.every_hour.value_changed().connect(&enable_timer);
                ui.every_minute.value_changed().connect(&enable_timer);
                ui.every_second.value_changed().connect(&enable_timer);
            }
            self.connect_color(ui.foreground.clone());
            self.connect_color(ui.background.clone());
            self.connect_browse_button(
                Browse::Open,
                &ui.sound_browse,
                &ui.sound,
                || "sounds/",
                "Waveaudio files (*.wav)",
            );
        }
    }

    pub fn exec(&self) -> bool {
        unsafe { self.ui.widget.exec() == DialogCode::Accepted.to_int() }
    }

    pub fn sender(&self) -> Sender {
        let ui = &self.ui;
        unsafe {
            debug_assert!(ui.send_to.count() == SendTo::SIZE as c_int);
            Sender {
                text: ui.text.to_plain_text().trimmed().to_std_string(),
                send_to: SendTo::from_index(ui.send_to.current_index() as usize).unwrap(),
                label: ui.label.text().to_std_string(),
                script: ui.script.text().to_std_string(),
                group: ui.group.text().to_std_string(),
                variable: ui.variable.text().trimmed().to_std_string(),
                enabled: ui.enabled.is_checked(),
                one_shot: ui.one_shot.is_checked(),
                temporary: ui.temporary.is_checked(),
                omit_from_output: ui.omit_from_output.is_checked(),
                omit_from_log: ui.omit_from_log.is_checked(),
            }
        }
    }

    pub fn reaction(&self) -> Result<Reaction, fancy_regex::Error> {
        let ui = &self.ui;
        unsafe {
            let pattern = ui.pattern.text().to_std_string();
            let is_regex = ui.is_regex.is_checked();
            Ok(Reaction {
                regex: Reaction::make_regex(&pattern, is_regex)?,
                pattern,
                send: self.sender(),
                sequence: ui.sequence.value() as i16,
                ignore_case: ui.ignore_case.is_checked(),
                keep_evaluating: ui.keep_evaluating.is_checked(),
                is_regex,
                expand_variables: ui.expand_variables.is_checked(),
                repeat: ui.repeat.is_checked(),
            })
        }
    }

    pub fn timer(&self) -> Timer {
        let ui = &self.ui;
        unsafe {
            let event = if ui.every.is_checked() {
                let hour = ui.every_hour.value();
                let minute = ui.every_minute.value();
                let second = ui.every_second.value();
                Event::Interval(Duration::from_hms(hour as u64, minute as u64, second))
            } else {
                let msecs = ui.time.time().msecs_since_start_of_day() as u32;
                let secs = msecs / 1_000;
                let nano = (msecs % 1_0000) * 1_000_000;
                let time = NaiveTime::from_num_seconds_from_midnight_opt(secs, nano);
                Event::Time(time.unwrap())
            };
            Timer {
                send: self.sender(),
                event,
                active_closed: ui.active_closed.is_checked(),
            }
        }
    }

    pub fn try_until<T>(&self, f: fn(&Self) -> Result<T, fancy_regex::Error>) -> Option<T> {
        loop {
            if !self.exec() {
                return None;
            }
            match f(self) {
                Ok(val) => return Some(val),
                Err(e) => self.alert(Icon::Warning, tr!("Invalid regular expression"), &e),
            }
        }
    }

    pub fn alias(&self) -> Result<Alias, fancy_regex::Error> {
        let ui = &self.ui;
        unsafe {
            Ok(Alias {
                reaction: self.reaction()?,
                echo_alias: ui.echo_alias.is_checked(),
                menu: ui.menu.is_checked(),
                omit_from_command_history: ui.omit_from_command_history.is_checked(),
            })
        }
    }

    pub fn trigger(&self) -> Result<Trigger, fancy_regex::Error> {
        let ui = &self.ui;
        unsafe {
            let soundfile = ui.sound.text().trimmed().to_std_string();
            let sound = if soundfile.is_empty() {
                None
            } else {
                Some(PathBuf::from(soundfile))
            };
            Ok(Trigger {
                reaction: self.reaction()?,
                change_foreground: ui.change_foreground.is_checked(),
                foreground: ui.foreground.palette_color(ColorRole::Button),
                change_background: ui.change_background.is_checked(),
                background: ui.background.palette_color(ColorRole::Button),
                make_bold: ui.make_bold.is_checked(),
                make_italic: ui.make_italic.is_checked(),
                make_underline: ui.make_underline.is_checked(),
                sound,
                sound_if_inactive: ui.sound_if_inactive.is_checked(),
                lowercase_wildcard: ui.lowercase_wildcard.is_checked(),
                multi_line: ui.multi_line.is_checked(),
                lines_to_match: ui.lines_to_match.value() as u8,
            })
        }
    }

    #[rustfmt::skip]
    pub fn load_sender(&self, send: &Sender) {
        let ui = &self.ui;
        unsafe {
            ui.text.set_plain_text(&QString::from_std_str(&send.text));
            ui.send_to.set_current_index(send.send_to.index() as c_int);
            ui.label.set_text(&QString::from_std_str(&send.label));
            ui.script.set_text(&QString::from_std_str(&send.script));
            ui.group.set_text(&QString::from_std_str(&send.group));
            ui.variable.set_text(&QString::from_std_str(&send.variable));
            ui.enabled.set_checked(send.enabled);
            ui.one_shot.set_checked(send.one_shot);
            ui.temporary.set_checked(send.temporary);
            ui.omit_from_output.set_checked(send.omit_from_output);
            ui.omit_from_log.set_checked(send.omit_from_log);
        }
    }

    #[rustfmt::skip]
    pub fn load_reaction(&self, reaction: &Reaction) {
        let ui = &self.ui;
        unsafe {
            self.load_sender(&reaction.send);
            ui.pattern.set_text(&QString::from_std_str(&reaction.pattern));
            ui.sequence.set_value(reaction.sequence as c_int);
            ui.ignore_case.set_checked(reaction.ignore_case);
            ui.keep_evaluating.set_checked(reaction.keep_evaluating);
            ui.is_regex.set_checked(reaction.is_regex);
            ui.expand_variables.set_checked(reaction.expand_variables);
        }
    }

    pub fn load_timer(&self, timer: &Timer) {
        let ui = &self.ui;
        unsafe {
            ui.sequence.hide();
            ui.sequence_label.hide();
            ui.ignore_case.hide();
            ui.keep_evaluating.hide();
            ui.expand_variables.hide();
            ui.lowercase_wildcard.hide();
            ui.is_regex.hide();
            ui.echo_alias.hide();
            ui.menu.hide();
            ui.repeat.hide();
            ui.multi_line.hide();
            ui.triggers_multi_line.hide();
            ui.triggers_sound.hide();
            ui.triggers_style.hide();
            ui.reactions_pattern.hide();
            ui.widget.adjust_size();

            self.load_sender(&timer.send);
            ui.active_closed.set_checked(timer.active_closed);
            match timer.event {
                Event::Interval(every) => {
                    ui.every.set_checked(true);
                    ui.every_hour.set_value(every.hour() as c_int);
                    ui.every_minute.set_value(every.minute() as c_int);
                    ui.every_second.set_value(every.second() as c_double);
                }
                Event::Time(at_time) => {
                    ui.at_time.set_checked(true);
                    let secs = at_time.num_seconds_from_midnight();
                    let nano = at_time.nanosecond();
                    let milli = secs * 1_000 + nano / 1_000_000;
                    self.ui
                        .time
                        .set_time(&QTime::from_m_secs_since_start_of_day(milli as c_int));
                }
            }
        }
    }

    #[rustfmt::skip]
    pub fn load_alias(&self, alias: &Alias) {
        let ui = &self.ui;
        unsafe {
            ui.repeat.hide();
            ui.multi_line.hide();
            ui.triggers_multi_line.hide();
            ui.triggers_sound.hide();
            ui.triggers_style.hide();
            ui.timers_event.hide();
            ui.active_closed.hide();
            ui.widget.adjust_size();

            self.load_reaction(&alias.reaction);
            ui.echo_alias.set_checked(alias.echo_alias);
            ui.omit_from_command_history.set_checked(alias.omit_from_command_history);
            ui.menu.set_checked(alias.menu);
        }
    }

    #[rustfmt::skip]
    pub fn load_trigger(&self, trigger: &Trigger) {
        let ui = &self.ui;
        unsafe {
            ui.echo_alias.hide();
            ui.omit_from_command_history.hide();
            ui.menu.hide();
            ui.timers_event.hide();
            ui.active_closed.hide();
            ui.widget.adjust_size();

            self.load_reaction(&trigger.reaction);
            ui.change_foreground.set_checked(trigger.change_foreground);
            ui.foreground.set_palette_color(ColorRole::Button, &trigger.foreground);
            ui.change_background.set_checked(trigger.change_background);
            ui.background.set_palette_color(ColorRole::Button, &trigger.background);
            ui.make_bold.set_checked(trigger.make_bold);
            ui.make_italic.set_checked(trigger.make_italic);
            ui.make_underline.set_checked(trigger.make_underline);
            ui.multi_line.set_checked(trigger.multi_line);
            ui.lines_to_match.set_value(trigger.lines_to_match as c_int);
            ui.repeat.set_checked(trigger.repeat);
            ui.lowercase_wildcard.set_checked(trigger.lowercase_wildcard);
            ui.sound.set_text(&QString::from_std_str(
                trigger.sound.as_ref().and_then(|x| x.to_str()).unwrap_or(""),
            ));
            ui.sound_if_inactive.set_checked(trigger.sound_if_inactive);

            ui.widget.set_window_title(&tr!("Edit Trigger"));
        }
    }
}

trait TreeItem: AsRef<Sender> {
    fn show_tree_item(&self, item: &QTreeWidgetItem);

    /// # Safety
    ///
    /// `parent` must be valid and non-null.
    unsafe fn make_tree_item(&self, parent: Ptr<QTreeWidgetItem>) -> Ptr<QTreeWidgetItem> {
        unsafe {
            let item = QTreeWidgetItem::from_q_tree_widget_item(parent);
            self.show_tree_item(&item);
            item.into_ptr()
        }
    }
}

impl TreeItem for Timer {
    fn show_tree_item(&self, item: &QTreeWidgetItem) {
        unsafe {
            item.set_text(0, &QString::from_std_str(&self.label));
            item.set_text(2, &QString::from_std_str(self.event.to_string()));
            item.set_text(3, &QString::from_std_str(&self.text));
        }
    }
}

impl<T: AsRef<Reaction> + AsRef<Sender>> TreeItem for T {
    fn show_tree_item(&self, item: &QTreeWidgetItem) {
        let subject: &Reaction = self.as_ref();
        unsafe {
            item.set_text(0, &QString::from_std_str(&subject.label));
            item.set_text(1, &QString::number_int(subject.sequence as c_int));
            item.set_text(2, &QString::from_std_str(&subject.pattern));
            item.set_text(3, &QString::from_std_str(&subject.text));
        }
    }
}

#[derive(Clone, Debug)]
struct TableModel<T> {
    tree: QPtr<QTreeWidget>,
    groups: RefCell<HashMap<String, Ptr<QTreeWidgetItem>>>,
    items: RefCell<Vec<Ptr<QTreeWidgetItem>>>,
    marker: PhantomData<T>,
}

impl<T: TreeItem> TableModel<T> {
    /// # Safety
    ///
    /// `widget` must be valid.
    unsafe fn new(tree: QPtr<QTreeWidget>, values: &[T]) -> Self {
        let mut groups: Vec<_> = values
            .iter()
            .filter_map(|value| {
                let group = &value.as_ref().group;
                if group.is_empty() {
                    None
                } else {
                    Some(group.clone())
                }
            })
            .collect();
        groups.sort_unstable();
        groups.dedup();
        let mut items = Vec::new();
        unsafe {
            let mut group_widgets: HashMap<String, Ptr<QTreeWidgetItem>> = groups
                .into_iter()
                .map(|group| {
                    let item = QTreeWidgetItem::from_q_tree_widget(&tree);
                    item.set_text(0, &QString::from_std_str(&group));
                    (group, item.into_ptr())
                })
                .collect();
            group_widgets.insert(String::new(), tree.invisible_root_item());
            for value in values {
                items.push(value.make_tree_item(*group_widgets.get(&value.as_ref().group).unwrap()));
            }
            Self {
                tree,
                groups: RefCell::new(group_widgets),
                items: RefCell::new(items),
                marker: PhantomData,
            }
        }
    }

    fn index_of(&self, item: Ptr<QTreeWidgetItem>) -> Option<usize> {
        self.items
            .borrow()
            .iter()
            .position(|it| ptr::eq(it.as_raw_ptr(), item.as_raw_ptr()))
    }

    fn add(&self, value: &T) {
        let group = &value.as_ref().group;
        unsafe {
            match self.groups.borrow_mut().entry(group.to_owned()) {
                Entry::Occupied(entry) => {
                    self.items
                        .borrow_mut()
                        .push(value.make_tree_item(*entry.get()));
                }
                Entry::Vacant(entry) => {
                    let item = QTreeWidgetItem::from_q_tree_widget(&self.tree);
                    item.set_text(0, &QString::from_std_str(group));
                    let ptr = item.into_ptr();
                    self.items.borrow_mut().push(value.make_tree_item(ptr));
                    entry.insert(ptr);
                }
            }
        }
    }

    fn remove_current(&self) -> Option<usize> {
        unsafe {
            let current = self.tree.current_item();
            let pos = self.index_of(current)?;
            current.delete();
            Some(pos)
        }
    }
}

#[derive(Debug, Widget, TrContext)]
pub struct PrefsTimers {
    ui: uic::PrefsTimers,
    world: Weak<RefCell<World>>,
    model: TableModel<Timer>,
}
impl_prefpage!(PrefsTimers);

impl PrefPageExt for PrefsTimers {
    fn new<P: CastInto<Ptr<QWidget>>>(parent: P, world: Weak<RefCell<World>>) -> Rc<Self> {
        let ui = uic::PrefsTimers::load(parent);
        let model =
            unsafe { TableModel::new(ui.tree.clone(), &world.upgrade().unwrap().borrow().timers) };
        let this = Rc::new(Self { ui, world, model });
        this.init();
        this
    }
}

impl PrefsTimers {
    #[rustfmt::skip]
    fn init(self: &Rc<Self>) {
        unsafe {
            connect_world!(
                self,
                enable_timers,
            );
            let ui = &self.ui;
            ui.add.clicked().connect(&self.slot_add());
            ui.remove.clicked().connect(&self.slot_remove());
            ui.edit.clicked().connect(&self.slot_edit());
            ui.tree.item_activated().connect(&self.slot_activated());
        }
    }

    #[slot(SlotNoArgs)]
    fn add(&self) {
        let dialog = TriggerEdit::new(&self.ui.widget);
        dialog.load_timer(&Timer::default());
        if !dialog.exec() {
            return;
        }
        let timer = dialog.timer();
        self.model.add(&timer);
        self.world
            .upgrade()
            .unwrap()
            .borrow_mut()
            .timers
            .push(timer);
    }

    #[slot(SlotNoArgs)]
    fn remove(&self) {
        if let Some(pos) = self.model.remove_current() {
            self.world
                .upgrade()
                .unwrap()
                .borrow_mut()
                .timers
                .remove(pos);
        }
    }

    #[slot(SlotNoArgs)]
    fn edit(&self) {
        unsafe {
            self.activated(self.ui.tree.current_item());
        }
    }

    /// # Safety
    ///
    /// `item` must be valid.
    #[slot(SlotOfQTreeWidgetItem)]
    unsafe fn activated(&self, item: Ptr<QTreeWidgetItem>) {
        let pos = match self.model.index_of(item) {
            Some(pos) => pos,
            None => return,
        };
        let worldrc = self.world.upgrade().unwrap();
        let mut world = worldrc.borrow_mut();
        let timer = &mut world.timers[pos];
        let dialog = TriggerEdit::new(&self.ui.widget);
        dialog.load_timer(timer);
        if dialog.exec() {
            let newtimer = dialog.timer();
            newtimer.show_tree_item(&item);
            *timer = newtimer;
        }
    }
}
#[derive(Debug, Widget, TrContext)]
pub struct PrefsAliases {
    ui: uic::PrefsAliases,
    world: Weak<RefCell<World>>,
    model: TableModel<Alias>,
}
impl_prefpage!(PrefsAliases);

impl PrefPageExt for PrefsAliases {
    fn new<P: CastInto<Ptr<QWidget>>>(parent: P, world: Weak<RefCell<World>>) -> Rc<Self> {
        let ui = uic::PrefsAliases::load(parent);
        let model =
            unsafe { TableModel::new(ui.tree.clone(), &world.upgrade().unwrap().borrow().aliases) };
        let this = Rc::new(Self { ui, world, model });
        this.init();
        this
    }
}

impl PrefsAliases {
    #[rustfmt::skip]
    fn init(self: &Rc<Self>) {
        unsafe {
            connect_world!(
                self,
                enable_aliases,
            );
            let ui = &self.ui;
            ui.add.clicked().connect(&self.slot_add());
            ui.remove.clicked().connect(&self.slot_remove());
            ui.edit.clicked().connect(&self.slot_edit());
            ui.tree.item_activated().connect(&self.slot_activated());
        }
    }

    #[slot(SlotNoArgs)]
    fn add(&self) {
        let dialog = TriggerEdit::new(&self.ui.widget);
        dialog.load_alias(&Alias::default());
        let alias = match dialog.try_until(TriggerEdit::alias) {
            Some(alias) => alias,
            None => return,
        };
        self.model.add(&alias);
        self.world
            .upgrade()
            .unwrap()
            .borrow_mut()
            .aliases
            .push(alias);
    }

    #[slot(SlotNoArgs)]
    fn remove(&self) {
        if let Some(pos) = self.model.remove_current() {
            self.world
                .upgrade()
                .unwrap()
                .borrow_mut()
                .aliases
                .remove(pos);
        }
    }

    #[slot(SlotNoArgs)]
    fn edit(&self) {
        unsafe {
            self.activated(self.ui.tree.current_item());
        }
    }

    /// # Safety
    ///
    /// `item` must be valid.
    #[slot(SlotOfQTreeWidgetItem)]
    unsafe fn activated(&self, item: Ptr<QTreeWidgetItem>) {
        let pos = match self.model.index_of(item) {
            Some(pos) => pos,
            None => return,
        };
        let worldrc = self.world.upgrade().unwrap();
        let mut world = worldrc.borrow_mut();
        let alias = &mut world.aliases[pos];
        let dialog = TriggerEdit::new(&self.ui.widget);
        dialog.load_alias(alias);
        if let Some(newalias) = dialog.try_until(TriggerEdit::alias) {
            newalias.show_tree_item(&item);
            *alias = newalias;
        }
    }
}

#[derive(Debug, Widget, TrContext)]
pub struct PrefsTriggers {
    ui: uic::PrefsTriggers,
    world: Weak<RefCell<World>>,
    model: TableModel<Trigger>,
}
impl_prefpage!(PrefsTriggers);

impl PrefPageExt for PrefsTriggers {
    fn new<P: CastInto<Ptr<QWidget>>>(parent: P, world: Weak<RefCell<World>>) -> Rc<Self> {
        let ui = uic::PrefsTriggers::load(parent);
        let model =
            unsafe { TableModel::new(ui.tree.clone(), &world.upgrade().unwrap().borrow().triggers) };
        let this = Rc::new(Self { ui, world, model });
        this.init();
        this
    }
}

impl PrefsTriggers {
    #[rustfmt::skip]
    fn init(self: &Rc<Self>) {
        unsafe {
            connect_world!(
                self,
                enable_triggers,
                enable_trigger_sounds
            );
            let ui = &self.ui;
            ui.add.clicked().connect(&self.slot_add());
            ui.remove.clicked().connect(&self.slot_remove());
            ui.edit.clicked().connect(&self.slot_edit());
            ui.tree.item_activated().connect(&self.slot_activated());
        }
    }

    #[slot(SlotNoArgs)]
    fn add(&self) {
        let dialog = TriggerEdit::new(&self.ui.widget);
        dialog.load_trigger(&Trigger::default());
        if !dialog.exec() {
            return;
        }
        let trigger = match dialog.try_until(TriggerEdit::trigger) {
            Some(trigger) => trigger,
            None => return,
        };
        self.model.add(&trigger);
        self.world
            .upgrade()
            .unwrap()
            .borrow_mut()
            .triggers
            .push(trigger);
    }

    #[slot(SlotNoArgs)]
    fn remove(&self) {
        if let Some(pos) = self.model.remove_current() {
            self.world
                .upgrade()
                .unwrap()
                .borrow_mut()
                .triggers
                .remove(pos);
        }
    }

    #[slot(SlotNoArgs)]
    fn edit(&self) {
        unsafe {
            self.activated(self.ui.tree.current_item());
        }
    }

    /// # Safety
    ///
    /// `item` must be valid.
    #[slot(SlotOfQTreeWidgetItem)]
    unsafe fn activated(&self, item: Ptr<QTreeWidgetItem>) {
        let pos = match self.model.index_of(item) {
            Some(pos) => pos,
            None => return,
        };
        let worldrc = self.world.upgrade().unwrap();
        let mut world = worldrc.borrow_mut();
        let trigger = &mut world.triggers[pos];
        let dialog = TriggerEdit::new(&self.ui.widget);
        // SAFETY: fields are valid.
        unsafe {
            self.connect_browse_button(
                Browse::Open,
                &dialog.ui.sound_browse,
                &dialog.ui.sound,
                || "sounds/",
                "Waveaudio files (*.wav)",
            );
        }
        dialog.load_trigger(trigger);
        if let Some(newtrigger) = dialog.try_until(TriggerEdit::trigger) {
            newtrigger.show_tree_item(&item);
            *trigger = newtrigger;
        }
    }
}
