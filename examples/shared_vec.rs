use consumable_vec::{Consumable, SharedConsumableVec};
use std::{thread, time::Duration};

fn main() {
    let con_vec = SharedConsumableVec::default();

    let prod_vec = con_vec.clone();

    let producer = thread::spawn(move || {
        for n in 1..100 {
            prod_vec.add(format!("Produced: {}", n));
            thread::sleep(Duration::from_millis(n));
        }
    });

    let consumer = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));
        if let Some(consumed) = con_vec.consume("Produced".to_string()) {
            println!("{:?}", consumed);
            if consumed.inner().iter().filter(|c| c.contains("99")).count() > 0 {
                break;
            }
        }
    });

    producer.join().expect("Could not join producer");
    consumer.join().expect("Could not join consumer");
}
