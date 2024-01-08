use std::collections::{BTreeMap, HashMap};
use std::fmt::{self, Display, Formatter};
use std::iter::FromIterator;
use std::os::raw::*;

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use cpp_core::{CppBox, Ref};
use qt_core::iter::{QEntryIterable, QIntoIterator};
use qt_core::q_meta_type::Type;
use uuid::Uuid;
use {qt_core as q, qt_gui as g, qt_widgets as w};

use crate::traits::QList;

const TYPE_NAMES: &[&str] = &[
    "UnknownType",
    "Bool",
    "Int",
    "UInt",
    "LongLong",
    "ULongLong",
    "Double",
    "QChar",
    "QVariantMap",
    "QVariantList",
    "QString",
    "QStringList",
    "QByteArray",
    "QBitArray",
    "QDate",
    "QTime",
    "QDateTime",
    "QUrl",
    "QLocale",
    "QRect",
    "QRectF",
    "QSize",
    "QSizeF",
    "QLine",
    "QLineF",
    "QPoint",
    "QPointF",
    "QRegExp",
    "QVariantHash",
    "QEasingCurve",
    "QUuid",
    "VoidStar",
    "Long",
    "Short",
    "Char",
    "ULong",
    "UShort",
    "UChar",
    "Float",
    "QObjectStar",
    "SChar",
    "QVariant",
    "QModelIndex",
    "Void",
    "QRegularExpression",
    "QJsonValue",
    "QJsonObject",
    "QJsonArray",
    "QJsonDocument",
    "QByteArrayList",
    "QPersistentModelIndex",
    "Nullptr",
    "QCborSimpleType",
    "QCborValue",
    "QCborArray",
    "QCborMap",
    "UnknownType",
    "UnknownType",
    "UnknownType",
    "UnknownType",
    "UnknownType",
    "UnknownType",
    "UnknownType",
    "UnknownType",
    "QFont",
    "QPixmap",
    "QBrush",
    "QColor",
    "QPalette",
    "QIcon",
    "QImage",
    "QPolygon",
    "QRegion",
    "QBitmap",
    "QCursor",
    "QKeySequence",
    "QPen",
    "QTextLength",
    "QTextFormat",
    "QMatrix",
    "QTransform",
    "QMatrix4x4",
    "QVector2D",
    "QVector3D",
    "QVector4D",
    "QQuaternion",
    "QPolygonF",
    "QColorSpace",
];
pub const fn type_name(ty: Type) -> &'static str {
    let i = ty.to_int() as usize;
    if i == 121 {
        "QSizePolicy"
    } else if i == 1024 {
        "User"
    } else if i > 0 && i < TYPE_NAMES.len() {
        TYPE_NAMES[i]
    } else {
        TYPE_NAMES[0]
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct QVariant {
    pub(crate) inner: CppBox<q::QVariant>,
}

impl_eq_cpp!(QVariant);

impl From<CppBox<q::QVariant>> for QVariant {
    fn from(value: CppBox<q::QVariant>) -> Self {
        Self { inner: value }
    }
}

impl QVariant {
    pub fn qtype(&self) -> Type {
        Type::from(unsafe { self.inner.type_() }.to_int())
    }

    fn from_strings<S: AsRef<str>, I: IntoIterator<Item = S>>(iter: I) -> Self {
        unsafe {
            QVariant::from(q::QStringList::from_iter(
                iter.into_iter()
                    .map(|x| q::QString::from_std_str(x.as_ref())),
            ))
        }
    }

    fn from_map<K, V, T>(value: T) -> Self
    where
        K: AsRef<str>,
        V: Into<QVariant>,
        T: IntoIterator<Item = (K, V)>,
    {
        unsafe {
            let hashmap = q::QHashOfQStringQVariant::new();
            for (k, v) in value {
                hashmap.insert(&q::QString::from_std_str(k), &v.into().inner);
            }
            QVariant::from(hashmap)
        }
    }

    fn into_map<V, T>(self) -> Result<T, Error>
    where
        V: TryFrom<QVariant, Error = Error>,
        T: FromIterator<(String, V)>,
    {
        let hashmap = CppBox::<q::QHashOfQStringQVariant>::try_from(self)?;
        unsafe {
            hashmap
                .entries()
                .map(|(k, v)| Ok((k.to_std_string(), recast(v)?)))
                .collect()
        }
    }
}

macro_rules! impl_from {
    ($me:ident, $t:ty) => {
        impl From<$t> for QVariant {
            fn from(value: $t) -> Self {
                Self {
                    inner: unsafe { q::QVariant::$me(value) },
                }
            }
        }
    };
}

macro_rules! impl_from_ref {
    ($me:ident, $t:ty) => {
        impl From<&CppBox<$t>> for QVariant {
            fn from(value: &CppBox<$t>) -> Self {
                Self {
                    inner: unsafe { q::QVariant::$me(value) },
                }
            }
        }
        impl From<CppBox<$t>> for QVariant {
            fn from(value: CppBox<$t>) -> Self {
                Self {
                    inner: unsafe { q::QVariant::$me(&value) },
                }
            }
        }
    };
}

macro_rules! impl_into {
    ($t:ty) => {
        impl From<&CppBox<$t>> for QVariant {
            fn from(value: &CppBox<$t>) -> Self {
                Self {
                    inner: unsafe { value.to_q_variant() },
                }
            }
        }
        impl From<CppBox<$t>> for QVariant {
            fn from(value: CppBox<$t>) -> Self {
                Self {
                    inner: unsafe { value.to_q_variant() },
                }
            }
        }
    };
}

impl_from!(from_int, c_int);
impl_from!(from_uint, c_uint);
impl_from!(from_i64, i64);
impl_from!(from_u64, u64);
impl_from!(from_bool, bool);
impl_from!(from_double, c_double);
impl_from!(from_float, c_float);
impl_from_ref!(from_q_byte_array, q::QByteArray);
impl_from_ref!(from_q_bit_array, q::QBitArray);
impl_from_ref!(from_q_string, q::QString);
impl_from_ref!(from_q_latin1_string, q::QLatin1String);
impl_from_ref!(from_q_string_list, q::QStringList);
impl_from_ref!(from_q_char, q::QChar);
impl_from_ref!(from_q_date, q::QDate);
impl_from_ref!(from_q_time, q::QTime);
impl_from_ref!(from_q_date_time, q::QDateTime);
impl_from_ref!(from_q_list_of_q_variant, q::QListOfQVariant);
impl_from_ref!(from_q_map_of_q_string_q_variant, q::QMapOfQStringQVariant);
impl_from_ref!(from_q_hash_of_q_string_q_variant, q::QHashOfQStringQVariant);
impl_from_ref!(from_q_size, q::QSize);
impl_from_ref!(from_q_size_f, q::QSizeF);
impl_from_ref!(from_q_point, q::QPoint);
impl_from_ref!(from_q_point_f, q::QPointF);
impl_from_ref!(from_q_line, q::QLine);
impl_from_ref!(from_q_line_f, q::QLineF);
impl_from_ref!(from_q_rect, q::QRect);
impl_from_ref!(from_q_rect_f, q::QRectF);
impl_from_ref!(from_q_locale, q::QLocale);
impl_from_ref!(from_q_reg_exp, q::QRegExp);
impl_from_ref!(from_q_regular_expression, q::QRegularExpression);
impl_from_ref!(from_q_url, q::QUrl);
impl_from_ref!(from_q_easing_curve, q::QEasingCurve);
impl_from_ref!(from_q_uuid, q::QUuid);
impl_from_ref!(from_q_model_index, q::QModelIndex);
impl_from_ref!(from_q_persistent_model_index, q::QPersistentModelIndex);
impl_from_ref!(from_q_json_value, q::QJsonValue);
impl_from_ref!(from_q_json_object, q::QJsonObject);
impl_from_ref!(from_q_json_array, q::QJsonArray);
impl_from_ref!(from_q_json_document, q::QJsonDocument);
impl_into!(g::QColor);
impl_into!(g::QRegion);
impl_into!(g::QKeySequence);
impl_into!(g::QVector2D);
impl_into!(g::QFont);
impl_into!(g::QPolygon);
impl_into!(g::QPolygonF);
impl_into!(g::QMatrix);
impl_into!(g::QTransform);
impl_into!(g::QImage);
impl_into!(g::QPixmap);
impl_into!(g::QBrush);
impl_into!(g::QPen);
impl_into!(g::QTextLength);
impl_into!(g::QTextFormat);
impl_into!(g::QPalette);
impl_into!(g::QIcon);
impl_into!(g::QCursor);
impl_into!(g::QBitmap);
impl_into!(g::QVector3D);
impl_into!(g::QVector4D);
impl_into!(g::QQuaternion);
impl_into!(g::QMatrix4X4);
impl_into!(w::QSizePolicy);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    WrongType { tried: Type, actual: Type },
    InvalidTime { ty: Type, major: i64, minor: u32 },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Self::WrongType { tried, actual } => write!(
                f,
                "tried to convert {} to {}",
                type_name(actual),
                type_name(tried)
            ),
            Self::InvalidTime { ty, major, minor } => {
                write!(
                    f,
                    "invalid time: {}, {}:{:-<9}",
                    type_name(ty),
                    major,
                    minor
                )
            }
        }
    }
}

impl std::error::Error for Error {}

macro_rules! impl_try_from {
    ($me:ident, $t:ty, $qt1:ident$(, $qt:ident)*) => {
        impl TryFrom<QVariant> for $t {
            type Error = Error;

            fn try_from(value: QVariant) -> Result<Self, Error> {
                let actual = value.qtype();
                if (actual == Type::$qt1)$(||(actual == Type::$qt))* {
                    Ok(unsafe { value.inner.$me() })
                } else {
                    Err(Error::WrongType { tried: Type::$qt1, actual })
                }
            }
        }
    };
}
macro_rules! impl_try_box {
    ($me:ident, $t:ty, $($qt:ident),+) => {
        impl_try_from!($me, CppBox<$t>, $($qt),+);
    };
}

impl_try_box!(to_bit_array, q::QBitArray, QBitArray);
impl_try_from!(to_bool, bool, Bool);
impl_try_box!(to_byte_array, q::QByteArray, QByteArray);
impl_try_box!(to_char, q::QChar, QChar);
impl_try_box!(to_date, q::QDate, QDate);
impl_try_box!(to_date_time, q::QDateTime, QDateTime);
impl_try_from!(to_double_0a, c_double, Double);
impl_try_box!(to_easing_curve, q::QEasingCurve, QEasingCurve);
impl_try_from!(to_float_0a, c_float, Float);
impl_try_box!(to_hash, q::QHashOfQStringQVariant, QVariantHash);
impl_try_from!(to_int_0a, c_int, Int);
impl_try_box!(to_json_array, q::QJsonArray, QJsonArray);
impl_try_box!(to_json_document, q::QJsonDocument, QJsonDocument);
impl_try_box!(to_json_object, q::QJsonObject, QJsonObject);
impl_try_box!(to_json_value, q::QJsonValue, QJsonValue);
impl_try_box!(to_line, q::QLine, QLine);
impl_try_box!(to_line_f, q::QLineF, QLineF);
impl_try_box!(to_list, q::QListOfQVariant, QVariantList, QStringList);
impl_try_box!(to_locale, q::QLocale, QLocale);
impl_try_from!(to_long_long_0a, i64, LongLong);
impl_try_box!(to_map, q::QMapOfQStringQVariant, QVariantMap);
impl_try_box!(to_model_index, q::QModelIndex, QModelIndex);
impl_try_box!(
    to_persistent_model_index,
    q::QPersistentModelIndex,
    QPersistentModelIndex
);
impl_try_box!(to_point, q::QPoint, QPoint);
impl_try_box!(to_point_f, q::QPointF, QPointF);
impl_try_box!(to_rect, q::QRect, QRect);
impl_try_box!(to_rect_f, q::QRectF, QRectF);
impl_try_box!(to_reg_exp, q::QRegExp, QRegExp);
impl_try_box!(
    to_regular_expression,
    q::QRegularExpression,
    QRegularExpression
);
impl_try_box!(to_size, q::QSize, QSize);
impl_try_box!(to_size_f, q::QSizeF, QSizeF);
impl_try_box!(to_string, q::QString, QString);
impl_try_box!(to_string_list, q::QStringList, QStringList, QString);
impl_try_box!(to_time, q::QTime, QTime);
impl_try_from!(to_u_int_0a, c_uint, UInt);
impl_try_from!(to_u_long_long_0a, u64, ULongLong);
impl_try_box!(to_url, q::QUrl, QUrl);
impl_try_box!(to_uuid, q::QUuid, QUuid);

impl From<char> for QVariant {
    fn from(value: char) -> Self {
        QVariant::from(value as u64)
    }
}

impl TryFrom<QVariant> for char {
    type Error = Error;

    fn try_from(value: QVariant) -> Result<Self, Error> {
        let try64 = u64::try_from(value)?;
        let try32 = u32::try_from(try64).map_err(|_| Error::WrongType {
            tried: Type::ULong,
            actual: Type::ULongLong,
        })?;
        char::try_from(try32).map_err(|_| Error::WrongType {
            tried: Type::ULong,
            actual: Type::Char,
        })
    }
}

impl From<NaiveDate> for QVariant {
    fn from(value: NaiveDate) -> Self {
        QVariant::from(unsafe {
            q::QDate::new_3a(value.year(), value.month() as c_int, value.day() as c_int)
        })
    }
}

impl TryFrom<QVariant> for NaiveDate {
    type Error = Error;

    fn try_from(value: QVariant) -> Result<Self, Error> {
        let date = CppBox::<q::QDate>::try_from(value)?;
        unsafe {
            let year = date.year_0a();
            let day = date.day_of_year_0a() as u32;
            NaiveDate::from_yo_opt(year, day).ok_or(Error::InvalidTime {
                ty: Type::QDate,
                major: year as i64,
                minor: day,
            })
        }
    }
}

const MILLI: u32 = 1_000;
const NANO: u32 = 1_000_000;

impl From<NaiveTime> for QVariant {
    fn from(value: NaiveTime) -> Self {
        let secs = value.num_seconds_from_midnight();
        let nano = value.nanosecond();
        let milli = secs * MILLI + nano / NANO;
        QVariant::from(unsafe { q::QTime::from_m_secs_since_start_of_day(milli as c_int) })
    }
}

impl TryFrom<QVariant> for NaiveTime {
    type Error = Error;

    fn try_from(value: QVariant) -> Result<Self, Error> {
        let time = CppBox::<q::QTime>::try_from(value)?;
        let msecs = unsafe { time.msecs_since_start_of_day() } as u32;
        let secs = msecs / MILLI;
        let nano = (msecs % MILLI) * NANO;
        NaiveTime::from_num_seconds_from_midnight_opt(secs, nano).ok_or(Error::InvalidTime {
            ty: Type::QTime,
            major: secs as i64,
            minor: nano,
        })
    }
}

impl From<NaiveDateTime> for QVariant {
    fn from(value: NaiveDateTime) -> Self {
        QVariant::from(unsafe {
            q::QDateTime::from_m_secs_since_epoch_i64(value.timestamp_millis())
        })
    }
}

impl TryFrom<QVariant> for NaiveDateTime {
    type Error = Error;

    fn try_from(value: QVariant) -> Result<Self, Error> {
        let datetime = CppBox::<q::QDateTime>::try_from(value)?;
        let msecs = unsafe { datetime.to_m_secs_since_epoch() };
        let secs = msecs / MILLI as i64;
        let nano = (msecs % MILLI as i64) as u32 * NANO;
        NaiveDateTime::from_timestamp_opt(secs, nano).ok_or(Error::InvalidTime {
            ty: Type::QTime,
            major: secs,
            minor: nano,
        })
    }
}

impl From<Uuid> for QVariant {
    fn from(value: Uuid) -> Self {
        QVariant::from(unsafe {
            q::QUuid::from_rfc4122(&q::QByteArray::from_slice(value.as_bytes()))
        })
    }
}

impl TryFrom<QVariant> for Uuid {
    type Error = Error;

    fn try_from(value: QVariant) -> Result<Self, Error> {
        let quuid = CppBox::<q::QUuid>::try_from(value)?;
        let array = unsafe { quuid.to_rfc4122() };
        Ok(Uuid::from_slice(&array.to_vec()).unwrap())
    }
}

impl From<String> for QVariant {
    fn from(value: String) -> Self {
        QVariant::from(q::QString::from_std_str(value))
    }
}

impl TryFrom<QVariant> for String {
    type Error = Error;

    fn try_from(value: QVariant) -> Result<Self, Error> {
        Ok(CppBox::<q::QString>::try_from(value)?.to_std_string())
    }
}

impl From<&str> for QVariant {
    fn from(value: &str) -> Self {
        QVariant::from(q::QString::from_std_str(value))
    }
}

impl From<&String> for QVariant {
    fn from(value: &String) -> Self {
        QVariant::from(q::QString::from_std_str(value))
    }
}

impl From<Vec<String>> for QVariant {
    fn from(value: Vec<String>) -> Self {
        Self::from_strings(value)
    }
}

impl From<&[String]> for QVariant {
    fn from(value: &[String]) -> Self {
        Self::from_strings(value)
    }
}

impl From<Vec<&str>> for QVariant {
    fn from(value: Vec<&str>) -> Self {
        Self::from_strings(value)
    }
}

impl From<&[&str]> for QVariant {
    fn from(value: &[&str]) -> Self {
        Self::from_strings(value)
    }
}

impl TryFrom<QVariant> for Vec<String> {
    type Error = Error;

    fn try_from(value: QVariant) -> Result<Self, Error> {
        unsafe {
            Ok(CppBox::<q::QStringList>::try_from(value)?
                .into_iter()
                .map(|x| x.to_std_string())
                .collect())
        }
    }
}

impl<K: AsRef<str>> From<&HashMap<K, String>> for QVariant {
    fn from(value: &HashMap<K, String>) -> Self {
        Self::from_map(value)
    }
}
impl<K: AsRef<str>, V: Into<QVariant>> From<HashMap<K, V>> for QVariant {
    fn from(value: HashMap<K, V>) -> Self {
        Self::from_map(value)
    }
}
impl<V: TryFrom<QVariant, Error = Error>> TryFrom<QVariant> for HashMap<String, V> {
    type Error = Error;

    fn try_from(value: QVariant) -> Result<Self, Error> {
        value.into_map()
    }
}

impl<K: AsRef<str>> From<&BTreeMap<K, String>> for QVariant {
    fn from(value: &BTreeMap<K, String>) -> Self {
        Self::from_map(value)
    }
}
impl<K: AsRef<str>, V: Into<QVariant>> From<BTreeMap<K, V>> for QVariant {
    fn from(value: BTreeMap<K, V>) -> Self {
        Self::from_map(value)
    }
}
impl<V: TryFrom<QVariant, Error = Error>> TryFrom<QVariant> for BTreeMap<String, V> {
    type Error = Error;

    fn try_from(value: QVariant) -> Result<Self, Error> {
        value.into_map()
    }
}

// Qt's copy-constructors are secretly reference counters. Although `T` could be cast directly
// from the `Ref`, `new_copy` signals to Qt that this function wants ownership of the data.
// There might be a way to do it without the copy constructor, but whenever I've tried, it seemed
// like it worked and then later everything crashed due to Qt's atomic counters apparently
// reaching impossible results. `CppBox`es don't do unexpected things like that, so I'm sticking
// with them for now.
fn recast<T: TryFrom<QVariant, Error = Error>>(variant: Ref<q::QVariant>) -> Result<T, Error> {
    T::try_from(QVariant {
        inner: unsafe { q::QVariant::new_copy(variant) },
    })
}
