use std::collections;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::iter::FromIterator;
use std::os::raw::*;

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use cpp_core::{CppBox, Ref};
use qt_core::iter::{QEntryIterable, QIntoIterator};
use qt_core::q_meta_type::Type;
use qt_core::*;
use uuid::Uuid;

use super::QList;

const TYPE_NAMES: [&str; 88] = [
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
pub fn type_name(ty: Type) -> &'static str {
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
pub struct RVariant(pub(super) CppBox<QVariant>);

impl From<CppBox<QVariant>> for RVariant {
    fn from(value: CppBox<QVariant>) -> Self {
        Self(value)
    }
}

impl RVariant {
    pub fn qtype(&self) -> Type {
        Type::from(unsafe { self.0.type_() }.to_int())
    }
}

macro_rules! impl_from {
    ($me:ident, $t:ty) => {
        impl From<$t> for RVariant {
            fn from(value: $t) -> Self {
                Self(unsafe { QVariant::$me(value) })
            }
        }
    };
}

macro_rules! impl_from_ref {
    ($me:ident, $t:ty) => {
        impl From<&CppBox<$t>> for RVariant {
            fn from(value: &CppBox<$t>) -> Self {
                Self(unsafe { QVariant::$me(value) })
            }
        }
        impl From<CppBox<$t>> for RVariant {
            fn from(value: CppBox<$t>) -> Self {
                Self(unsafe { QVariant::$me(&value) })
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
impl_from!(from_char, *const c_char);
impl_from_ref!(from_q_byte_array, QByteArray);
impl_from_ref!(from_q_bit_array, QBitArray);
impl_from_ref!(from_q_string, QString);
impl_from_ref!(from_q_latin1_string, QLatin1String);
impl_from_ref!(from_q_string_list, QStringList);
impl_from_ref!(from_q_char, QChar);
impl_from_ref!(from_q_date, QDate);
impl_from_ref!(from_q_time, QTime);
impl_from_ref!(from_q_date_time, QDateTime);
impl_from_ref!(from_q_list_of_q_variant, QListOfQVariant);
impl_from_ref!(from_q_map_of_q_string_q_variant, QMapOfQStringQVariant);
impl_from_ref!(from_q_hash_of_q_string_q_variant, QHashOfQStringQVariant);
impl_from_ref!(from_q_size, QSize);
impl_from_ref!(from_q_size_f, QSizeF);
impl_from_ref!(from_q_point, QPoint);
impl_from_ref!(from_q_line, QLine);
impl_from_ref!(from_q_line_f, QLineF);
impl_from_ref!(from_q_rect, QRect);
impl_from_ref!(from_q_locale, QLocale);
impl_from_ref!(from_q_reg_exp, QRegExp);
impl_from_ref!(from_q_regular_expression, QRegularExpression);
impl_from_ref!(from_q_url, QUrl);
impl_from_ref!(from_q_easing_curve, QEasingCurve);
impl_from_ref!(from_q_uuid, QUuid);
impl_from_ref!(from_q_model_index, QModelIndex);
impl_from_ref!(from_q_persistent_model_index, QPersistentModelIndex);
impl_from_ref!(from_q_json_value, QJsonValue);
impl_from_ref!(from_q_json_object, QJsonObject);
impl_from_ref!(from_q_json_array, QJsonArray);
impl_from_ref!(from_q_json_document, QJsonDocument);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    MismatchedType(Type),
    InvalidTime(Type, i64, u32),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MismatchedType(ty) => write!(f, "MismatchedType({})", type_name(*ty)),
            Self::InvalidTime(ty, secs, nano) => {
                write!(f, "InvalidTime({}, {}:{:-<9})", type_name(*ty), secs, nano)
            }
        }
    }
}

impl std::error::Error for Error {}

macro_rules! impl_try_from {
    ($me:ident, $t:ty, $($qt:ident),+) => {
        impl TryFrom<RVariant> for $t {
            type Error = Error;

            fn try_from(value: RVariant) -> Result<Self, Error> {
                let hastype = value.qtype();
                if $(hastype == Type::$qt)||+ {
                    Ok(unsafe { value.0.$me() })
                } else {
                    Err(Error::MismatchedType(hastype))
                }
            }
        }
    };
}
macro_rules! impl_try_box {
    ($me:ident, $t:ident) => {
        impl_try_from!($me, CppBox<$t>, $t);
    };
    ($me:ident, $t:ty, $($qt:ident),+) => {
        impl_try_from!($me, CppBox<$t>, $($qt),+);
    };
}

impl_try_box!(to_bit_array, QBitArray);
impl_try_from!(to_bool, bool, Bool);
impl_try_box!(to_byte_array, QByteArray);
impl_try_box!(to_char, QChar);
impl_try_box!(to_date, QDate);
impl_try_box!(to_date_time, QDateTime);
impl_try_from!(to_double_0a, c_double, Double);
impl_try_box!(to_easing_curve, QEasingCurve);
impl_try_from!(to_float_0a, c_float, Float);
impl_try_box!(to_hash, QHashOfQStringQVariant, QVariantHash);
impl_try_from!(to_int_0a, c_int, Int);
impl_try_box!(to_json_array, QJsonArray);
impl_try_box!(to_json_document, QJsonDocument);
impl_try_box!(to_json_object, QJsonObject);
impl_try_box!(to_json_value, QJsonValue);
impl_try_box!(to_line, QLine);
impl_try_box!(to_line_f, QLineF);
impl_try_box!(to_list, QListOfQVariant, QVariantList, QStringList);
impl_try_box!(to_locale, QLocale);
impl_try_from!(to_long_long_0a, i64, LongLong);
impl_try_box!(to_map, QMapOfQStringQVariant, QVariantMap);
impl_try_box!(to_model_index, QModelIndex);
impl_try_box!(to_persistent_model_index, QPersistentModelIndex);
impl_try_box!(to_point, QPoint);
impl_try_box!(to_point_f, QPointF);
impl_try_box!(to_rect, QRect);
impl_try_box!(to_rect_f, QRectF);
impl_try_box!(to_reg_exp, QRegExp);
impl_try_box!(to_regular_expression, QRegularExpression);
impl_try_box!(to_size, QSize);
impl_try_box!(to_size_f, QSizeF);
impl_try_box!(to_string, QString);
impl_try_box!(to_string_list, QStringList, QStringList, QString);
impl_try_box!(to_time, QTime);
impl_try_from!(to_u_int_0a, c_uint, UInt);
impl_try_from!(to_u_long_long_0a, u64, ULongLong);
impl_try_box!(to_url, QUrl);
impl_try_box!(to_uuid, QUuid);

impl From<char> for RVariant {
    fn from(value: char) -> Self {
        RVariant::from(value as u64)
    }
}

impl TryFrom<RVariant> for char {
    type Error = Error;

    fn try_from(value: RVariant) -> Result<Self, Error> {
        const E: Error = Error::MismatchedType(Type::ULongLong);
        char::try_from(u32::try_from(u64::try_from(value)?).map_err(|_| E)?).map_err(|_| E)
    }
}

impl From<NaiveDate> for RVariant {
    fn from(value: NaiveDate) -> Self {
        RVariant::from(unsafe {
            QDate::new_3a(value.year(), value.month() as i32, value.day() as i32)
        })
    }
}

impl TryFrom<RVariant> for NaiveDate {
    type Error = Error;

    fn try_from(value: RVariant) -> Result<Self, Error> {
        let date = CppBox::<QDate>::try_from(value)?;
        unsafe {
            let year = date.year_0a();
            let day = date.day_of_year_0a() as u32;
            NaiveDate::from_yo_opt(year, day)
                .ok_or_else(|| Error::InvalidTime(Type::QDate, year as i64, day))
        }
    }
}

const MILLI: u32 = 1_000;
const NANO: u32 = 1_000_000;

impl From<NaiveTime> for RVariant {
    fn from(value: NaiveTime) -> Self {
        let secs = value.num_seconds_from_midnight();
        let nano = value.nanosecond();
        let milli = secs * MILLI + nano / NANO;
        RVariant::from(unsafe { QTime::from_m_secs_since_start_of_day(milli as i32) })
    }
}

impl TryFrom<RVariant> for NaiveTime {
    type Error = Error;

    fn try_from(value: RVariant) -> Result<Self, Error> {
        let time = CppBox::<QTime>::try_from(value)?;
        let msecs = unsafe { time.msecs_since_start_of_day() } as u32;
        let secs = msecs / MILLI;
        let nano = (msecs % MILLI) * NANO;
        NaiveTime::from_num_seconds_from_midnight_opt(secs, nano)
            .ok_or_else(|| Error::InvalidTime(Type::QTime, secs as i64, nano))
    }
}

impl From<NaiveDateTime> for RVariant {
    fn from(value: NaiveDateTime) -> Self {
        RVariant::from(unsafe { QDateTime::from_m_secs_since_epoch_i64(value.timestamp_millis()) })
    }
}

impl TryFrom<RVariant> for NaiveDateTime {
    type Error = Error;

    fn try_from(value: RVariant) -> Result<Self, Error> {
        let datetime = CppBox::<QDateTime>::try_from(value)?;
        let msecs = unsafe { datetime.to_m_secs_since_epoch() };
        let secs = msecs / MILLI as i64;
        let nano = (msecs % MILLI as i64) as u32 * NANO;
        NaiveDateTime::from_timestamp_opt(secs, nano)
            .ok_or_else(|| Error::InvalidTime(Type::QTime, secs, nano))
    }
}

impl From<Uuid> for RVariant {
    fn from(value: Uuid) -> Self {
        RVariant::from(unsafe { QUuid::from_rfc4122(&QByteArray::from_slice(value.as_bytes())) })
    }
}

impl TryFrom<RVariant> for Uuid {
    type Error = Error;

    fn try_from(value: RVariant) -> Result<Self, Error> {
        let quuid = CppBox::<QUuid>::try_from(value)?;
        let array = unsafe { quuid.to_rfc4122() };
        Ok(Uuid::from_slice(&array.to_vec()).unwrap())
    }
}

impl From<String> for RVariant {
    fn from(value: String) -> Self {
        RVariant::from(QString::from_std_str(value))
    }
}

impl TryFrom<RVariant> for String {
    type Error = Error;

    fn try_from(value: RVariant) -> Result<Self, Error> {
        Ok(CppBox::<QString>::try_from(value)?.to_std_string())
    }
}

impl From<&str> for RVariant {
    fn from(value: &str) -> Self {
        RVariant::from(QString::from_std_str(value))
    }
}

impl From<&String> for RVariant {
    fn from(value: &String) -> Self {
        RVariant::from(QString::from_std_str(value))
    }
}

impl From<Vec<String>> for RVariant {
    fn from(value: Vec<String>) -> Self {
        unsafe {
            RVariant::from(QStringList::from_iter(
                value.into_iter().map(|x| QString::from_std_str(&x)),
            ))
        }
    }
}

impl From<&[String]> for RVariant {
    fn from(value: &[String]) -> Self {
        unsafe {
            RVariant::from(QStringList::from_iter(
                value.into_iter().map(QString::from_std_str),
            ))
        }
    }
}

impl From<Vec<&str>> for RVariant {
    fn from(value: Vec<&str>) -> Self {
        unsafe {
            RVariant::from(QStringList::from_iter(
                value.into_iter().map(QString::from_std_str),
            ))
        }
    }
}

impl From<&[&str]> for RVariant {
    fn from(value: &[&str]) -> Self {
        unsafe {
            RVariant::from(QStringList::from_iter(
                value.into_iter().map(QString::from_std_str),
            ))
        }
    }
}

impl TryFrom<RVariant> for Vec<String> {
    type Error = Error;

    fn try_from(value: RVariant) -> Result<Self, Error> {
        unsafe {
            Ok(CppBox::<QStringList>::try_from(value)?
                .into_iter()
                .map(|x| x.to_std_string())
                .collect())
        }
    }
}

// Qt's copy-constructors are secretly reference counters. Although `T` could be cast directly
// from the `Ref`, `new_copy` signals to Qt that this function wants ownership of the data.
// There might be a way to do it without the copy constructor, but whenever I've tried, it seemed
// like it worked and then later everything crashed due to Qt's atomic counters apparently
// reaching impossible results. `CppBox`es don't do unexpected things like that, so I'm sticking
// with them for now.
fn recast<T: TryFrom<RVariant, Error = Error>>(variant: Ref<QVariant>) -> Result<T, Error> {
    T::try_from(RVariant(unsafe { QVariant::new_copy(variant) }))
}

fn from_map<K, V, T>(value: T) -> RVariant
where
    K: AsRef<str>,
    V: Into<RVariant>,
    T: IntoIterator<Item = (K, V)>,
{
    unsafe {
        let hashmap = QHashOfQStringQVariant::new();
        for (k, v) in value {
            hashmap.insert(&QString::from_std_str(k), &v.into().0);
        }
        RVariant::from(hashmap)
    }
}

fn to_map<V, T>(value: RVariant) -> Result<T, Error>
where
    V: TryFrom<RVariant, Error = Error>,
    T: FromIterator<(String, V)>,
{
    let hashmap = CppBox::<QHashOfQStringQVariant>::try_from(value)?;
    unsafe {
        hashmap
            .entries()
            .map(|(k, v)| Ok((k.to_std_string(), recast(v)?)))
            .collect()
    }
}

impl<K: AsRef<str>, V: Into<RVariant>> From<hashbrown::HashMap<K, V>> for RVariant {
    fn from(value: hashbrown::HashMap<K, V>) -> Self {
        from_map(value)
    }
}
impl<V: TryFrom<RVariant, Error = Error>> TryFrom<RVariant> for hashbrown::HashMap<String, V> {
    type Error = Error;

    fn try_from(value: RVariant) -> Result<Self, Error> {
        to_map(value)
    }
}

impl<K: AsRef<str>, V: Into<RVariant>> From<collections::HashMap<K, V>> for RVariant {
    fn from(value: collections::HashMap<K, V>) -> Self {
        from_map(value)
    }
}
impl<V: TryFrom<RVariant, Error = Error>> TryFrom<RVariant> for collections::HashMap<String, V> {
    type Error = Error;

    fn try_from(value: RVariant) -> Result<Self, Error> {
        to_map(value)
    }
}

impl<K: AsRef<str>, V: Into<RVariant>> From<BTreeMap<K, V>> for RVariant {
    fn from(value: BTreeMap<K, V>) -> Self {
        from_map(value)
    }
}
impl<V: TryFrom<RVariant, Error = Error>> TryFrom<RVariant> for BTreeMap<String, V> {
    type Error = Error;

    fn try_from(value: RVariant) -> Result<Self, Error> {
        to_map(value)
    }
}
