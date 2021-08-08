use std::convert::TryFrom;
use std::iter::FromIterator;

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
pub struct RSettings(QBox<QSettings>);

impl Default for RSettings {
    fn default() -> Self {
        Self(unsafe { QSettings::new() })
    }
}

impl RSettings {
    pub fn new(organization: &str, name: &str) -> Self {
        Self(unsafe {
            QSettings::from_2_q_string(
                &QString::from_std_str(organization),
                &QString::from_std_str(name),
            )
        })
    }
    fn qget(&self, key: &str) -> Result<RVariant, Error> {
        unsafe {
            let key = QString::from_std_str(key);
            if self.0.contains(&key) {
                Ok(RVariant::from(self.0.value_1a(&key)))
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
            self.0
                .set_value(&QString::from_std_str(key), &RVariant::from(val).0);
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
                QListOfQVariant::from_iter(iter.into_iter().map(|x| x.into().0)),
            );
        }
    }
}
