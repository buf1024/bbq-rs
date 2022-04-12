use serde::{Deserialize, Serialize};

use crate::fetch::RtQuot;

pub const PROTO_VERSION: u8 = 1u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Header {
    pub version: u8,
    pub sid: [u8; 32],
    pub body_len: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Request {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Push {
    Quot(RtQuot),
}

#[cfg(test)]
mod test {
    use crate::fetch::{Quot, RtQuot};
    use super::Push;

    #[test]
    fn test_proto() {
        let mut rt = RtQuot::new();
        let quot = Quot {
            code: "123".to_string(),
            name: "123".to_string(),
            open: 1.2,
            pre_close: 1.44,
            now: 1.33,
            high: 1.99,
            low: 0.44,
            buy: 2.33,
            sell: 4.44,
            vol: 100,
            amount: 4.5,
            bid: ((1, 2.0), (1, 2.0), (1, 2.0), (1, 2.0), (1, 2.0)),
            ask: ((1, 2.0), (1, 2.0), (1, 2.0), (1, 2.0), (1, 2.0)),
            date: "1900-0909".to_string(),
            time: "1900-0909".to_string(),
        };
        rt.insert("123".to_string(), quot);

        let msg = Push::Quot(rt);
        // let s = bincode::serialize(&msg).unwrap();

        // println!("before: {:?}", msg);
        // println!("serialize: {}", s.len());
        // let ds: Push = bincode::deserialize(&s).unwrap();

        // println!("after: {:?}", ds);

        let s = serde_json::to_string(&msg).unwrap();
        println!("json: {}", s);

    }
}
