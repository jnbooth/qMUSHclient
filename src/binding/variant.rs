use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::os::raw::*;

use cpp_core::CppBox;
use qt_core::q_meta_type::Type;
use qt_core::*;

use super::Binding;

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
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
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
    } else if (i > 0 && i < 57) || (i > 63 && i < TYPE_NAMES.len()) {
        TYPE_NAMES[i]
    } else {
        TYPE_NAMES[0]
    }
}

#[derive(Debug, Binding)]
pub struct RVariant(CppBox<QVariant>);

impl RVariant {
    pub fn qtype(&self) -> Type {
        unsafe { self.0.type_() }.to_int().into()
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
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MismatchedType(ty) => write!(f, "Error::MismatchedType({}", type_name(*ty)),
        }
    }
}

impl std::error::Error for Error {}

macro_rules! impl_try_from {
    ($me:ident, $t:ty, $($qt:ident),+) => {
        impl TryFrom<RVariant> for $t {
            type Error = Error;

            fn try_from(value: RVariant) -> Result<Self, Self::Error> {
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
