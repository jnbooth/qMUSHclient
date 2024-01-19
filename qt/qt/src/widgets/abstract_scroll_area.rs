use qt_widgets as q;

use super::scrollbar::QScrollBar;

qt_binding!(
    AbstractScrollAreaBinding,
    q::QAbstractScrollArea,
    super::frame::FrameBinding
);

impl AbstractScrollAreaBinding {
    pub fn horizontal_scroll_bar(&self) -> QScrollBar {
        QScrollBar::get_horizontal(&self.inner)
    }

    pub fn vertical_scroll_bar(&self) -> QScrollBar {
        QScrollBar::get_vertical(&self.inner)
    }
}
