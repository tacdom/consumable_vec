use std::sync::{Arc, Mutex};

pub struct AtReplies {
    data: Vec<String>,
}

impl AtReplies {
    fn new(data: Option<Vec<String>>) -> Self {
        AtReplies {
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
            .filter(|r| r.starts_with(pattern))
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        // remove all values just consumed
        // nighlty rust has drain_filter which could do
        // filtering and removal in one step
        self.data.retain(|d| !d.starts_with(pattern));

        if !val.is_empty() {
            Some(AtReplies::new(Some(val)))
        } else {
            None
        }
    }
}

impl len_trait::Len for AtReplies {
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl len_trait::Empty for AtReplies {
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

pub struct SharedAtReplies {
    data: Arc<Mutex<AtReplies>>,
}

impl SharedAtReplies {
    pub fn new(data: Option<Vec<String>>) -> Self {
        SharedAtReplies {
            data: Arc::new(Mutex::new(AtReplies::new(data))),
        }
    }

    pub fn add(&mut self, reply: String) {
        self.data.lock().unwrap().add(reply);
    }

    pub fn consume(&mut self, pattern: &str) -> Option<AtReplies> {
        self.data.lock().unwrap().consume(pattern)
    }
}

impl len_trait::Len for SharedAtReplies {
    fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }
}

impl len_trait::Empty for SharedAtReplies {
    fn is_empty(&self) -> bool {
        self.data.lock().unwrap().is_empty()
    }
}


#[allow(unused_imports)]
mod test_at_replies {
    use super::AtReplies;
    use len_trait::Len;

    #[test]
    fn consume_when_pattern_not_in_replies_should_return_none() {
        let mut at = AtReplies::new(None);
        at.add("data".to_string());
        assert!(at.consume("pattern").is_none());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_return_some() {
        let mut at = AtReplies::new(None);
        at.add("data".to_string());
        assert!(at.consume("da").is_some());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_have_data() {
        let mut at = AtReplies::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        let consumed = at.consume("da").unwrap();
        assert_eq!(1, consumed.len());
        assert_eq!("data".to_string(), consumed.data[0]);
    }

    #[test]
    fn consume_when_pattern_in_replies_multiple_times_should_have_data_multipletimes() {
        let mut at = AtReplies::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        at.add("data2".to_string());
        let consumed = at.consume("da").unwrap();
        assert_eq!(2, consumed.len());
        assert_eq!("data2".to_string(), consumed.data[1]);
    }

    #[test]
    fn consume_should_remove_values_from_data() {
        let mut at = AtReplies::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        at.add("data2".to_string());
        let consumed = at.consume("da").unwrap();
        assert_eq!(1, at.len());
    }
}


#[allow(unused_imports)]
mod test_shared_at_replies {
 
    use super::SharedAtReplies;
    use len_trait::Len;

    #[test]
    fn consume_when_pattern_not_in_replies_should_return_none() {
        let mut at = SharedAtReplies::new(None);
        at.add("data".to_string());
        assert!(at.consume("pattern").is_none());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_return_some() {
        let mut at = SharedAtReplies::new(None);
        at.add("data".to_string());
        assert!(at.consume("da").is_some());
    }

    #[test]
    fn consume_when_pattern_in_replies_should_have_data() {
        let mut at = SharedAtReplies::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        let consumed = at.consume("da").unwrap();
        assert_eq!(1, consumed.len());
        assert_eq!("data".to_string(), consumed.data[0]);
    }

    #[test]
    fn consume_when_pattern_in_replies_multiple_times_should_have_data_multipletimes() {
        let mut at = SharedAtReplies::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        at.add("data2".to_string());
        let consumed = at.consume("da").unwrap();
        assert_eq!(2, consumed.len());
        assert_eq!("data2".to_string(), consumed.data[1]);
    }

    #[test]
    fn consume_should_remove_values_from_data() {
        let mut at = SharedAtReplies::new(None);
        at.add("data".to_string());
        at.add("ata".to_string());
        at.add("data2".to_string());
        let _ = at.consume("da").unwrap();
        assert_eq!(1, at.len());
    }
}
