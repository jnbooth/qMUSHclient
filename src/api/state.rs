use std::cell::Cell;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ApiState {
    pub compressing: Cell<bool>,
    pub mxp_active: Cell<bool>,
    pub no_echo: Cell<bool>,
    pub pueblo_active: Cell<bool>,
}
