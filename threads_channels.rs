use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::error::Error;
use std::sync::mpsc::{channel, Receiver, Sender};

const FINISHED_CODE: f64 = 0.234234362345785346523;




fn read(reader: Receiver<f64>) {
    loop {
        match reader.recv() {
            Ok(msg) => {
                if msg == FINISHED_CODE {
                    break;
                }
                println!("Message is from the read function: {:?}", msg);
                
            }
            Err(err) => {
                match err.source() {
                    Some(msg) => println!("Error is: {:?}", msg),
                    _ => (), // OK, no error, nothing more to read
                }
                break;
            }
        }
    }
}

fn looping_function(sender: Sender<f64>) {
    println!("Looping_function began...");

    for i in 1..10 {
        sender.send(i as f64).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Looping_function finished");
    sender.send(FINISHED_CODE).unwrap();
}






fn main() {

    //EXAMPLE 1


    let handle = thread::spawn(|| {
        for i in 1..3 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..3 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //EXAMPLE 2


    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi from example 2");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    //EXAMPLE 3

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi from example 3"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }


    //EXAMPLE 4

    let (tx_looping_function, rx_looping_function): (Sender<f64>, Receiver<f64>) = channel();
    let tx2_looping_function = tx_looping_function.clone();

    thread::spawn(move || looping_function(tx2_looping_function));

    read(rx_looping_function);
}
