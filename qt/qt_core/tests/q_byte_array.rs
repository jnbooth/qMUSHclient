#[test]
fn qrect() {
    unsafe {
        let array = qt_core::QByteArray::new();

        assert!(array.as_slice().is_empty());
        array.append_char(42);
        array.append_char(46);

        assert_eq!(array.as_slice(), &[42, 46]);

        array.as_mut_slice()[1] = 47;

        assert_eq!(array.index_int(0), 42);
        assert_eq!(array.index_int(1), 47);
    }
}
