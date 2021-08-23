// Copyright (c) Siemens AG, 2021
//
// Authors:
//  Dominik Tacke <dominik.tacke@siemens.com>
//
// This work is licensed under the terms of the MIT.  See
// the LICENSE-MIT file in the top-level directory.
//
// SPDX-License-Identifier: MIT


//! Provides a vector which content can be consumed
//!
//! This allows multiple producers to add data to a shared data base. Any consumer
//! can take out data in a deleting manner if certain criteria are met, e.g. a search
//! pattern is fulfilled in a `String` implementation.
//! 
//! This crate provides two different implementations:
//! The struct `ConsumableVec` is a plain implementation of the trait `Consumable`. Here the 
//! user needs to take care of the ownership of the object when adding data or trying to consume
//! data from it.    
//! The struct `SharedConsumableVec` uses a `ConsumableVec` which can be referenced by multiple owners
//! from multiple threads. 

use std::sync::{Arc, Mutex};

/// Consume content from a data collection
///
/// Consumption can be inmplemented in a mutabl or immutable way, or both. 
/// Which implementation needs to be used is dependent on the using application.
///
///
/// Example:
/// In this example, `consume_mut` will take all entries of type u16 which are greater than
/// the input value
/// ```
/// use consumable_vec::Consumable;
///
/// struct Example {
///     data : Vec<u16>   
/// }  
///
/// impl Consumable for Example {
///   type Item = Example;
///   type DataType = u16;
///
///   fn consume_mut(&mut self, pattern: u16) -> Option<Example> {
///             let val = self
///                 .data
///                 .iter()
///                 .filter(|r| *r > &pattern)
///                 .map(|x| x.to_owned()) 
///                 .collect::<Vec<u16>>();
///             self.data.retain(|d| d <= &pattern);
///
///             if !val.is_empty() {
///                 Some(Example {data:val})
///             } else {
///                 None
///             }
///     }
/// }
///      
pub trait Consumable {
    type Item;
    type DataType;

    /// # Immutable consume method    
    /// This shall be implemented for shared access, e.g. when inner Vector 
    /// uses reference counters and mutexes to be changed.
    fn consume(&self, _pattern: Self::DataType) -> Option<Self::Item> {
        unimplemented!();
    }

    /// # Mutable consume method    
    /// This allows to directly manipiulate the internal data. Here the caller needs
    /// to take care of ownership
    fn consume_mut(&mut self, _pattern: Self::DataType) -> Option<Self::Item> {
        unimplemented!();
    }
}

#[derive(Debug,Clone)]
pub struct ConsumableVec<T> {
    data: Vec<T>,
}

impl<T> ConsumableVec<T> {
    fn new(data: Option<Vec<T>>) -> Self {
        ConsumableVec {
            data: match data {
                Some(d) => d,
                None => Vec::new(),
            },
        }
    }

    fn add(&mut self, reply: T) {
        self.data.push(reply);
    }

    fn clear(&mut self ) {
        self.data.clear();
    }

    pub fn inner(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T> len_trait::Len for ConsumableVec<T> {
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> len_trait::Empty for ConsumableVec<T> {
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Consumable for ConsumableVec<String> {
    type Item = ConsumableVec<String>;
    type DataType = String;

    fn consume_mut(&mut self, pattern: Self::DataType) -> Option<Self::Item> {
        let trimmed_pattern = pattern.trim();
        
        let val = self
            .data
            .iter()
            .filter(|r| r.trim().starts_with(trimmed_pattern))
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        
        // remove all values just consumed
        // nighlty rust has drain_filter which could do
        // filtering and removal in one step
        self.data.retain(|d| !d.starts_with(trimmed_pattern));

        if !val.is_empty() {
            Some(ConsumableVec::new(Some(val)))
        } else {
            None
        }
    }
}

#[derive(Debug,Clone)]
pub struct SharedConsumableVec<T> {
    data: Arc<Mutex<ConsumableVec<T>>>,
}

impl<T> SharedConsumableVec<T> {
    pub fn new(data: Option<Vec<T>>) -> Self {
        SharedConsumableVec {
            data: Arc::new(Mutex::new(ConsumableVec::new(data))),
        }
    }

    pub fn add(&self, reply: T) {
        self.data.lock().unwrap().add(reply);
    }

    pub fn clear(&self) {
        self.data.lock().unwrap().clear();
    }
}

impl<T> len_trait::Len for SharedConsumableVec<T> {
    fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }
}

impl<T> len_trait::Empty for SharedConsumableVec<T> {
    fn is_empty(&self) -> bool {
        self.data.lock().unwrap().is_empty()
    }
}

impl Consumable for SharedConsumableVec<String> {
    type Item = ConsumableVec<String>;
    type DataType = String;

    fn consume(&self, pattern: Self::DataType) 
     -> Option<Self::Item> {
        self.data.lock().unwrap().consume_mut(pattern)
    }
}


#[cfg(test)]
mod test_at_replies {
    use super::*;
    use len_trait::Len;

    #[test]
    fn consume_when_pattern_not_in_replies_should_return_none() {
        let mut at = ConsumableVec::new(None);
        at.add("data".to_string());
        assert!(at.consume_mut("pattern".to_string()).is_none());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_return_some() {
        let mut at = ConsumableVec::new(None);
        at.add("data".to_string());
        assert!(at.consume_mut("da".to_string()).is_some());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_have_data() {
        let mut at = ConsumableVec::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        let consumed = at.consume_mut("da".to_string()).unwrap();
        assert_eq!(1, consumed.len());
        assert_eq!("data".to_string(), consumed.data[0]);
    }

    #[test]
    fn consume_when_pattern_in_replies_multiple_times_should_have_data_multipletimes() {
        let mut at = ConsumableVec::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        at.add("data2".to_string());
        let consumed = at.consume_mut("da".to_string()).unwrap();
        assert_eq!(2, consumed.len());
        assert_eq!("data2".to_string(), consumed.data[1]);
    }

    #[test]
    fn consume_should_remove_values_from_data() {
        let mut at = ConsumableVec::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        at.add("data2".to_string());
        assert_eq!(3, at.len());
        let _ = at.consume_mut("da".to_string()).unwrap();
        assert_eq!(1, at.len());
    }
}


#[cfg(test)]
mod test_shared_at_replies {
 
    use super::*;
    use len_trait::Len;

    #[test]
    fn consume_when_pattern_not_in_replies_should_return_none() {
        let mut at = SharedConsumableVec::new(None);
        at.add("data".to_string());
        assert!(at.consume("pattern".to_string()).is_none());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_return_some() {
        let mut at = SharedConsumableVec::new(None);
        at.add("data".to_string());
        assert!(at.consume("da".to_string()).is_some());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_have_data() {
        let mut at = SharedConsumableVec::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        let consumed = at.consume("da".to_string()).unwrap();
        assert_eq!(1, consumed.len());
        assert_eq!("data".to_string(), consumed.data[0]);
    }

    #[test]
    fn consume_when_pattern_in_replies_multiple_times_should_have_data_multipletimes() {
        let mut at = SharedConsumableVec::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        at.add("data2".to_string());
        let consumed = at.consume("da".to_string()).unwrap();
        assert_eq!(2, consumed.len());
        assert_eq!("data2".to_string(), consumed.data[1]);
    }

    #[test]
    fn consume_should_remove_values_from_data() {
        let mut at = SharedConsumableVec::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        at.add("data2".to_string());
        assert_eq!(3, at.len());
        let _ = at.consume("da".to_string()).unwrap();
        assert_eq!(1, at.len());
    }
}
