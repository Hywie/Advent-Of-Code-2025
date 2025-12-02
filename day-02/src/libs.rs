pub fn is_valid_id(id: &str) -> bool {
    let count = id.chars().count();

    // If the count it's not divisible by 2, then it's valid
    if count % 2 != 0 {
        return true;
    }

    // If sequence of identical digits is found, then it's invalid
    let half = count / 2;
    if id[..half] == id[half..] {
        return false;
    }

    return true;
}

#[test]
pub fn test_is_valid_id() {
    assert_eq!(is_valid_id("11"), false);
    assert_eq!(is_valid_id("1188511885"), false);
    assert_eq!(is_valid_id("1010"), false);
    assert_eq!(is_valid_id("222222"), false);
    assert_eq!(is_valid_id("95"), true);
    assert_eq!(is_valid_id("1188511880"), true);
}
