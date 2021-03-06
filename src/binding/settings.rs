use std::iter::FromIterator;
use std::path::PathBuf;

use cpp_core::CppBox;
use qt_core::iter::QIntoIterator;
use qt_core::*;

use super::list::QList;
use super::variant::{self, RVariant};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error(display = "key not found in settings")]
    NotFound,
    TypeError(variant::Error),
}

#[derive(Debug)]
pub struct RSettings {
    inner: QBox<QSettings>,
}

impl Default for RSettings {
    fn default() -> Self {
        Self {
            inner: unsafe { QSettings::new() },
        }
    }
}

impl RSettings {
    pub fn new(organization: &str, name: &str) -> Self {
        Self {
            inner: unsafe {
                QSettings::from_2_q_string(
                    &QString::from_std_str(organization),
                    &QString::from_std_str(name),
                )
            },
        }
    }
    fn qget(&self, key: &str) -> Result<RVariant, Error> {
        unsafe {
            let key = QString::from_std_str(key);
            if self.inner.contains(&key) {
                Ok(RVariant::from(self.inner.value_1a(&key)))
            } else {
                Err(Error::NotFound)
            }
        }
    }
    pub fn get<T>(&self, key: &str) -> Result<T, Error>
    where
        T: TryFrom<RVariant>,
        Error: From<T::Error>,
    {
        Ok(T::try_from(self.qget(key)?)?)
    }

    pub fn set<T>(&self, key: &str, val: T)
    where
        RVariant: From<T>,
    {
        unsafe {
            self.inner
                .set_value(&QString::from_std_str(key), &RVariant::from(val).inner);
        }
    }

    pub fn get_list<T>(&self, key: &str) -> Result<T, Error>
    where
        T: IntoIterator + FromIterator<T::Item>,
        T::Item: TryFrom<RVariant>,
        Error: From<<T::Item as TryFrom<RVariant>>::Error>,
    {
        let list = CppBox::<QListOfQVariant>::try_from(self.qget(key)?)?;
        let vec: Result<_, _> = unsafe { list.into_iter() }
            .map(|x| T::Item::try_from(RVariant::from(x)))
            .collect();
        Ok(vec?)
    }

    pub fn set_list<T: IntoIterator>(&self, key: &str, iter: T)
    where
        T::Item: Into<RVariant>,
    {
        unsafe {
            self.set(
                key,
                QListOfQVariant::from_iter(iter.into_iter().map(|x| x.into().inner)),
            );
        }
    }

    pub fn path(&self) -> PathBuf {
        unsafe { self.inner.file_name().to_std_string().into() }
    }
}
