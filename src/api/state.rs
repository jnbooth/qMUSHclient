use std::cell::Cell;
use std::time::Duration;

use crate::client::state::Mccp;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ApiState {
    pub linecount: Cell<u64>,

    pub disconnect_ok: Cell<bool>,
    pub total_connect_duration: Cell<Duration>,

    pub mxp_active: Cell<bool>,
    pub pueblo_active: Cell<bool>,

    pub compressing: Cell<bool>,
    pub mccp_ver: Cell<Option<Mccp>>,
    pub supports_mccp_2: Cell<bool>,
    pub no_echo: Cell<bool>,
}
