use std::mem::drop;
use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender};
use std::{thread, time::Duration};

fn clone_experiment() -> Vec<i32> {
    let (tx, rx) = mpsc::channel();
    let mut receiver_vec: Vec<i32> = vec![];

    let tx2 = tx.clone();

    thread::spawn(move || {
        for i in 1..6 {
            tx.send(i)
                .expect("There was an issue sending your tranmission from within the new thread.");
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..11 {
        tx2.send(i)
            .expect("There was an issue sending your tranmission from within the main thread.");
        thread::sleep(Duration::from_secs(1));
    }

    drop(tx2);

    for reciever in rx {
        receiver_vec.push(reciever);
        println!("{:?}", reciever);
    }

    receiver_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_jump() {
        let vector = clone_experiment();
        println!("{:?}", vector);
        assert_eq!(vector.len(), 15);
    }
}
