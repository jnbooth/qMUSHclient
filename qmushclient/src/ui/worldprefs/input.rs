use std::cell::RefCell;
use std::rc::{Rc, Weak};

use qt::traits::Widget;
use qt_widgets::*;
use tr::TrContext;

use super::PrefPageExt;
use crate::ui::uic;
use crate::world::World;

#[derive(Debug, Widget, TrContext)]
pub struct PrefsCommands {
    ui: uic::PrefsCommands,
    world: Weak<RefCell<World>>,
}
impl_prefpage!(PrefsCommands);
impl_prefpageext!(PrefsCommands);

impl PrefsCommands {
    fn init(self: &Rc<Self>) {
        // SAFETY: fields are valid.
        unsafe {
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
            let ui = &self.ui;
            self.connect_font(&ui.input_font, &ui.input_font_size, |world| {
                &mut world.input_font
            });
        }
    }
}
