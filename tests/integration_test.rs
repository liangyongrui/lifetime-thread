use std::{thread, time::Duration};

#[test]
fn it_works() {
    let s = String::from("xxx");
    let outer = lifetime_thread::spawn(s, |inner| {
        println!("begin");
        while let Some(t) = inner.get() {
            println!("ok! {}", t);
            assert_eq!(t, "xxx")
        }
        println!("over")
    });
    thread::sleep(Duration::from_millis(1));
    assert_eq!(*outer, "xxx")
}
