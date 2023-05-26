mod as_ref_hashmap;

mod case_fold;
pub use case_fold::{CaseFold, CaseFoldMap};

mod case_fold_impl;
pub use case_fold_impl::{ascii, unicode};

mod to_case_fold;
pub use to_case_fold::ToCaseFold;
