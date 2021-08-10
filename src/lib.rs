use std::sync::{Arc, Mutex};

#[derive(Debug,Clone)]
pub struct ConsumableVec {
    data: Vec<String>,
}

impl ConsumableVec {
    fn new(data: Option<Vec<String>>) -> Self {
        ConsumableVec {
            data: match data {
                Some(d) => d,
                None => Vec::new(),
            },
        }
    }

    fn add(&mut self, reply: String) {
        self.data.push(reply);
    }

    fn consume(&mut self, pattern: &str) -> Option<Self> {
        let val = self
            .data
            .iter()
            .filter(|r| r.trim().starts_with(pattern.trim()))
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        // remove all values just consumed
        // nighlty rust has drain_filter which could do
        // filtering and removal in one step
        self.data.retain(|d| !d.starts_with(pattern));

        if !val.is_empty() {
            Some(ConsumableVec::new(Some(val)))
        } else {
            None
        }
    }

    fn clear(&mut self ) {
        self.data.clear();
    }

    pub fn inner(&self) -> &Vec<String> {
        &self.data
    }
}

impl len_trait::Len for ConsumableVec {
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl len_trait::Empty for ConsumableVec {
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[derive(Debug,Clone)]
pub struct SharedConsumableVec {
    data: Arc<Mutex<ConsumableVec>>,
}

impl SharedConsumableVec {
    pub fn new(data: Option<Vec<String>>) -> Self {
        SharedConsumableVec {
            data: Arc::new(Mutex::new(ConsumableVec::new(data))),
        }
    }

    pub fn add(&self, reply: String) {
        self.data.lock().unwrap().add(reply);
    }

    pub fn consume(&self, pattern: &str) -> Option<ConsumableVec> {
        self.data.lock().unwrap().consume(pattern)
    }

    pub fn clear(&self) {
        self.data.lock().unwrap().clear();
    }
}

impl len_trait::Len for SharedConsumableVec {
    fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }
}

impl len_trait::Empty for SharedConsumableVec {
    fn is_empty(&self) -> bool {
        self.data.lock().unwrap().is_empty()
    }
}


#[allow(unused_imports)]
mod test_at_replies {
    use super::ConsumableVec;
    use len_trait::Len;

    #[test]
    fn consume_when_pattern_not_in_replies_should_return_none() {
        let mut at = ConsumableVec::new(None);
        at.add("data".to_string());
        assert!(at.consume("pattern").is_none());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_return_some() {
        let mut at = ConsumableVec::new(None);
        at.add("data".to_string());
        assert!(at.consume("da").is_some());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_have_data() {
        let mut at = ConsumableVec::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        let consumed = at.consume("da").unwrap();
        assert_eq!(1, consumed.len());
        assert_eq!("data".to_string(), consumed.data[0]);
    }

    #[test]
    fn consume_when_pattern_in_replies_multiple_times_should_have_data_multipletimes() {
        let mut at = ConsumableVec::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        at.add("data2".to_string());
        let consumed = at.consume("da").unwrap();
        assert_eq!(2, consumed.len());
        assert_eq!("data2".to_string(), consumed.data[1]);
    }

    #[test]
    fn consume_should_remove_values_from_data() {
        let mut at = ConsumableVec::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        at.add("data2".to_string());
        let consumed = at.consume("da").unwrap();
        assert_eq!(1, at.len());
    }
}


#[allow(unused_imports)]
mod test_shared_at_replies {
 
    use super::SharedConsumableVec;
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
        let _ = at.consume("da").unwrap();
        assert_eq!(1, at.len());
    }
}
