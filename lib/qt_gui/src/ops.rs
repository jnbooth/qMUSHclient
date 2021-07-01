#![cfg_attr(feature = "ritual_rustdoc_nightly", feature(doc_cfg))]
//! Functions that provide access to C++ operators

/// <p>Returns <code>true</code> if page layout <i>lhs</i> is equal to page layout <i>rhs</i>, i.e. if all the attributes are exactly equal.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator==(QKeySequence::StandardKey key, QKeyEvent* e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qpagelayout.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(const QPageLayout &lhs, const QPageLayout &rhs)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if page layout <i>lhs</i> is equal to page layout <i>rhs</i>, i.e. if all the attributes are exactly equal.</p>
/// <p>Note that this is a strict equality, especially for page size where the <a href="http://doc.qt.io/qt-5/qpagesize.html">QPageSize</a> ID, name and size must exactly match, and the margins where the units must match.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qpagelayout.html#isEquivalentTo">QPageLayout::isEquivalentTo</a>().</p></div>
#[inline(always)]
pub unsafe fn eq(
    key: crate::q_key_sequence::StandardKey,
    e: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QKeyEvent>>,
) -> bool {
    crate::__ffi::ctr_qt_gui_ffi_operator__17(
        key,
        ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QKeyEvent>>::cast_into(e).as_raw_ptr()
            as *mut crate::QKeyEvent,
    )
}

/// Calls C++ function: <span style='color: green;'>```QVector2D operator*(float factor, const QVector2D& vector)```</span>.
#[inline(always)]
pub unsafe fn mul_float_q_vector_2d(
    factor: ::std::os::raw::c_float,
    vector: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVector2D>>,
) -> ::cpp_core::CppBox<crate::QVector2D> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_2(
            factor,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVector2D>>::cast_into(vector)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QPoint operator*(const QPoint& p, const QMatrix& m)```</span>.
#[inline(always)]
pub unsafe fn mul_q_point_q_matrix(
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QPoint>>,
    m: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMatrix>>,
) -> ::cpp_core::CppBox<::qt_core::QPoint> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_14(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QPoint>>::cast_into(p).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMatrix>>::cast_into(m).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QPointF operator*(const QPointF& p, const QMatrix& m)```</span>.
#[inline(always)]
pub unsafe fn mul_q_point_f_q_matrix(
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QPointF>>,
    m: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMatrix>>,
) -> ::cpp_core::CppBox<::qt_core::QPointF> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_15(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QPointF>>::cast_into(p).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMatrix>>::cast_into(m).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QLineF operator*(const QLineF& l, const QMatrix& m)```</span>.
#[inline(always)]
pub unsafe fn mul_q_line_f_q_matrix(
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QLineF>>,
    m: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMatrix>>,
) -> ::cpp_core::CppBox<::qt_core::QLineF> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_16(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QLineF>>::cast_into(l).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMatrix>>::cast_into(m).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QLine operator*(const QLine& l, const QMatrix& m)```</span>.
#[inline(always)]
pub unsafe fn mul_q_line_q_matrix(
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QLine>>,
    m: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMatrix>>,
) -> ::cpp_core::CppBox<::qt_core::QLine> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_17(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QLine>>::cast_into(l).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMatrix>>::cast_into(m).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QPoint operator*(const QPoint& p, const QTransform& m)```</span>.
#[inline(always)]
pub unsafe fn mul_q_point_q_transform(
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QPoint>>,
    m: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTransform>>,
) -> ::cpp_core::CppBox<::qt_core::QPoint> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_22(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QPoint>>::cast_into(p).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTransform>>::cast_into(m).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QPointF operator*(const QPointF& p, const QTransform& m)```</span>.
#[inline(always)]
pub unsafe fn mul_q_point_f_q_transform(
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QPointF>>,
    m: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTransform>>,
) -> ::cpp_core::CppBox<::qt_core::QPointF> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_23(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QPointF>>::cast_into(p).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTransform>>::cast_into(m).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QLineF operator*(const QLineF& l, const QTransform& m)```</span>.
#[inline(always)]
pub unsafe fn mul_q_line_f_q_transform(
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QLineF>>,
    m: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTransform>>,
) -> ::cpp_core::CppBox<::qt_core::QLineF> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_24(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QLineF>>::cast_into(l).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTransform>>::cast_into(m).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QLine operator*(const QLine& l, const QTransform& m)```</span>.
#[inline(always)]
pub unsafe fn mul_q_line_q_transform(
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QLine>>,
    m: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTransform>>,
) -> ::cpp_core::CppBox<::qt_core::QLine> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_25(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QLine>>::cast_into(l).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTransform>>::cast_into(m).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QVector3D operator*(float factor, const QVector3D& vector)```</span>.
#[inline(always)]
pub unsafe fn mul_float_q_vector_3d(
    factor: ::std::os::raw::c_float,
    vector: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVector3D>>,
) -> ::cpp_core::CppBox<crate::QVector3D> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_57(
            factor,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVector3D>>::cast_into(vector)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QVector4D operator*(float factor, const QVector4D& vector)```</span>.
#[inline(always)]
pub unsafe fn mul_float_q_vector_4d(
    factor: ::std::os::raw::c_float,
    vector: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVector4D>>,
) -> ::cpp_core::CppBox<crate::QVector4D> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_65(
            factor,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVector4D>>::cast_into(vector)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QQuaternion operator*(float factor, const QQuaternion& quaternion)```</span>.
#[inline(always)]
pub unsafe fn mul_float_q_quaternion(
    factor: ::std::os::raw::c_float,
    quaternion: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QQuaternion>>,
) -> ::cpp_core::CppBox<crate::QQuaternion> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_74(
            factor,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QQuaternion>>::cast_into(quaternion)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QPoint operator*(const QPoint& point, const QMatrix4x4& matrix)```</span>.
#[inline(always)]
pub unsafe fn mul_q_point_q_matrix4_x4(
    point: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QPoint>>,
    matrix: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMatrix4X4>>,
) -> ::cpp_core::CppBox<::qt_core::QPoint> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_87(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QPoint>>::cast_into(point)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMatrix4X4>>::cast_into(matrix)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QPointF operator*(const QPointF& point, const QMatrix4x4& matrix)```</span>.
#[inline(always)]
pub unsafe fn mul_q_point_f_q_matrix4_x4(
    point: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QPointF>>,
    matrix: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMatrix4X4>>,
) -> ::cpp_core::CppBox<::qt_core::QPointF> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_88(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QPointF>>::cast_into(point)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMatrix4X4>>::cast_into(matrix)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QMatrix4x4 operator*(float factor, const QMatrix4x4& matrix)```</span>.
#[inline(always)]
pub unsafe fn mul_float_q_matrix4_x4(
    factor: ::std::os::raw::c_float,
    matrix: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMatrix4X4>>,
) -> ::cpp_core::CppBox<crate::QMatrix4X4> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator_92(
            factor,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMatrix4X4>>::cast_into(matrix)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QColor& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_color(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QColor>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QColor>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QColor& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_color(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QColor>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__1(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QColor>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QRegion& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_region(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QRegion>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__3(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QRegion>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QRegion& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_region(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QRegion>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__5(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QRegion>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& in, const QKeySequence& ks)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_key_sequence(
    in_: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    ks: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QKeySequence>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__6(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(in_)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QKeySequence>>::cast_into(ks)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QKeySequence& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_key_sequence(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QKeySequence>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__8(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QKeySequence>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QVector2D& vector)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_2d(
    dbg: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vector: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVector2D>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__11(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(dbg).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVector2D>>::cast_into(vector)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QVector2D& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_2d(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVector2D>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__12(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVector2D>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QTouchDevice* arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_touch_device(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QTouchDevice>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__14(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QTouchDevice>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QEvent* arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_event(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ptr<::qt_core::QEvent>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__15(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<::qt_core::QEvent>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QTouchEvent::TouchPoint& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_touch_point(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::q_touch_event::TouchPoint>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__20(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::q_touch_event::TouchPoint>>::cast_into(
                arg2,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QFont& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_font(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QFont>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__21(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QFont>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QFont& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_font(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QFont>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__23(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QFont>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QPolygon& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_polygon(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPolygon>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__24(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPolygon>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& stream, const QPolygon& polygon)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_polygon(
    stream: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    polygon: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPolygon>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__25(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(stream)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPolygon>>::cast_into(polygon)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QPolygonF& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_polygon_f(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPolygonF>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__27(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPolygonF>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& stream, const QPolygonF& array)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_polygon_f(
    stream: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    array: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPolygonF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__28(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(stream)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPolygonF>>::cast_into(array)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QMatrix& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_matrix(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMatrix>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__30(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMatrix>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QMatrix& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_matrix(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMatrix>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__32(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMatrix>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QPainterPath& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_painter_path(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPainterPath>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__33(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPainterPath>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QPainterPath& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_painter_path(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPainterPath>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__35(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPainterPath>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QTransform& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_transform(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTransform>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__36(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTransform>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QTransform& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_transform(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTransform>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__38(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTransform>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QImage& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_image(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QImage>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__39(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QImage>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QImage& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_image(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QImage>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__41(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QImage>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QPixmap& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_pixmap(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPixmap>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__42(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPixmap>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QPixmap& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_pixmap(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPixmap>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__44(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPixmap>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QBrush& arg2)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_brush(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QBrush>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__45(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QBrush>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QBrush& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_brush(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QBrush>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__47(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QBrush>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QPen& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_pen(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPen>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__48(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPen>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QPen& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_pen(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPen>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__50(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPen>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QTextLength& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_text_length(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTextLength>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__51(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTextLength>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QTextLength& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_text_length(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTextLength>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__53(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTextLength>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QTextFormat& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_text_format(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTextFormat>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__54(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTextFormat>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QTextFormat& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_text_format(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTextFormat>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__56(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTextFormat>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& ds, const QPalette& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_palette(
    ds: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPalette>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__57(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(ds)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPalette>>::cast_into(p).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QPalette& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_palette(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPalette>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__59(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPalette>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug d, const QAccessibleInterface* iface)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_accessible_interface(
    d: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    iface: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QAccessibleInterface>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__61(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(d).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QAccessibleInterface>>::cast_into(iface)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug d, const QAccessibleEvent& ev)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_accessible_event(
    d: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    ev: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QAccessibleEvent>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__62(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(d).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QAccessibleEvent>>::cast_into(ev)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QSurfaceFormat& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_surface_format(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSurfaceFormat>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__65(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSurfaceFormat>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QIcon& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_icon(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QIcon>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__66(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QIcon>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QIcon& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_icon(
    dbg: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QIcon>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__68(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(dbg).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QIcon>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& outS, const QCursor& cursor)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_cursor(
    out_s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    cursor: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QCursor>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__69(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(out_s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QCursor>>::cast_into(cursor).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QCursor& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_cursor(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QCursor>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__71(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QCursor>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QWindow* arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_window(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QWindow>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__72(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QWindow>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QVector3D& vector)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_3d(
    dbg: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vector: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVector3D>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__75(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(dbg).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVector3D>>::cast_into(vector)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QVector3D& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_3d(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVector3D>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__76(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVector3D>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QVector4D& vector)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_4d(
    dbg: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vector: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVector4D>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__80(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(dbg).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVector4D>>::cast_into(vector)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QVector4D& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_4d(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVector4D>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__81(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVector4D>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QQuaternion& q)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_quaternion(
    dbg: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    q: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QQuaternion>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__85(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(dbg).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QQuaternion>>::cast_into(q).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QQuaternion& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_quaternion(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QQuaternion>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__86(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QQuaternion>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QMatrix4x4& m)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_matrix4_x4(
    dbg: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    m: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMatrix4X4>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__88(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(dbg).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMatrix4X4>>::cast_into(m).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QMatrix4x4& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_matrix4_x4(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMatrix4X4>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__89(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMatrix4X4>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QOpenGLDebugMessage& message)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_opengl_debug_message(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    message: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QOpenGLDebugMessage>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__93(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QOpenGLDebugMessage>>::cast_into(message)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, QOpenGLDebugMessage::Source source)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_source(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    source: crate::q_opengl_debug_message::Source,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__94(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            source,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, QOpenGLDebugMessage::Type type)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_type(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    type_: crate::q_opengl_debug_message::Type,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__95(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            type_,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, QOpenGLDebugMessage::Severity severity)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_severity(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    severity: crate::q_opengl_debug_message::Severity,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__96(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            severity,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QPageSize& pageSize)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_page_size(
    dbg: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    page_size: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPageSize>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__99(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(dbg).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPageSize>>::cast_into(page_size)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QPageLayout& pageLayout)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_page_layout(
    dbg: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    page_layout: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPageLayout>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__102(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(dbg).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPageLayout>>::cast_into(page_layout)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QPicture& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_picture(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPicture>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__103(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPicture>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QScreen* arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_screen(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QScreen>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__105(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QScreen>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& out, const QStandardItem& item)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_standard_item(
    out: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    item: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QStandardItem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__107(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(out)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QStandardItem>>::cast_into(item)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QPaintEngine::PaintEngineFeature> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_paint_engine_feature(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_paint_engine::PaintEngineFeature>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__200(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QPaintEngine::DirtyFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_dirty_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_paint_engine::DirtyFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__201(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QTouchDevice::CapabilityFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_capability_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_touch_device::CapabilityFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__202(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QTouchEvent::TouchPoint::InfoFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_info_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_touch_event::touch_point::InfoFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__203(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QTextOption::Flag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_text_option::Flag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__204(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QTextFormat::PageBreakFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_page_break_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_text_format::PageBreakFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__205(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QRawFont::LayoutFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_layout_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_raw_font::LayoutFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__206(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QGlyphRun::GlyphRunFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_glyph_run_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_glyph_run::GlyphRunFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__207(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QTextDocument::FindFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_find_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_text_document::FindFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__208(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QAccessible::RelationFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_relation_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_accessible::RelationFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__209(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QSurfaceFormat::FormatOption> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_format_option(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_surface_format::FormatOption>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__210(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QImageIOPlugin::Capability> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_capability(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_image_io_plugin::Capability>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__211(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QImageIOHandler::Transformation> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_transformation(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_image_io_handler::Transformation>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__212(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QOpenGLBuffer::RangeAccessFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_range_access_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_opengl_buffer::RangeAccessFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__213(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QOpenGLDebugMessage::Source> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_source(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_opengl_debug_message::Source>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__214(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QOpenGLDebugMessage::Type> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_type(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_opengl_debug_message::Type>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__215(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QOpenGLDebugMessage::Severity> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_severity(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_opengl_debug_message::Severity>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__216(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QOpenGLFunctions::OpenGLFeature> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_opengl_feature(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_opengl_functions::OpenGLFeature>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__217(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QOpenGLShader::ShaderTypeBit> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_shader_type_bit(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_opengl_shader::ShaderTypeBit>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__218(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QOpenGLTexture::Feature> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_feature(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_opengl_texture::Feature>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__219(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QPainter::PixmapFragmentHint> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_pixmap_fragment_hint(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_painter::PixmapFragmentHint>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__220(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QPainter::RenderHint> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_render_hint(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_painter::RenderHint>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__221(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QTextItem::RenderFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_render_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_text_item::RenderFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__222(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QSize>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_list_of_q_size(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQSize>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__268(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQSize>>::cast_into(l).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QKeySequence>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_list_of_q_key_sequence(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQKeySequence>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__269(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQKeySequence>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<const QTouchDevice*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_list_of_q_touch_device(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQTouchDevice>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__270(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQTouchDevice>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QPolygonF>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_list_of_q_polygon_f(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQPolygonF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__271(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQPolygonF>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<double>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_list_of_double(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfDouble>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__272(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfDouble>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QFontDatabase::WritingSystem>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_list_of_writing_system(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfWritingSystem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__274(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfWritingSystem>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QAccessibleInterface*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_list_of_q_accessible_interface(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQAccessibleInterface>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__277(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQAccessibleInterface>>::cast_into(
                l,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QWindow*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_list_of_q_window(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQWindow>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__278(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQWindow>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QScreen*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_list_of_q_screen(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQScreen>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__279(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQScreen>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QOpenGLContext*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_list_of_q_opengl_context(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQOpenglContext>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__280(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQOpenglContext>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QOpenGLShader*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_list_of_q_opengl_shader(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQOpenglShader>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__282(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQOpenglShader>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QStandardItem*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_list_of_q_standard_item(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQStandardItem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__283(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQStandardItem>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QTextFrame*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_list_of_q_text_frame(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQTextFrame>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__285(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQTextFrame>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<QPoint>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_of_q_point(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQPoint>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__302(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQPoint>>::cast_into(v)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<QPair<double, QColor>>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_of_q_pair_of_double_q_color(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQPairOfDoubleQColor>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__303(::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s).as_raw_ptr() as *mut ::qt_core::QDataStream, ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQPairOfDoubleQColor>>::cast_into(v).as_raw_ptr())
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<QRect>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_of_q_rect(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQRect>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__305(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQRect>>::cast_into(v)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<double>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_of_double(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfDouble>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__306(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfDouble>>::cast_into(v)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<QTextLength>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_of_q_text_length(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQTextLength>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__307(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQTextLength>>::cast_into(v)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<quint32>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_of_u32(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfU32>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__308(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfU32>>::cast_into(v).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<QTextFormat>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_of_q_text_format(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQTextFormat>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__310(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQTextFormat>>::cast_into(v)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<QPair<QAccessibleInterface*, QFlags<QAccessible::RelationFlag>>>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_of_q_pair_of_q_accessible_interface_q_flags_relation_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<
        ::cpp_core::Ref<crate::QVectorOfQPairOfQAccessibleInterfaceQFlagsRelationFlag>,
    >,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__311(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::QVectorOfQPairOfQAccessibleInterfaceQFlagsRelationFlag>,
            >::cast_into(v)
            .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<QSize>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_of_q_size(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQSize>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__312(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQSize>>::cast_into(v)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<float>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_of_float(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfFloat>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__313(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfFloat>>::cast_into(v)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<QLineF>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_of_q_line_f(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQLineF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__315(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQLineF>>::cast_into(v)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<QLine>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_of_q_line(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQLine>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__316(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQLine>>::cast_into(v)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<QRectF>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_vector_of_q_rect_f(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQRectF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__317(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQRectF>>::cast_into(v)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QSet<QByteArray>& set)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_set_of_q_byte_array(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    set: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSetOfQByteArray>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__319(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSetOfQByteArray>>::cast_into(set)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QPair<double, QColor>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_pair_of_double_q_color(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfDoubleQColor>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__325(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfDoubleQColor>>::cast_into(p)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QPair<int, int>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_pair_of_int_int(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfIntInt>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__326(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfIntInt>>::cast_into(p)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QPair<QAccessibleInterface*, QFlags<QAccessible::RelationFlag>>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_pair_of_q_accessible_interface_q_flags_relation_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfQAccessibleInterfaceQFlagsRelationFlag>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__327(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::QPairOfQAccessibleInterfaceQFlagsRelationFlag>,
            >::cast_into(p)
            .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QPair<QOpenGLTexture::Filter, QOpenGLTexture::Filter>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_pair_of_filter_filter(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfFilterFilter>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__328(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfFilterFilter>>::cast_into(p)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QPair<float, float>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_pair_of_float_float(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfFloatFloat>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__329(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfFloatFloat>>::cast_into(p)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QTouchEvent::TouchPoint>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_touch_point(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfTouchPoint>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__331(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfTouchPoint>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QSize>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_q_size(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQSize>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__332(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQSize>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QKeySequence>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_q_key_sequence(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQKeySequence>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__333(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQKeySequence>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<const QTouchDevice*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_q_touch_device(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQTouchDevice>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__334(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQTouchDevice>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QPolygonF>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_q_polygon_f(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQPolygonF>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__335(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQPolygonF>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<double>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_double(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfDouble>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__336(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfDouble>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QFontDatabase::WritingSystem>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_writing_system(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfWritingSystem>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__338(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfWritingSystem>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QAccessibleInterface*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_q_accessible_interface(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQAccessibleInterface>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__341(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQAccessibleInterface>>::cast_into(
                list,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QWindow*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_q_window(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQWindow>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__342(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQWindow>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QScreen*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_q_screen(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQScreen>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__343(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQScreen>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QOpenGLContext*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_q_opengl_context(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQOpenglContext>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__344(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQOpenglContext>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QOpenGLDebugMessage>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_q_opengl_debug_message(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQOpenglDebugMessage>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__345(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQOpenglDebugMessage>>::cast_into(
                list,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QOpenGLShader*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_q_opengl_shader(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQOpenglShader>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__346(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQOpenglShader>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QStandardItem*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_q_standard_item(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQStandardItem>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__347(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQStandardItem>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QTextFrame*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_q_text_frame(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQTextFrame>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__349(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQTextFrame>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<QPoint>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_q_point(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQPoint>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__350(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQPoint>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<QPair<double, QColor>>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_q_pair_of_double_q_color(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQPairOfDoubleQColor>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__351(::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQPairOfDoubleQColor>>::cast_into(vec).as_raw_ptr())
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<QRect>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_q_rect(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQRect>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__353(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQRect>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<double>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_double(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfDouble>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__354(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfDouble>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<QTextLength>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_q_text_length(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQTextLength>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__355(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQTextLength>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<quint32>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_u32(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfU32>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__356(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfU32>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<QTextFormat>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_q_text_format(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQTextFormat>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__358(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQTextFormat>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<QPair<QAccessibleInterface*, QFlags<QAccessible::RelationFlag>>>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_q_pair_of_q_accessible_interface_q_flags_relation_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<
        ::cpp_core::Ref<crate::QVectorOfQPairOfQAccessibleInterfaceQFlagsRelationFlag>,
    >,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__359(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::QVectorOfQPairOfQAccessibleInterfaceQFlagsRelationFlag>,
            >::cast_into(vec)
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<QSize>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_q_size(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQSize>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__360(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQSize>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<float>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_float(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfFloat>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__361(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfFloat>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<GLuint64>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_u64(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfU64>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__362(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfU64>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<QLineF>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_q_line_f(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQLineF>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__363(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQLineF>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<QLine>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_q_line(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQLine>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__364(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQLine>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<QRectF>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_vector_of_q_rect_f(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQRectF>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__365(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQRectF>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QPair<double, QColor>& pair)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_pair_of_double_q_color(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    pair: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfDoubleQColor>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__366(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfDoubleQColor>>::cast_into(pair)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QPair<int, int>& pair)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_pair_of_int_int(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    pair: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfIntInt>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__367(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfIntInt>>::cast_into(pair)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QPair<QAccessibleInterface*, QFlags<QAccessible::RelationFlag>>& pair)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_pair_of_q_accessible_interface_q_flags_relation_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    pair: impl ::cpp_core::CastInto<
        ::cpp_core::Ref<crate::QPairOfQAccessibleInterfaceQFlagsRelationFlag>,
    >,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__368(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::QPairOfQAccessibleInterfaceQFlagsRelationFlag>,
            >::cast_into(pair)
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QPair<QOpenGLTexture::Filter, QOpenGLTexture::Filter>& pair)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_pair_of_filter_filter(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    pair: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfFilterFilter>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__369(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfFilterFilter>>::cast_into(pair)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QPair<float, float>& pair)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_pair_of_float_float(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    pair: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfFloatFloat>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__370(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfFloatFloat>>::cast_into(pair)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QSet<QByteArray>& set)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_set_of_q_byte_array(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    set: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSetOfQByteArray>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__371(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSetOfQByteArray>>::cast_into(set)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QPaintEngine::PaintEngineFeature>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_paint_engine_feature(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_paint_engine::PaintEngineFeature>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__372(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QPaintEngine::DirtyFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_dirty_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_paint_engine::DirtyFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__373(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QTouchDevice::CapabilityFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_capability_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_touch_device::CapabilityFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__374(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QTouchEvent::TouchPoint::InfoFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_info_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_touch_event::touch_point::InfoFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__375(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QTextOption::Flag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_text_option::Flag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__376(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QTextFormat::PageBreakFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_page_break_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_text_format::PageBreakFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__377(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QRawFont::LayoutFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_layout_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_raw_font::LayoutFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__378(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QGlyphRun::GlyphRunFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_glyph_run_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_glyph_run::GlyphRunFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__379(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QTextDocument::FindFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_find_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_text_document::FindFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__380(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QAccessible::RelationFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_relation_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_accessible::RelationFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__381(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QSurfaceFormat::FormatOption>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_format_option(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_surface_format::FormatOption>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__382(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QImageIOPlugin::Capability>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_capability(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_image_io_plugin::Capability>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__383(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QImageIOHandler::Transformation>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_transformation(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_image_io_handler::Transformation>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__384(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QOpenGLBuffer::RangeAccessFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_range_access_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_opengl_buffer::RangeAccessFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__385(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QOpenGLDebugMessage::Source>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_source(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_opengl_debug_message::Source>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__386(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QOpenGLDebugMessage::Type>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_type(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_opengl_debug_message::Type>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__387(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QOpenGLDebugMessage::Severity>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_severity(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_opengl_debug_message::Severity>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__388(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QOpenGLFunctions::OpenGLFeature>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_opengl_feature(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_opengl_functions::OpenGLFeature>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__389(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QOpenGLShader::ShaderTypeBit>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_shader_type_bit(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_opengl_shader::ShaderTypeBit>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__390(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QOpenGLTexture::Feature>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_feature(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_opengl_texture::Feature>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__391(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QPainter::PixmapFragmentHint>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_pixmap_fragment_hint(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_painter::PixmapFragmentHint>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__392(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QPainter::RenderHint>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_render_hint(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_painter::RenderHint>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__393(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QTextItem::RenderFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_render_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_text_item::RenderFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__394(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QOpenGLVersionProfile& vp)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.11.3",
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.11.3",
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn shl_q_debug_q_opengl_version_profile(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vp: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QOpenGLVersionProfile>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__792(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QOpenGLVersionProfile>>::cast_into(vp)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QOpenGLContext* ctx)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_opengl_context(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    ctx: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QOpenGLContext>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__793(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QOpenGLContext>>::cast_into(ctx)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QOpenGLContextGroup* cg)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_opengl_context_group(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    cg: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QOpenGLContextGroup>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__794(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QOpenGLContextGroup>>::cast_into(cg)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QOpenGLTexture* t)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>brush</i> to the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.11.3",
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.11.3",
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn shl_q_debug_q_opengl_texture(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    t: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QOpenGLTexture>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__795(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QOpenGLTexture>>::cast_into(t)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the given <i>colorSpace</i> to the given <i>stream</i> as an ICC profile.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QColorSpace& arg2)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qcolorspace.html#operator-lt-lt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>colorSpace</i> to the given <i>stream</i> as an ICC profile.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qcolorspace.html#iccProfile">QColorSpace::iccProfile</a>() and <a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(cpp_lib_version = "5.14.0"))
)]
#[cfg(any(cpp_lib_version = "5.14.0", feature = "ritual_rustdoc"))]
pub unsafe fn shl_q_data_stream_q_color_space(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QColorSpace>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__804(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QColorSpace>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the given <i>colorSpace</i> to the given <i>stream</i> as an ICC profile.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QColorSpace& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qcolorspace.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &stream, const QColorSpace &colorSpace)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the given <i>colorSpace</i> to the given <i>stream</i> as an ICC profile.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qcolorspace.html#iccProfile">QColorSpace::iccProfile</a>() and <a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(cpp_lib_version = "5.14.0"))
)]
#[cfg(any(cpp_lib_version = "5.14.0", feature = "ritual_rustdoc"))]
pub unsafe fn shl_q_debug_q_color_space(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QColorSpace>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__806(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QColorSpace>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QTextDocument::MarkdownFeature> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(cpp_lib_version = "5.14.0"))
)]
#[cfg(any(cpp_lib_version = "5.14.0", feature = "ritual_rustdoc"))]
pub unsafe fn shl_q_data_stream_q_flags_markdown_feature(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_text_document::MarkdownFeature>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__807(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QTextDocument::MarkdownFeature>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(cpp_lib_version = "5.14.0"))
)]
#[cfg(any(cpp_lib_version = "5.14.0", feature = "ritual_rustdoc"))]
pub unsafe fn shl_q_debug_q_flags_markdown_feature(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_text_document::MarkdownFeature>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__809(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QColor& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_color(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QColor>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__2(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QColor>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QColor,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QRegion& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_region(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QRegion>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__4(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QRegion>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QRegion,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& out, QKeySequence& ks)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_key_sequence(
    out: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    ks: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QKeySequence>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__7(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(out)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QKeySequence>>::cast_into(ks).as_raw_ptr()
                as *mut crate::QKeySequence,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QVector2D& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_2d(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVector2D>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__13(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVector2D>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QVector2D,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QFont& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_font(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QFont>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__22(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QFont>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QFont,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& stream, QPolygon& polygon)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_polygon(
    stream: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    polygon: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPolygon>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__26(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(stream)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPolygon>>::cast_into(polygon)
                .as_raw_ptr() as *mut crate::QPolygon,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& stream, QPolygonF& array)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_polygon_f(
    stream: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    array: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPolygonF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__29(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(stream)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPolygonF>>::cast_into(array).as_raw_ptr()
                as *mut crate::QPolygonF,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QMatrix& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_matrix(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMatrix>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__31(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMatrix>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QMatrix,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QPainterPath& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_painter_path(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPainterPath>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__34(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPainterPath>>::cast_into(arg2)
                .as_raw_ptr() as *mut crate::QPainterPath,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QTransform& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_transform(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTransform>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__37(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTransform>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QTransform,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QImage& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_image(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QImage>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__40(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QImage>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QImage,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QPixmap& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_pixmap(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPixmap>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__43(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPixmap>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QPixmap,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QBrush& arg2)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_brush(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QBrush>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__46(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QBrush>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QBrush,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QPen& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_pen(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPen>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__49(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPen>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QPen,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QTextLength& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_text_length(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTextLength>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__52(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTextLength>>::cast_into(arg2)
                .as_raw_ptr() as *mut crate::QTextLength,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QTextFormat& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_text_format(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTextFormat>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__55(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTextFormat>>::cast_into(arg2)
                .as_raw_ptr() as *mut crate::QTextFormat,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& ds, QPalette& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_palette(
    ds: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPalette>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__58(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(ds)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPalette>>::cast_into(p).as_raw_ptr()
                as *mut crate::QPalette,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QIcon& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_icon(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QIcon>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__67(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QIcon>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QIcon,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& inS, QCursor& cursor)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_cursor(
    in_s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    cursor: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QCursor>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__70(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(in_s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QCursor>>::cast_into(cursor).as_raw_ptr()
                as *mut crate::QCursor,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QVector3D& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_3d(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVector3D>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__77(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVector3D>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QVector3D,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QVector4D& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_4d(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVector4D>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__82(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVector4D>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QVector4D,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QQuaternion& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_quaternion(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QQuaternion>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__87(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QQuaternion>>::cast_into(arg2)
                .as_raw_ptr() as *mut crate::QQuaternion,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QMatrix4x4& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_matrix4_x4(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMatrix4X4>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__90(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMatrix4X4>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QMatrix4X4,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QPicture& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_picture(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPicture>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__104(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPicture>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QPicture,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& in, QStandardItem& item)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_standard_item(
    in_: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    item: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QStandardItem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__106(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(in_)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QStandardItem>>::cast_into(item)
                .as_raw_ptr() as *mut crate::QStandardItem,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QList<QSize>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_list_of_q_size(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQSize>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__248(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQSize>>::cast_into(l).as_raw_ptr()
                as *mut crate::QListOfQSize,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QList<QKeySequence>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_list_of_q_key_sequence(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQKeySequence>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__249(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQKeySequence>>::cast_into(l)
                .as_raw_ptr() as *mut crate::QListOfQKeySequence,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QList<QPolygonF>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_list_of_q_polygon_f(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQPolygonF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__251(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQPolygonF>>::cast_into(l)
                .as_raw_ptr() as *mut crate::QListOfQPolygonF,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QList<double>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_list_of_double(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfDouble>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__252(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfDouble>>::cast_into(l).as_raw_ptr()
                as *mut crate::QListOfDouble,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QList<QFontDatabase::WritingSystem>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(cpp_lib_version = "5.14.0"))
)]
#[cfg(any(cpp_lib_version = "5.14.0", feature = "ritual_rustdoc"))]
pub unsafe fn shr_q_data_stream_q_list_of_writing_system(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfWritingSystem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__254(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfWritingSystem>>::cast_into(l)
                .as_raw_ptr() as *mut crate::QListOfWritingSystem,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVector<QPoint>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_of_q_point(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQPoint>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__286(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQPoint>>::cast_into(v)
                .as_raw_ptr() as *mut crate::QVectorOfQPoint,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVector<QPair<double, QColor>>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_of_q_pair_of_double_q_color(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQPairOfDoubleQColor>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__287(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQPairOfDoubleQColor>>::cast_into(
                v,
            )
            .as_raw_ptr() as *mut crate::QVectorOfQPairOfDoubleQColor,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVector<QRect>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_of_q_rect(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQRect>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__289(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQRect>>::cast_into(v)
                .as_raw_ptr() as *mut crate::QVectorOfQRect,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVector<double>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_of_double(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfDouble>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__290(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfDouble>>::cast_into(v)
                .as_raw_ptr() as *mut crate::QVectorOfDouble,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVector<QTextLength>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_of_q_text_length(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQTextLength>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__291(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQTextLength>>::cast_into(v)
                .as_raw_ptr() as *mut crate::QVectorOfQTextLength,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVector<quint32>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_of_u32(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfU32>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__292(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfU32>>::cast_into(v).as_raw_ptr()
                as *mut crate::QVectorOfU32,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVector<QTextFormat>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_of_q_text_format(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQTextFormat>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__294(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQTextFormat>>::cast_into(v)
                .as_raw_ptr() as *mut crate::QVectorOfQTextFormat,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVector<QSize>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_of_q_size(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQSize>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__296(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQSize>>::cast_into(v)
                .as_raw_ptr() as *mut crate::QVectorOfQSize,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVector<float>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_of_float(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfFloat>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__297(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfFloat>>::cast_into(v)
                .as_raw_ptr() as *mut crate::QVectorOfFloat,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVector<QLineF>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_of_q_line_f(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQLineF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__299(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQLineF>>::cast_into(v)
                .as_raw_ptr() as *mut crate::QVectorOfQLineF,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVector<QLine>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_of_q_line(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQLine>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__300(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQLine>>::cast_into(v)
                .as_raw_ptr() as *mut crate::QVectorOfQLine,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVector<QRectF>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_of_q_rect_f(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQRectF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__301(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQRectF>>::cast_into(v)
                .as_raw_ptr() as *mut crate::QVectorOfQRectF,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QSet<QByteArray>& set)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_set_of_q_byte_array(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    set: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSetOfQByteArray>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__318(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSetOfQByteArray>>::cast_into(set)
                .as_raw_ptr() as *mut crate::QSetOfQByteArray,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QPair<double, QColor>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_pair_of_double_q_color(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfDoubleQColor>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__320(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfDoubleQColor>>::cast_into(p)
                .as_raw_ptr() as *mut crate::QPairOfDoubleQColor,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QPair<int, int>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_pair_of_int_int(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfIntInt>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__321(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfIntInt>>::cast_into(p).as_raw_ptr()
                as *mut crate::QPairOfIntInt,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QPair<QOpenGLTexture::Filter, QOpenGLTexture::Filter>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(cpp_lib_version = "5.14.0"))
)]
#[cfg(any(cpp_lib_version = "5.14.0", feature = "ritual_rustdoc"))]
pub unsafe fn shr_q_data_stream_q_pair_of_filter_filter(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfFilterFilter>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__323(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfFilterFilter>>::cast_into(p)
                .as_raw_ptr() as *mut crate::QPairOfFilterFilter,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QPair<float, float>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_pair_of_float_float(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfFloatFloat>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__324(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfFloatFloat>>::cast_into(p)
                .as_raw_ptr() as *mut crate::QPairOfFloatFloat,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QColorSpace& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbrush.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &stream, QBrush &brush)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads the given <i>brush</i> from the given <i>stream</i> and returns a reference to the <i>stream</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(cpp_lib_version = "5.14.0"))
)]
#[cfg(any(cpp_lib_version = "5.14.0", feature = "ritual_rustdoc"))]
pub unsafe fn shr_q_data_stream_q_color_space(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QColorSpace>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_operator__805(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QColorSpace>>::cast_into(arg2)
                .as_raw_ptr() as *mut crate::QColorSpace,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}
