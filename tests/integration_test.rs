use std::{thread, time::Duration};

#[test]
fn it_works() {
    let s = "xxx";
    let outer = lifetime_thread::spawn(s, |inner| {
        println!("begin");
        while let Some(t) = inner.get() {
            println!("ok! {}", t);
            assert_eq!(*t, "xxx")
        }
        println!("over")
    });
    thread::sleep(Duration::from_millis(1));
    assert_eq!(*outer, "xxx")
}

#[async_std::test]
async fn async_works() {
    let s = "xxx";
    let outer = lifetime_thread::async_spawn(s, |inner| async move {
        println!("begin");
        while let Some(t) = inner.get() {
            println!("ok! {}", t);
            assert_eq!(*t, "xxx")
        }
        println!("over")
    });
    async_std::task::sleep(Duration::from_millis(1)).await;
    assert_eq!(*outer, "xxx")
}

/// inner 先死
#[test]
fn it_works2() {
    let s = String::from("xxx");
    let outer = lifetime_thread::spawn(s, |inner| {
        println!("inner: {:?}", inner.get());
    });
    thread::sleep(Duration::from_millis(10));
    assert_eq!(*outer, "xxx")
}

#[derive(Debug)]
struct DropTest {
    x: i32,
}
impl Drop for DropTest {
    fn drop(&mut self) {
        println!("drop: {}", self.x);
    }
}
#[test]
fn test_drop() {
    {
        let s = DropTest { x: 123 };
        let outer = lifetime_thread::spawn(s, |_| println!("inner over"));
        thread::sleep(Duration::from_millis(1));
        assert_eq!(outer.x, 123);
        println!("outer over")
    }
    thread::sleep(Duration::from_millis(10));
}

#[test]
fn test_drop2() {
    {
        let s = DropTest { x: 123 };
        let outer = lifetime_thread::spawn(s, |inner| {
            println!("begin");
            while let Some(t) = inner.get() {
                thread::sleep(Duration::from_millis(10));
                assert_eq!(t.x, 123)
            }
            println!("inner over")
        });
        assert_eq!(outer.x, 123);
        thread::sleep(Duration::from_millis(1));
        println!("outer over")
    }
    thread::sleep(Duration::from_millis(10));
}
#[test]
fn test_drop3() {
    for _ in 0..10 {
        let s = DropTest { x: 123 };
        let outer = lifetime_thread::spawn(s, |inner| {
            println!("begin: {:?}", inner.get());
            println!("inner over")
        });
        println!("outer: {:?}", outer.x);
        println!("outer over")
    }
}
