use std::time::{Duration, SystemTime, SystemTimeError};

pub type WebeID = u64;

pub struct WebeIDFactory {
    epoch: SystemTime,
    node_id: u64,
    sequence: u16,
}

impl WebeIDFactory {
    pub fn new(epoch: SystemTime, node_id: u8) -> Result<WebeIDFactory, SystemTimeError> {
        // check that provided epoch system time isn't already out of range of max time frame
        SystemTime::now().duration_since(epoch + Duration::from_millis(std::u16::MAX as u64))?;

        Ok(WebeIDFactory {
            epoch: epoch,
            node_id: (node_id as u64) << 16,
            sequence: 0,
        })
    }

    pub fn next(&mut self) -> Result<u64, SystemTimeError> {
        let now = SystemTime::now();
        let duration = now.duration_since(self.epoch)?.as_millis() as u64;
        self.sequence = self.sequence.wrapping_add(1);
        return Ok((duration << 24) | (self.node_id) | (self.sequence as u64));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let epoch = SystemTime::UNIX_EPOCH
            .checked_add(Duration::from_millis(1546300800000)) // 01-01-2019 12:00:00 AM GMT
            .expect("failed to create custom epoch");
        let mut factory = WebeIDFactory::new(epoch, 0u8).unwrap();
        let id = factory.next().unwrap();
        println!("New ID: {:x}", id);
    }
}
