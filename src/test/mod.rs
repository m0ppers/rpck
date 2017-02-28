use write_data;

#[test]
fn test_repetitions() {
    let src: &[u8] = &[0u8, 0u8, 0u8];
    let mut dest: Vec<u8> = Vec::new();

    assert!(write_data(src, &mut dest).is_ok());
    assert_eq!(dest, [2, 0]);
}

#[test]
fn test_uniques() {
    let src: &[u8] = &[0u8, 1u8, 2u8];
    let mut dest: Vec<u8> = Vec::new();

    assert!(write_data(src, &mut dest).is_ok());
    assert_eq!(dest, [255 -2, 0, 1, 2]);
}

#[test]
fn test_mixed_repeated_unique() {
    let src: &[u8] = &[0u8, 0u8, 0u8, 1u8];
    let mut dest: Vec<u8> = Vec::new();

    assert!(write_data(src, &mut dest).is_ok());
    assert_eq!(dest, [2, 0, 255, 1]);
}

#[test]
fn test_mixed_unique_repeated() {
    let src: &[u8] = &[0u8, 1u8, 0u8, 0u8, 0u8];
    let mut dest: Vec<u8> = Vec::new();

    assert!(write_data(src, &mut dest).is_ok());
    assert_eq!(dest, [254, 0, 1, 2, 0]);
}

#[test]
fn test_min3_repeated() {
    let src: &[u8] = &[0u8, 0u8];
    let mut dest: Vec<u8> = Vec::new();

    assert!(write_data(src, &mut dest).is_ok());
    assert_eq!(dest, [254, 0, 0]);
}

#[test]
fn test_break_repeated_after_128() {
    let src: &[u8] = &[0u8; 129];
    let mut dest: Vec<u8> = Vec::new();

    assert!(write_data(src, &mut dest).is_ok());
    assert_eq!(dest, [127, 0, 255, 0]);
}

