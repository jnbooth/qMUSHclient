mod as_ref_hashmap;
use as_ref_hashmap::AsRefHashMap;

mod case_fold;
pub use case_fold::{CaseFold, CaseFoldMap, ToCaseFold};

mod case_fold_impl;
pub use case_fold_impl::{ascii, unicode};
