use write_data;

#[test]
fn test_repetitions() {
    let src: &[u8] = &[0u8, 0u8, 0u8];// as &[u8];//&[1u8, 2, 3] as &[u8];
    let mut dest: Vec<u8> = Vec::new();

    assert!(write_data(src, &mut dest).is_ok());
    assert_eq!(dest, [2, 0]);
}

#[test]
fn test_uniques() {
    let src: &[u8] = &[0u8, 1u8, 2u8];// as &[u8];//&[1u8, 2, 3] as &[u8];
    let mut dest: Vec<u8> = Vec::new();

    assert!(write_data(src, &mut dest).is_ok());
    assert_eq!(dest, [255 -2, 0, 1, 2]);
}