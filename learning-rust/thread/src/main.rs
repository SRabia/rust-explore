use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn _example_thread(){
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move||{
        println!("move vector {:?} from the spawned thread!", v);
        thread::sleep(Duration::from_millis(1));
    });


    for i in 1..5{
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    println!("Hello, world!");
}

fn _single_producer_thread(){

    let (tx, rx) = mpsc::channel();

    thread::spawn(move ||{
        let val = String::from("hi");
        tx.send(val.clone()).unwrap();
        tx.send(val.clone()).unwrap();
        tx.send(val).unwrap();
    });

    //receive all value
    for received in rx.iter(){
        println!("got {}", received);
    }

    // is not ok since rx is empty
    assert_eq!(false, rx.recv().is_ok());

}

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    
    thread::spawn(move ||{
        let vals = vec![1,2,3,4];

        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move ||{

        let vals = vec![5, 6, 7, 8, 9, 10];

        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("got {}", received);
    }

}
