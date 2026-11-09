pub fn example_primitive() {
    let orig: i32 = -2000;

    // `chaincast_primitive!` expands to `orig as u8 as i8`, i.e. two plain
    // `as` casts applied left to right. Integer `as` casts truncate instead
    // of rounding or saturating, so each step keeps only the low bits:
    //
    //   -2000i32      == 0xFFFF_F830   (two's complement)
    //   as u8  -> 0x30 == 48            (low byte; upper bits are dropped)
    //   as i8  -> 48                    (0x30 fits in i8, so nothing changes)
    //
    // So the comparison below is against 48 because that is the truncated low
    // byte of -2000.
    let casted = chaincast::chaincast_primitive!(orig; u8, i8);
    assert_eq!(casted, 48);
}
