# Consumable_Vec &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] 

[Build Status]: https://img.shields.io/github/workflow/status/serde-rs/serde/CI/master
[actions]: https://github.com/serde-rs/serde/actions?query=branch%3Amaster

**Consumable_Vec is a generic approach to create a mutual database for multiple producers and consumers**

---

## Concept

The crate offers a trait for data consumption as well as an implementation for sahred and unshared consumable vectors.
In both cases the idea is that producers can `add` data to a Vector at any time. When this data is consumed it is then 
removed from the datapool.
When using the unshared implementation, the caller has to take care of the ownership of the data to allow mutable access
to it. When using the shared implementation, every user does get a Clone of the consumable_vec to add or consume data.

```rs
 use consumable_vec::{SharedConsumableVec, Consumable};
 use std::thread;


 int main() {

 let con_vec = SharedConsumableVec::new(None);

 let producer = con_vec.clone();
 let consumer = con_vec.clone();
 
 thread::spawn(move || {
     for n in 1..100 {
         producer.add(format!("Produced: {}", n));
     }   
 });

 thread::spawn(move || {
     loop {
     if let Some(consumed) = consumer.consume("Produced".to_string()) {
         println!("{:?}", consumed);
         if consumed.inner().iter().filter(|c| c.contains("99")).count() > 0 {
             break;
         }
     }   
     }
});


 }
```
#### License

<sup>
Licensed under <a href="LICENSE">MIT license</a> 

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Consumable_Vec by you, shall be licensed as above, without any 
additional terms or conditions.
</sub>