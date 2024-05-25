mod common;

use template;

#[test]
fn it_adds_two() {
    assert_eq!(4, template::add_two(2));
}
