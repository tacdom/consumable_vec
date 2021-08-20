use std::sync::{Arc, Mutex};

pub trait Consumable {
    type Item;

    fn consume(&self, pattern: &str) -> Option<Self::Item>;

    fn consume_mut(&mut self, pattern: &str) -> Option<Self::Item>;
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

    fn consume_mut(&mut self, pattern: &str) -> Option<Self::Item> {
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

    fn consume(&self, _: &str) -> Option<Self::Item> {
        todo!()
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

    fn consume(&self, pattern: &str) -> Option<Self::Item> {
        self.data.lock().unwrap().consume_mut(pattern)
    }

    // do not implement mutable consumer for the shared reference
    fn consume_mut(&mut self, _: &str) -> Option<Self::Item> {
        None
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
        assert!(at.consume_mut("pattern").is_none());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_return_some() {
        let mut at = ConsumableVec::new(None);
        at.add("data".to_string());
        assert!(at.consume_mut("da").is_some());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_have_data() {
        let mut at = ConsumableVec::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        let consumed = at.consume_mut("da").unwrap();
        assert_eq!(1, consumed.len());
        assert_eq!("data".to_string(), consumed.data[0]);
    }

    #[test]
    fn consume_when_pattern_in_replies_multiple_times_should_have_data_multipletimes() {
        let mut at = ConsumableVec::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        at.add("data2".to_string());
        let consumed = at.consume_mut("da").unwrap();
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
        let _ = at.consume_mut("da").unwrap();
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
        assert!(at.consume("pattern").is_none());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_return_some() {
        let mut at = SharedConsumableVec::new(None);
        at.add("data".to_string());
        assert!(at.consume("da").is_some());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_have_data() {
        let mut at = SharedConsumableVec::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        let consumed = at.consume("da").unwrap();
        assert_eq!(1, consumed.len());
        assert_eq!("data".to_string(), consumed.data[0]);
    }

    #[test]
    fn consume_when_pattern_in_replies_multiple_times_should_have_data_multipletimes() {
        let mut at = SharedConsumableVec::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        at.add("data2".to_string());
        let consumed = at.consume("da").unwrap();
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
        let _ = at.consume("da").unwrap();
        assert_eq!(1, at.len());
    }
}
