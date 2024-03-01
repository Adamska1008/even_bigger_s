use even_bigger_s::{self, S};

#[test]
fn try_this() {
    assert_eq!(S!(), "".to_string());
    assert_eq!(S!("foo"), "foo".to_string());
    assert_eq!(S!("foo" "bar"), "foobar".to_string());
    assert_eq!(S!("hello", "world"), "hello world".to_string());
}
