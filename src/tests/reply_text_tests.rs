use std::borrow::Cow;
use crate::reply_text::{get_random_element};

const EMPTY: &'static [&'static str] = &[];
const WITH_ONE_ELEMENT: &'static [&'static str] = &["1"];
const WITH_MORE_THAN_ONE_ELEMENT: &'static [&'static str] = &["1", "2"];

#[test]
pub fn test_get_random_element_from_empty() {
    assert_eq!(get_random_element(Cow::from(&EMPTY[..])), "");
}

#[test]
pub fn test_get_random_element_from_non_empty_with_one() {
    assert_eq!(get_random_element(Cow::from(&WITH_ONE_ELEMENT[..])), "1");
}

#[test]
pub fn test_get_random_element_from_non_empty_with_more_than_one() {
    let elem = get_random_element(Cow::from(&WITH_MORE_THAN_ONE_ELEMENT[..]));
    assert_eq!(elem == "1" || elem == "2", true);
}