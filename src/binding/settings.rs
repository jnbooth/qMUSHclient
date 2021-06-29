use super::variant::{self, RVariant};
use super::{Binding, QList};
use cpp_core::CppBox;
use qt_core::q_meta_type::Type;
use qt_core::{QBox, QSettings, QString, QStringList};
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;

pub trait StoreAsQ {
    type Store;
    type ListStore: QList<Item = Self::Store>;
    fn from_store(store: Self::Store) -> Self;
    fn to_store(&self) -> Self::Store;
}

impl StoreAsQ for String {
    type Store = CppBox<QString>;
    type ListStore = QStringList;
    fn from_store(store: Self::Store) -> Self {
        store.to_std_string()
    }
    fn to_store(&self) -> Self::Store {
        QString::from_std_str(self)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    NotFound,
    MismatchedType(Type),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "Error::NotFound"),
            Self::MismatchedType(ty) => {
                write!(f, "Error::MismatchedType({}", variant::type_name(*ty))
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<variant::Error> for Error {
    fn from(value: variant::Error) -> Self {
        match value {
            variant::Error::MismatchedType(Type::UnknownType) => Self::NotFound,
            variant::Error::MismatchedType(ty) => Self::MismatchedType(ty),
        }
    }
}

#[derive(Debug, Binding)]
pub struct RSettings(QBox<QSettings>);

impl RSettings {
    pub fn new(organization: &str, name: &str) -> Self {
        Self(unsafe {
            QSettings::from_2_q_string(
                &QString::from_std_str(organization),
                &QString::from_std_str(name),
            )
        })
    }
    fn qget(&self, key: &str) -> RVariant {
        unsafe { self.0.value_1a(&QString::from_std_str(key)) }.into()
    }
    pub fn get<T>(&self, key: &str) -> Result<T, Error>
    where
        T: StoreAsQ,
        T::Store: TryFrom<RVariant>,
        Error: From<<T::Store as TryFrom<RVariant>>::Error>,
    {
        let attempt = self.qget(key);
        let store = T::Store::try_from(attempt)?;
        Ok(T::from_store(store))
    }

    pub fn set<T: Into<RVariant>>(&self, key: &str, val: T) {
        let variant: RVariant = val.into();
        unsafe {
            self.0
                .set_value(&QString::from_std_str(key), &variant.into_raw());
        }
    }
    pub fn get_list<I>(&self, key: &str) -> Result<impl Iterator<Item = I>, Error>
    where
        I: StoreAsQ,
        CppBox<I::ListStore>: TryFrom<RVariant>,
        Error: From<<CppBox<I::ListStore> as TryFrom<RVariant>>::Error>,
    {
        let attempt = self.qget(key);
        let store: CppBox<I::ListStore> = CppBox::try_from(attempt)?;
        Ok(unsafe { store.cpp_iter().map(StoreAsQ::from_store) })
    }

    /// # Safety
    ///
    /// All items produced by `iter` must be valid.
    pub unsafe fn set_list<I>(&self, key: &str, iter: I)
    where
        I: Iterator,
        I::Item: Deref,
        <I::Item as Deref>::Target: StoreAsQ,
        CppBox<<<I::Item as Deref>::Target as StoreAsQ>::ListStore>: Into<RVariant>,
    {
        unsafe {
            self.set(
                key,
                <<<I::Item as Deref>::Target as StoreAsQ>::ListStore>::from_iter(
                    iter.map(|x| x.to_store()),
                ),
            );
        }
    }
}
