use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Reading {
    pub qualifier: u8,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Envelope {
    pub device_id: Uuid,
    pub alive: u64,
    pub readings: Vec<Reading>,
}

impl TryFrom<&[u8]> for Envelope {
    type Error = serde_json::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_decoding() {
        let input_envelope: Envelope = Envelope { device_id: Uuid::new_v4(), alive: 1000, readings: vec![Reading { qualifier: b'H', value: 12.0 }] };

        let expected_buf: Vec<u8> = serde_json::to_vec(&(input_envelope.device_id, input_envelope.alive, &input_envelope.readings)).unwrap();

        assert_eq!(
            input_envelope,
            serde_json::from_slice(&expected_buf).unwrap()
        );
    }
}