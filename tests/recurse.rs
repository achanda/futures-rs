extern crate futures;

use std::sync::mpsc::channel;

use futures::future::{ok, Future};

#[test]
fn lots() {
    fn doit(n: usize) -> Box<Future<Item=(), Error=()> + Send> {
        if n == 0 {
            Box::new(ok(()))
        } else {
            Box::new(ok(n - 1).and_then(doit))
        }
    }

    let (tx, rx) = channel();
    ::std::thread::spawn(|| {
        doit(1_000).map(move |_| tx.send(()).unwrap()).wait()
    });
    rx.recv().unwrap();
}
