use std::time::{Duration, SystemTime, SystemTimeError};

pub type WebeID = u64;

pub struct WebeIDFactory {
    epoch: SystemTime,
    last_duration_ms: u128,
    node_id: u64,
    sequence: u16,
}

#[derive(Debug)]
pub enum WebeIDError {
    BadEpoch,  // provided epoch is already out of max time frame from system time
    BadLastDuration, // provided last time is already out of time frame from epoch
    SequenceOverflow, // overflowed the 16bit sequence counter
    SystemTimeError(SystemTimeError),
    TimeRewind,  // system clock may have drifted backwards
}

impl From<SystemTimeError> for WebeIDError {
    fn from(err: SystemTimeError) -> WebeIDError {
        WebeIDError::SystemTimeError(err)
    }
}

impl WebeIDFactory {
    pub fn new(epoch: SystemTime, node_id: u8) -> Result<WebeIDFactory, WebeIDError> {
        // check that provided epoch system time isn't already out of range of max time frame
        SystemTime::now().duration_since(epoch + Duration::from_millis(std::u16::MAX as u64))?;

        Ok(WebeIDFactory {
            epoch: epoch,
            last_duration_ms: 0,
            node_id: (node_id as u64) << 16,
            sequence: 0,
        })
    }

    // 'last_time_ms' = last duration in ms since provided epoch
    // same as 'new' but can provide last known run time - in case of planned system restarts
    pub fn new_with_last_time(epoch: SystemTime, last_duration_ms: u64, node_id: u8) -> Result<WebeIDFactory, WebeIDError> {
        let mut factory = WebeIDFactory::new(epoch, node_id)?;
        // check that current duration since epoch is greater than last time since epoch
        if SystemTime::now().duration_since(epoch)?.as_millis() <= last_duration_ms as u128 {
            return Err(WebeIDError::BadLastDuration);
        }
        factory.last_duration_ms = last_duration_ms as u128;
        return Ok(factory);
    }

    pub fn next(&mut self) -> Result<WebeID, WebeIDError> {
        let cur_duration_ms = SystemTime::now().duration_since(self.epoch)?.as_millis();
        // for security - verify time has not gone backwards since factory was created.
        if cur_duration_ms < self.last_duration_ms {return Err(WebeIDError::TimeRewind)}
        if cur_duration_ms > self.last_duration_ms {self.sequence = 0} // reset sequence
        self.last_duration_ms = cur_duration_ms;
        let new_id = ((cur_duration_ms as u64) << 24) | (self.node_id) | (self.sequence as u64);
        match self.sequence.checked_add(1) {
            Some(new_sequence) => self.sequence = new_sequence,
            None => return Err(WebeIDError::SequenceOverflow)
        }
        return Ok(new_id);   
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
