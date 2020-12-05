use std::{char::MAX, time::Duration};

pub struct FabonaciiBackoff {
    current: u64,
    next: u64,
    max_delay: Option<Duration>,
    max_reties: Option<u64>,
    retries: u64,
}

impl FabonaciiBackoff {
    pub const fn from_millis(millis: u64) -> FabonaciiBackoff {
        Self {
            current: millis,
            next: millis,
            max_delay: None,
            max_reties: None,
            retries: 0,
        }
    }

    pub fn max_delay(mut self, duration: Duration) -> Self {
        self.max_delay = Some(duration);
        self
    }
    pub fn max_reties(mut self, max_reties: u64) -> Self {
        self.max_reties = Some(max_reties);
        self
    }
}

impl Iterator for FabonaciiBackoff {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        match self.max_reties {
            Some(max_retries) => {
                if self.retries >= max_retries {
                    return None;
                }
            }
            None => {
                if self.retries == std::u64::MAX {
                    return None;
                }
            }
        }
        self.retries += 1;

        let duration = Duration::from_millis(self.current);
        if let Some(max_delay) = self.max_delay {
            if max_delay <= duration {
                return Some(max_delay);
            }
        }

        if let Some(next_next) = self.current.checked_add(self.next) {
            self.current = self.next;
            self.next = next_next
        } else {
            self.current = self.next;
            self.next = std::u64::MAX;
        }
        Some(duration)
    }
}

#[test]

fn test_iter() {
    let mut iter = FabonaciiBackoff::from_millis(10);

    for _ in 0..100 {
        println!("{:?}", iter.next());
    }
}
