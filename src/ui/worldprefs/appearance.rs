use std::cell::RefCell;
use std::ops::{Index, IndexMut};
use std::os::raw::c_int;
use std::rc::{Rc, Weak};

use cpp_core::{CastInto, Ptr};
use qt_core::{slot, QPtr, SlotNoArgs};
use qt_gui::q_palette::ColorRole;
use qt_widgets::*;
use rand::RngCore;

use super::PrefPageExt;
use crate::binding::color::{HasPalette, RColorPair};
use crate::binding::{RColor, Widget};
use crate::client::color::Colors;
use crate::tr::TrContext;
use crate::ui::uic;
use crate::world::World;

const RATE_RESHADE: c_int = 5;
const RATE_SATURATE: c_int = 17;

#[derive(Debug, Widget, TrContext)]
pub struct PrefsOutput {
    ui: uic::PrefsOutput,
    world: Weak<RefCell<World>>,
}
impl_prefpage!(PrefsOutput);
impl_prefpageext!(PrefsOutput);

impl PrefsOutput {
    fn init(self: &Rc<Self>) {
        // SAFETY: fields are valid.
        unsafe {
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
            let ui = &self.ui;
            self.connect_font(&ui.output_font, &ui.output_font_size, |world| {
                &mut world.output_font
            });
        }
    }
}

#[derive(Debug, Widget, TrContext)]
pub struct PrefsMxp {
    ui: uic::PrefsMxp,
    world: Weak<RefCell<World>>,
}
impl_prefpage!(PrefsMxp);
impl_prefpageext!(PrefsMxp);

impl PrefsMxp {
    fn init(self: &Rc<Self>) {
        // SAFETY: fields are valid.
        unsafe {
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
}

#[derive(Debug, Widget, TrContext)]
pub struct PrefsColor {
    ui: uic::PrefsColor,
    world: Weak<RefCell<World>>,
    colorfields: [QPtr<QPushButton>; 16],
}
impl_prefpage!(PrefsColor);

impl PrefPageExt for PrefsColor {
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
        // SAFETY: fields are valid.
        unsafe {
            connect_world!(self, use_default_colors);

            let ui = &self.ui;
            ui.swap.clicked().connect(&self.slot_swap());
            ui.reset.clicked().connect(&self.slot_reset());
            ui.invert.clicked().connect(&self.slot_invert());
            ui.random.clicked().connect(&self.slot_random());
            ui.saturate.clicked().connect(&self.slot_saturate());
            ui.desaturate.clicked().connect(&self.slot_desaturate());
            ui.lighter.clicked().connect(&self.slot_lighter());
            ui.darker.clicked().connect(&self.slot_darker());
            ui.lighter_normal.clicked().connect(&self.slot_lighter_front());
            ui.lighter_bold.clicked().connect(&self.slot_lighter_back());
            ui.darker_normal.clicked().connect(&self.slot_darker_front());
            ui.darker_bold.clicked().connect(&self.slot_darker_back());

            let worldrc = self.world.upgrade().unwrap();
            let world = &mut *worldrc.borrow_mut();
            self.setcolors(&world.ansi_colors);
            for (i, field) in self.colorfields.iter().enumerate() {
                self.connect(world, field, move |world| &mut world.ansi_colors[i]);
            }
        }
    }

    fn setcolors(&self, colors: &[RColor]) {
        for (field, color) in self.colorfields.iter().zip(colors.iter()) {
            field.set_palette_color(ColorRole::Button, color);
        }
    }

    fn recolor<Idx, F>(&self, index: Idx, mut adjust: F)
    where
        Idx: Clone,
        [RColor]: IndexMut<Idx, Output = [RColor]>,
        [QPtr<QPushButton>]: Index<Idx, Output = [QPtr<QPushButton>]>,
        F: FnMut(&RColor) -> RColor,
    {
        for (field, color) in self.colorfields[index.clone()]
            .iter()
            .zip(self.world.upgrade().unwrap().borrow_mut().ansi_colors[index].iter_mut())
        {
            let adjusted = adjust(color);
            field.set_palette_color(ColorRole::Button, &adjusted);
            *color = adjusted;
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

    #[slot(SlotNoArgs)]
    fn invert(&self) {
        self.recolor(.., |color| color.invert());
    }

    #[slot(SlotNoArgs)]
    fn random(&self) {
        let mut randoms = [0; 3 * 16];
        rand::thread_rng().fill_bytes(&mut randoms);
        let mut byte = randoms.iter();
        self.recolor(.., |color| {
            RColor::rgba(
                *byte.next().unwrap(),
                *byte.next().unwrap(),
                *byte.next().unwrap(),
                color.alpha(),
            )
        });
    }

    #[slot(SlotNoArgs)]
    fn saturate(&self) {
        self.recolor(.., |color| color.saturate(RATE_SATURATE));
    }

    #[slot(SlotNoArgs)]
    fn desaturate(&self) {
        self.recolor(.., |color| color.saturate(-RATE_SATURATE));
    }

    #[slot(SlotNoArgs)]
    fn lighter(&self) {
        self.recolor(.., |color| color.reshade(RATE_RESHADE));
    }

    #[slot(SlotNoArgs)]
    fn darker(&self) {
        self.recolor(.., |color| color.reshade(-RATE_RESHADE));
    }

    #[slot(SlotNoArgs)]
    fn lighter_front(&self) {
        self.recolor(..8, |color| color.reshade(RATE_RESHADE));
    }

    #[slot(SlotNoArgs)]
    fn darker_front(&self) {
        self.recolor(..8, |color| color.reshade(-RATE_RESHADE));
    }

    #[slot(SlotNoArgs)]
    fn lighter_back(&self) {
        self.recolor(8.., |color| color.reshade(RATE_RESHADE));
    }

    #[slot(SlotNoArgs)]
    fn darker_back(&self) {
        self.recolor(8.., |color| color.reshade(-RATE_RESHADE));
    }
}

#[derive(Debug, Widget, TrContext)]
pub struct PrefsCustomColor {
    ui: uic::PrefsCustomColor,
    world: Weak<RefCell<World>>,
    namefields: [QPtr<QLineEdit>; 16],
    fgfields: [QPtr<QPushButton>; 16],
    bgfields: [QPtr<QPushButton>; 16],
}
impl_prefpage!(PrefsCustomColor);

impl PrefPageExt for PrefsCustomColor {
    fn new<P: CastInto<Ptr<QWidget>>>(parent: P, world: Weak<RefCell<World>>) -> Rc<Self> {
        let ui = uic::PrefsCustomColor::load(parent);
        let namefields = [
            ui.custom_name_0.clone(),
            ui.custom_name_1.clone(),
            ui.custom_name_2.clone(),
            ui.custom_name_3.clone(),
            ui.custom_name_4.clone(),
            ui.custom_name_5.clone(),
            ui.custom_name_6.clone(),
            ui.custom_name_7.clone(),
            ui.custom_name_8.clone(),
            ui.custom_name_9.clone(),
            ui.custom_name_10.clone(),
            ui.custom_name_11.clone(),
            ui.custom_name_12.clone(),
            ui.custom_name_13.clone(),
            ui.custom_name_14.clone(),
            ui.custom_name_15.clone(),
        ];
        let fgfields = [
            ui.custom_fg_0.clone(),
            ui.custom_fg_1.clone(),
            ui.custom_fg_2.clone(),
            ui.custom_fg_3.clone(),
            ui.custom_fg_4.clone(),
            ui.custom_fg_5.clone(),
            ui.custom_fg_6.clone(),
            ui.custom_fg_7.clone(),
            ui.custom_fg_8.clone(),
            ui.custom_fg_9.clone(),
            ui.custom_fg_10.clone(),
            ui.custom_fg_11.clone(),
            ui.custom_fg_12.clone(),
            ui.custom_fg_13.clone(),
            ui.custom_fg_14.clone(),
            ui.custom_fg_15.clone(),
        ];
        let bgfields = [
            ui.custom_bg_0.clone(),
            ui.custom_bg_1.clone(),
            ui.custom_bg_2.clone(),
            ui.custom_bg_3.clone(),
            ui.custom_bg_4.clone(),
            ui.custom_bg_5.clone(),
            ui.custom_bg_6.clone(),
            ui.custom_bg_7.clone(),
            ui.custom_bg_8.clone(),
            ui.custom_bg_9.clone(),
            ui.custom_bg_10.clone(),
            ui.custom_bg_11.clone(),
            ui.custom_bg_12.clone(),
            ui.custom_bg_13.clone(),
            ui.custom_bg_14.clone(),
            ui.custom_bg_15.clone(),
        ];
        let this = Rc::new(Self {
            ui,
            world,
            namefields,
            fgfields,
            bgfields,
        });
        this.init();
        this
    }
}

impl PrefsCustomColor {
    fn init(self: &Rc<Self>) {
        // SAFETY: fields are valid.
        unsafe {
            let ui = &self.ui;
            ui.reset.clicked().connect(&self.slot_reset());
            ui.invert.clicked().connect(&self.slot_invert());
            ui.random.clicked().connect(&self.slot_random());
            ui.saturate.clicked().connect(&self.slot_saturate());
            ui.desaturate.clicked().connect(&self.slot_desaturate());
            ui.lighter.clicked().connect(&self.slot_lighter());
            ui.darker.clicked().connect(&self.slot_darker());

            let worldrc = self.world.upgrade().unwrap();
            let world = &mut *worldrc.borrow_mut();
            self.setcolors(&world.custom_colors);
            for (i, field) in self.namefields.iter().enumerate() {
                self.connect(world, field, move |world| &mut world.custom_names[i]);
            }
            for (i, field) in self.fgfields.iter().enumerate() {
                self.connect(world, field, move |world| {
                    &mut world.custom_colors[i].foreground
                });
            }
            for (i, field) in self.bgfields.iter().enumerate() {
                self.connect(world, field, move |world| {
                    &mut world.custom_colors[i].background
                });
            }
        }
    }

    fn setcolors(&self, colors: &[RColorPair]) {
        for ((fgfield, bgfield), color) in self
            .fgfields
            .iter()
            .zip(self.bgfields.iter())
            .zip(colors.iter())
        {
            fgfield.set_palette_color(ColorRole::Button, &color.foreground);
            bgfield.set_palette_color(ColorRole::Button, &color.background);
        }
    }

    #[slot(SlotNoArgs)]
    fn reset(&self) {
        let colors = Colors::default_custom();
        self.setcolors(&colors);
        self.world.upgrade().unwrap().borrow_mut().custom_colors = colors;
    }

    fn recolor<F>(&self, mut adjust: F)
    where
        F: FnMut(&RColor) -> RColor,
    {
        for ((fgfield, bgfield), color) in self.fgfields.iter().zip(self.bgfields.iter()).zip(
            self.world
                .upgrade()
                .unwrap()
                .borrow_mut()
                .custom_colors
                .iter_mut(),
        ) {
            let foreground = adjust(&color.foreground);
            fgfield.set_palette_color(ColorRole::Button, &foreground);
            let background = adjust(&color.background);
            bgfield.set_palette_color(ColorRole::Button, &background);
            *color = RColorPair {
                foreground,
                background,
            };
        }
    }

    #[slot(SlotNoArgs)]
    fn invert(&self) {
        self.recolor(|color| color.invert());
    }

    #[slot(SlotNoArgs)]
    fn random(&self) {
        let mut randoms = [0; 3 * 16 * 2];
        rand::thread_rng().fill_bytes(&mut randoms);
        let mut byte = randoms.iter();
        self.recolor(|color| {
            RColor::rgba(
                *byte.next().unwrap(),
                *byte.next().unwrap(),
                *byte.next().unwrap(),
                color.alpha(),
            )
        });
    }

    #[slot(SlotNoArgs)]
    fn saturate(&self) {
        self.recolor(|color| color.saturate(RATE_SATURATE));
    }

    #[slot(SlotNoArgs)]
    fn desaturate(&self) {
        self.recolor(|color| color.saturate(-RATE_SATURATE));
    }

    #[slot(SlotNoArgs)]
    fn lighter(&self) {
        self.recolor(|color| color.reshade(RATE_RESHADE));
    }

    #[slot(SlotNoArgs)]
    fn darker(&self) {
        self.recolor(|color| color.reshade(-RATE_RESHADE));
    }
}
