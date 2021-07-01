#![cfg_attr(feature = "ritual_rustdoc_nightly", feature(doc_cfg))]
//! Functions that provide access to C++ operators

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QSizePolicy& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_size_policy(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSizePolicy>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSizePolicy>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QSizePolicy& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_size_policy(
    dbg: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSizePolicy>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__2(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(dbg).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSizePolicy>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QWidget* arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_widget(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QWidget>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__3(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QWidget>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QStyleOption::OptionType& optionType)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_style_option_option_type(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    option_type: *const crate::q_style_option::OptionType,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__5(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            option_type,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QStyleOption& option)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_style_option(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    option: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QStyleOption>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__6(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QStyleOption>>::cast_into(option)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QAction* arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_action(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QAction>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__7(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QAction>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QGesture* arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_gesture(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QGesture>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__8(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QGesture>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QGestureEvent* arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_gesture_event(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QGestureEvent>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__9(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QGestureEvent>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, QGraphicsItem* item)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_graphics_item(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    item: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QGraphicsItem>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__10(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QGraphicsItem>>::cast_into(item)
                .as_raw_ptr() as *mut crate::QGraphicsItem,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, QGraphicsObject* item)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_graphics_object(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    item: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QGraphicsObject>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__11(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QGraphicsObject>>::cast_into(item)
                .as_raw_ptr() as *mut crate::QGraphicsObject,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, QGraphicsItem::GraphicsItemChange change)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_graphics_item_graphics_item_change(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    change: crate::q_graphics_item::GraphicsItemChange,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__12(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            change,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, QGraphicsItem::GraphicsItemFlag flag)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_graphics_item_graphics_item_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flag: crate::q_graphics_item::GraphicsItemFlag,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__13(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flag,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& out, const QListWidgetItem& item)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_widget_item(
    out: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    item: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListWidgetItem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__15(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(out)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListWidgetItem>>::cast_into(item)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QTextStream& operator<<(QTextStream& arg1, const QSplitter& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_text_stream_q_splitter(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QTextStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSplitter>>,
) -> ::cpp_core::Ref<::qt_core::QTextStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__17(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QTextStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QTextStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSplitter>>::cast_into(arg2).as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& out, const QTableWidgetItem& item)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_table_widget_item(
    out: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    item: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTableWidgetItem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__20(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(out)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTableWidgetItem>>::cast_into(item)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& out, const QTreeWidgetItem& item)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_tree_widget_item(
    out: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    item: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTreeWidgetItem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__21(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(out)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTreeWidgetItem>>::cast_into(item)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QStyle::StateFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_style_state_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_style::StateFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__77(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QStyleOptionFrame::FrameFeature> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_style_option_frame_frame_feature(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_style_option_frame::FrameFeature>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__78(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QStyleOptionButton::ButtonFeature> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_style_option_button_button_feature(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_style_option_button::ButtonFeature>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__79(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QStyleOptionTab::CornerWidget> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_style_option_tab_corner_widget(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_style_option_tab::CornerWidget>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__80(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QStyleOptionTab::TabFeature> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_style_option_tab_tab_feature(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_style_option_tab::TabFeature>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__81(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QStyleOptionToolBar::ToolBarFeature> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_style_option_tool_bar_tool_bar_feature(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_style_option_tool_bar::ToolBarFeature>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__82(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QStyleOptionViewItem::ViewItemFeature> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_style_option_view_item_view_item_feature(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_style_option_view_item::ViewItemFeature>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__83(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QStyle::SubControl> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_style_sub_control(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_style::SubControl>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__84(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QAbstractSpinBox::StepEnabledFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_abstract_spin_box_step_enabled_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_abstract_spin_box::StepEnabledFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__85(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QStyleOptionToolButton::ToolButtonFeature> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_style_option_tool_button_tool_button_feature(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_style_option_tool_button::ToolButtonFeature>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__86(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QTreeWidgetItemIterator::IteratorFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_tree_widget_item_iterator_iterator_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_tree_widget_item_iterator::IteratorFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__87(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QSizePolicy::ControlType> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_size_policy_control_type(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_size_policy::ControlType>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__88(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QWidget::RenderFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_widget_render_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_widget::RenderFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__89(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QAbstractItemView::EditTrigger> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_abstract_item_view_edit_trigger(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_abstract_item_view::EditTrigger>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__90(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QColorDialog::ColorDialogOption> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_color_dialog_color_dialog_option(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_color_dialog::ColorDialogOption>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__91(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QDateTimeEdit::Section> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_date_time_edit_section(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_date_time_edit::Section>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__92(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QDialogButtonBox::StandardButton> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_dialog_button_box_standard_button(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_dialog_button_box::StandardButton>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__93(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QFileIconProvider::Option> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_file_icon_provider_option(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_file_icon_provider::Option>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__94(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QDockWidget::DockWidgetFeature> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_dock_widget_dock_widget_feature(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_dock_widget::DockWidgetFeature>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__95(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QDrawBorderPixmap::DrawingHint> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_draw_border_pixmap_drawing_hint(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_draw_border_pixmap::DrawingHint>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__96(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QFileDialog::Option> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_file_dialog_option(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_file_dialog::Option>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__97(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QFontComboBox::FontFilter> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_font_combo_box_font_filter(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_font_combo_box::FontFilter>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__98(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QFontDialog::FontDialogOption> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_font_dialog_font_dialog_option(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_font_dialog::FontDialogOption>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__99(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QPinchGesture::ChangeFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_pinch_gesture_change_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_pinch_gesture::ChangeFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__100(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QGestureRecognizer::ResultFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_gesture_recognizer_result_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_gesture_recognizer::ResultFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__101(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QGraphicsItem::GraphicsItemFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_graphics_item_graphics_item_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_graphics_item::GraphicsItemFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__102(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QGraphicsEffect::ChangeFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_graphics_effect_change_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_graphics_effect::ChangeFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__103(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QGraphicsBlurEffect::BlurHint> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_graphics_blur_effect_blur_hint(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_graphics_blur_effect::BlurHint>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__104(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QGraphicsScene::SceneLayer> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_graphics_scene_scene_layer(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_graphics_scene::SceneLayer>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__105(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QGraphicsView::OptimizationFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_graphics_view_optimization_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_graphics_view::OptimizationFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__106(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QGraphicsView::CacheModeFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_graphics_view_cache_mode_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_graphics_view::CacheModeFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__107(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QInputDialog::InputDialogOption> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_input_dialog_input_dialog_option(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_input_dialog::InputDialogOption>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__108(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QMainWindow::DockOption> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_main_window_dock_option(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_main_window::DockOption>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__109(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QMdiArea::AreaOption> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_mdi_area_area_option(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_mdi_area::AreaOption>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__110(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QMdiSubWindow::SubWindowOption> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_mdi_sub_window_sub_window_option(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_mdi_sub_window::SubWindowOption>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__111(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QMessageBox::StandardButton> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_message_box_standard_button(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_message_box::StandardButton>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__112(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QTextEdit::AutoFormattingFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_text_edit_auto_formatting_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_text_edit::AutoFormattingFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__113(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QWizard::WizardOption> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_wizard_wizard_option(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_wizard::WizardOption>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__114(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QGesture*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_gesture(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQGesture>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__174(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQGesture>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QTreeWidgetItem*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_tree_widget_item(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQTreeWidgetItem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__175(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQTreeWidgetItem>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QAction*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_action(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQAction>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__176(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQAction>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QWidget*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_widget(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQWidget>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__177(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQWidget>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QGraphicsWidget*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_graphics_widget(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQGraphicsWidget>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__178(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQGraphicsWidget>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QAbstractButton*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_abstract_button(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQAbstractButton>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__179(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQAbstractButton>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QGraphicsItem*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_graphics_item(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQGraphicsItem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__180(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQGraphicsItem>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QGraphicsTransform*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_graphics_transform(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQGraphicsTransform>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__181(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQGraphicsTransform>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QPair<double, QPointF>>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_pair_of_double_q_point_f(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQPairOfDoubleQPointF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__182(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQPairOfDoubleQPointF>>::cast_into(
                l,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QPair<double, double>>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_pair_of_double_double(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQPairOfDoubleDouble>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__183(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQPairOfDoubleDouble>>::cast_into(
                l,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QGraphicsView*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_graphics_view(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQGraphicsView>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__184(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQGraphicsView>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QRectF>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_rect_f(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQRectF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__185(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQRectF>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QListWidgetItem*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_list_widget_item(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQListWidgetItem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__186(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQListWidgetItem>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QDockWidget*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_dock_widget(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQDockWidget>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__187(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQDockWidget>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QMdiSubWindow*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_mdi_sub_window(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQMdiSubWindow>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__188(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQMdiSubWindow>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QScroller*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_scroller(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQScroller>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__190(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQScroller>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QTableWidgetItem*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_table_widget_item(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQTableWidgetItem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__192(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQTableWidgetItem>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QUndoStack*>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_undo_stack(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQUndoStack>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__193(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQUndoStack>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QWizard::WizardButton>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_wizard_button(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfWizardButton>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__194(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfWizardButton>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<void*>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_vector_of_void(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfVoid>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__197(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfVoid>>::cast_into(v)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVector<QColor>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_vector_of_q_color(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQColor>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__198(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQColor>>::cast_into(v)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QHash<int, QItemEditorCreatorBase*>& hash)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_hash_of_int_q_item_editor_creator_base(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    hash: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QHashOfIntQItemEditorCreatorBase>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__200(::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s).as_raw_ptr() as *mut ::qt_core::QDataStream, ::cpp_core::CastInto::<::cpp_core::Ref<crate::QHashOfIntQItemEditorCreatorBase>>::cast_into(hash).as_raw_ptr())
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QMap<Qt::GestureType, bool>& map)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_map_of_gesture_type_bool(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    map: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMapOfGestureTypeBool>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__204(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMapOfGestureTypeBool>>::cast_into(map)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QMap<Qt::GestureType, QWidget*>& map)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_map_of_gesture_type_q_widget(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    map: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMapOfGestureTypeQWidget>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__205(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMapOfGestureTypeQWidget>>::cast_into(
                map,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QMap<QDate, QTextCharFormat>& map)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_map_of_q_date_q_text_char_format(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    map: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMapOfQDateQTextCharFormat>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__206(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMapOfQDateQTextCharFormat>>::cast_into(
                map,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QPair<double, QPointF>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_pair_of_double_q_point_f(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfDoubleQPointF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__209(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfDoubleQPointF>>::cast_into(p)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QPair<double, double>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_pair_of_double_double(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfDoubleDouble>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__210(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfDoubleDouble>>::cast_into(p)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QGesture*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_gesture(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQGesture>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__211(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQGesture>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QTreeWidgetItem*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_tree_widget_item(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQTreeWidgetItem>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__212(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQTreeWidgetItem>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QAction*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_action(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQAction>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__213(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQAction>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QWidget*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_widget(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQWidget>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__214(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQWidget>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QGraphicsWidget*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_graphics_widget(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQGraphicsWidget>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__215(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQGraphicsWidget>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QAbstractButton*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_abstract_button(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQAbstractButton>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__216(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQAbstractButton>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QGraphicsItem*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_graphics_item(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQGraphicsItem>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__217(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQGraphicsItem>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QGraphicsTransform*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_graphics_transform(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQGraphicsTransform>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__218(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQGraphicsTransform>>::cast_into(
                list,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QPair<double, QPointF>>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_pair_of_double_q_point_f(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQPairOfDoubleQPointF>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__219(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQPairOfDoubleQPointF>>::cast_into(
                list,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QPair<double, double>>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_pair_of_double_double(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQPairOfDoubleDouble>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__220(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQPairOfDoubleDouble>>::cast_into(
                list,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QGraphicsView*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_graphics_view(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQGraphicsView>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__221(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQGraphicsView>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QRectF>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_rect_f(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQRectF>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__222(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQRectF>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QListWidgetItem*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_list_widget_item(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQListWidgetItem>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__223(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQListWidgetItem>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QDockWidget*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_dock_widget(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQDockWidget>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__224(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQDockWidget>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QMdiSubWindow*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_mdi_sub_window(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQMdiSubWindow>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__225(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQMdiSubWindow>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QScroller*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_scroller(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQScroller>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__227(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQScroller>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QTableWidgetItem*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_table_widget_item(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQTableWidgetItem>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__229(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQTableWidgetItem>>::cast_into(
                list,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QUndoStack*>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_undo_stack(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQUndoStack>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__230(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQUndoStack>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QWizard::WizardButton>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_wizard_button(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfWizardButton>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__231(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfWizardButton>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<void*>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_vector_of_void(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfVoid>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__232(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfVoid>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<QColor>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_vector_of_q_color(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQColor>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__233(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQColor>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QMap<Qt::GestureType, bool>& map)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_map_of_gesture_type_bool(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    map: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMapOfGestureTypeBool>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__234(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMapOfGestureTypeBool>>::cast_into(map)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QMap<Qt::GestureType, QWidget*>& map)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_map_of_gesture_type_q_widget(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    map: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMapOfGestureTypeQWidget>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__235(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMapOfGestureTypeQWidget>>::cast_into(
                map,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QMap<QDate, QTextCharFormat>& map)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_map_of_q_date_q_text_char_format(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    map: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMapOfQDateQTextCharFormat>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__236(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMapOfQDateQTextCharFormat>>::cast_into(
                map,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QHash<int, QItemEditorCreatorBase*>& hash)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_hash_of_int_q_item_editor_creator_base(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    hash: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QHashOfIntQItemEditorCreatorBase>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__237(::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::QHashOfIntQItemEditorCreatorBase>>::cast_into(hash).as_raw_ptr())
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QPair<double, QPointF>& pair)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_pair_of_double_q_point_f(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    pair: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfDoubleQPointF>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__238(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfDoubleQPointF>>::cast_into(pair)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QPair<double, double>& pair)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_pair_of_double_double(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    pair: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfDoubleDouble>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__239(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfDoubleDouble>>::cast_into(pair)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QStyle::StateFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_style_state_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_style::StateFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__240(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QStyleOptionFrame::FrameFeature>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_style_option_frame_frame_feature(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_style_option_frame::FrameFeature>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__241(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QStyleOptionButton::ButtonFeature>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_style_option_button_button_feature(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_style_option_button::ButtonFeature>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__242(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QStyleOptionTab::CornerWidget>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_style_option_tab_corner_widget(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_style_option_tab::CornerWidget>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__243(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QStyleOptionTab::TabFeature>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_style_option_tab_tab_feature(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_style_option_tab::TabFeature>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__244(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QStyleOptionToolBar::ToolBarFeature>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_style_option_tool_bar_tool_bar_feature(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_style_option_tool_bar::ToolBarFeature>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__245(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QStyleOptionViewItem::ViewItemFeature>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_style_option_view_item_view_item_feature(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_style_option_view_item::ViewItemFeature>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__246(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QStyle::SubControl>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_style_sub_control(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_style::SubControl>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__247(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QAbstractSpinBox::StepEnabledFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_abstract_spin_box_step_enabled_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_abstract_spin_box::StepEnabledFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__248(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QStyleOptionToolButton::ToolButtonFeature>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_style_option_tool_button_tool_button_feature(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_style_option_tool_button::ToolButtonFeature>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__249(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QTreeWidgetItemIterator::IteratorFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_tree_widget_item_iterator_iterator_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_tree_widget_item_iterator::IteratorFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__250(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QSizePolicy::ControlType>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_size_policy_control_type(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_size_policy::ControlType>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__251(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QWidget::RenderFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_widget_render_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_widget::RenderFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__252(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QAbstractItemView::EditTrigger>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_abstract_item_view_edit_trigger(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_abstract_item_view::EditTrigger>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__253(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QColorDialog::ColorDialogOption>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_color_dialog_color_dialog_option(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_color_dialog::ColorDialogOption>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__254(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QDateTimeEdit::Section>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_date_time_edit_section(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_date_time_edit::Section>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__255(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QDialogButtonBox::StandardButton>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_dialog_button_box_standard_button(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_dialog_button_box::StandardButton>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__256(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QFileIconProvider::Option>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_file_icon_provider_option(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_file_icon_provider::Option>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__257(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QDockWidget::DockWidgetFeature>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_dock_widget_dock_widget_feature(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_dock_widget::DockWidgetFeature>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__258(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QDrawBorderPixmap::DrawingHint>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_draw_border_pixmap_drawing_hint(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_draw_border_pixmap::DrawingHint>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__259(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QFileDialog::Option>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_file_dialog_option(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_file_dialog::Option>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__260(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QFontComboBox::FontFilter>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_font_combo_box_font_filter(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_font_combo_box::FontFilter>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__261(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QFontDialog::FontDialogOption>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_font_dialog_font_dialog_option(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_font_dialog::FontDialogOption>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__262(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QPinchGesture::ChangeFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_pinch_gesture_change_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_pinch_gesture::ChangeFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__263(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QGestureRecognizer::ResultFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_gesture_recognizer_result_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_gesture_recognizer::ResultFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__264(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QGraphicsItem::GraphicsItemFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_graphics_item_graphics_item_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_graphics_item::GraphicsItemFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__265(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QGraphicsEffect::ChangeFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_graphics_effect_change_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_graphics_effect::ChangeFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__266(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QGraphicsBlurEffect::BlurHint>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_graphics_blur_effect_blur_hint(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_graphics_blur_effect::BlurHint>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__267(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QGraphicsScene::SceneLayer>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_graphics_scene_scene_layer(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_graphics_scene::SceneLayer>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__268(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QGraphicsView::OptimizationFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_graphics_view_optimization_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_graphics_view::OptimizationFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__269(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QGraphicsView::CacheModeFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_graphics_view_cache_mode_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_graphics_view::CacheModeFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__270(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QInputDialog::InputDialogOption>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_input_dialog_input_dialog_option(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_input_dialog::InputDialogOption>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__271(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QMainWindow::DockOption>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_main_window_dock_option(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_main_window::DockOption>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__272(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QMdiArea::AreaOption>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_mdi_area_area_option(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_mdi_area::AreaOption>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__273(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QMdiSubWindow::SubWindowOption>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_mdi_sub_window_sub_window_option(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_mdi_sub_window::SubWindowOption>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__274(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QMessageBox::StandardButton>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_message_box_standard_button(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_message_box::StandardButton>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__275(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QTextEdit::AutoFormattingFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_text_edit_auto_formatting_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_text_edit::AutoFormattingFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__276(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QWizard::WizardOption>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_wizard_wizard_option(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_wizard::WizardOption>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__277(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QGraphicsSceneEvent* arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes the list widget item <i>item</i> to stream <i>out</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#write">QListWidgetItem::write</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_graphics_scene_event(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ptr<crate::QGraphicsSceneEvent>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__556(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ptr<crate::QGraphicsSceneEvent>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QFileSystemModel::Option> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(cpp_lib_version = "5.14.0"))
)]
#[cfg(any(cpp_lib_version = "5.14.0", feature = "ritual_rustdoc"))]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_file_system_model_option(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_file_system_model::Option>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__558(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QFileSystemModel::Option>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(cpp_lib_version = "5.14.0"))
)]
#[cfg(any(cpp_lib_version = "5.14.0", feature = "ritual_rustdoc"))]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_file_system_model_option(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_file_system_model::Option>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__560(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Reads a list widget item from stream <i>in</i> into <i>item</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QSizePolicy& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a list widget item from stream <i>in</i> into <i>item</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#read">QListWidgetItem::read</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_size_policy(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSizePolicy>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__1(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSizePolicy>>::cast_into(arg2)
                .as_raw_ptr() as *mut crate::QSizePolicy,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a list widget item from stream <i>in</i> into <i>item</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& in, QListWidgetItem& item)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-gt-gt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a list widget item from stream <i>in</i> into <i>item</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#read">QListWidgetItem::read</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_list_widget_item(
    in_: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    item: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListWidgetItem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__16(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(in_)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListWidgetItem>>::cast_into(item)
                .as_raw_ptr() as *mut crate::QListWidgetItem,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a list widget item from stream <i>in</i> into <i>item</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QTextStream& operator>>(QTextStream& arg1, QSplitter& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a list widget item from stream <i>in</i> into <i>item</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#read">QListWidgetItem::read</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_text_stream_q_splitter(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QTextStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSplitter>>,
) -> ::cpp_core::Ref<::qt_core::QTextStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__18(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QTextStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QTextStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSplitter>>::cast_into(arg2).as_raw_ptr()
                as *mut crate::QSplitter,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a list widget item from stream <i>in</i> into <i>item</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& in, QTableWidgetItem& item)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a list widget item from stream <i>in</i> into <i>item</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#read">QListWidgetItem::read</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_table_widget_item(
    in_: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    item: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTableWidgetItem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__19(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(in_)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTableWidgetItem>>::cast_into(item)
                .as_raw_ptr() as *mut crate::QTableWidgetItem,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a list widget item from stream <i>in</i> into <i>item</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& in, QTreeWidgetItem& item)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QListWidgetItem &item)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a list widget item from stream <i>in</i> into <i>item</i>.</p>
/// <p>This operator uses <a href="http://doc.qt.io/qt-5/qlistwidgetitem.html#read">QListWidgetItem::read</a>().</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_tree_widget_item(
    in_: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    item: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QTreeWidgetItem>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__22(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(in_)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QTreeWidgetItem>>::cast_into(item)
                .as_raw_ptr() as *mut crate::QTreeWidgetItem,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QList<QPair<double, QPointF>>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_list_of_q_pair_of_double_q_point_f(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQPairOfDoubleQPointF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__161(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQPairOfDoubleQPointF>>::cast_into(
                l,
            )
            .as_raw_ptr() as *mut crate::QListOfQPairOfDoubleQPointF,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QList<QPair<double, double>>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_list_of_q_pair_of_double_double(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQPairOfDoubleDouble>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__162(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQPairOfDoubleDouble>>::cast_into(l)
                .as_raw_ptr() as *mut crate::QListOfQPairOfDoubleDouble,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QList<QRectF>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_list_of_q_rect_f(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQRectF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__164(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQRectF>>::cast_into(l).as_raw_ptr()
                as *mut crate::QListOfQRectF,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QList<QWizard::WizardButton>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(cpp_lib_version = "5.14.0"))
)]
#[cfg(any(cpp_lib_version = "5.14.0", feature = "ritual_rustdoc"))]
pub unsafe fn shr_q_data_stream_q_list_of_wizard_button(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfWizardButton>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__173(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfWizardButton>>::cast_into(l)
                .as_raw_ptr() as *mut crate::QListOfWizardButton,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVector<QColor>& v)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_vector_of_q_color(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    v: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQColor>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__196(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQColor>>::cast_into(v)
                .as_raw_ptr() as *mut crate::QVectorOfQColor,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QMap<Qt::GestureType, bool>& map)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(cpp_lib_version = "5.14.0"))
)]
#[cfg(any(cpp_lib_version = "5.14.0", feature = "ritual_rustdoc"))]
pub unsafe fn shr_q_data_stream_q_map_of_gesture_type_bool(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    map: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMapOfGestureTypeBool>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__201(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMapOfGestureTypeBool>>::cast_into(map)
                .as_raw_ptr() as *mut crate::QMapOfGestureTypeBool,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QMap<QDate, QTextCharFormat>& map)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_map_of_q_date_q_text_char_format(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    map: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMapOfQDateQTextCharFormat>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__203(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMapOfQDateQTextCharFormat>>::cast_into(
                map,
            )
            .as_raw_ptr() as *mut crate::QMapOfQDateQTextCharFormat,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QPair<double, QPointF>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_pair_of_double_q_point_f(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfDoubleQPointF>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__207(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfDoubleQPointF>>::cast_into(p)
                .as_raw_ptr() as *mut crate::QPairOfDoubleQPointF,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QPair<double, double>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_pair_of_double_double(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfDoubleDouble>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_widgets_ffi_operator__208(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfDoubleDouble>>::cast_into(p)
                .as_raw_ptr() as *mut crate::QPairOfDoubleDouble,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}
