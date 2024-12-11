use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

pub fn parallelize<F>(functions: Vec<F>) -> i64
where
    F: Fn() -> i64 + Send + 'static,
{
    let (tx, rx): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    functions.into_iter().for_each(|f| {
        let tx_2 = tx.clone();
        thread::spawn(move || tx_2.send(f()).unwrap());
    });
    drop(tx);
    rx.iter().reduce(|prev, next| prev + next).unwrap()
}
