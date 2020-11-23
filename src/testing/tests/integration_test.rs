use testing;

mod common;

#[test]
fn can_hold_integration() {
    common::setup();

    let a = testing::Rectangle {
        width: 5,
        height: 2,
    };

    let b = testing::Rectangle {
        width: 3,
        height: 1,
    };

    assert!(a.can_hold(&b));
}
