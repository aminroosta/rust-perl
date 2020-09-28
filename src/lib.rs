use utils::expose;
use redis::{Client, Connection};
use serde_cbor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Tick {
    quote: String,
    // ask: String,
    // bid: String,
    // epoch: String,
    // symbol: String
}

struct Person {
    name: String,
    lucky_number: i32,
    con: Connection,
}

impl Person {
    fn new(name: &str, lucky_number: i32) -> Person {
        let client = redis::Client::open("redis://127.0.0.1:6360/").unwrap();
        let con = client.get_connection().unwrap();
        Person {
            name: String::from(name),
            lucky_number,
            con
        }
    }

    fn spot_min_max(&mut self, key: &str, start: u64, end: u64) -> String {
        let res = redis::cmd("ZRANGEBYSCORE").arg(key).arg(start).arg(end).query(&mut self.con).unwrap();
        // dbg!(res);

        // let quotes: Vec<_> = res.into_iter().map(|v| {
        //     let t : Tick = serde_cbor::from_reader(v.as_slice()).unwrap();
        //     t.quote
        // }).collect();

        // let min = quotes.iter().min().unwrap();
        // let max = quotes.iter().max().unwrap();

        // format!("{} {}", min, max)
        "".to_string()
        // res.to_string()
    }

    fn name(&self) -> String {
        String::from(&self.name)
    }

    fn lucky_number(&mut self) -> i32 {
        self.lucky_number += 1;
        self.lucky_number
    }
}

expose!{
	Person {
		fn new(name: &str, lucky_number: i32) -> Person;
		fn name(&self) -> String;
        fn lucky_number(&self) -> i32;
        fn spot_min_max(&self, key: &str, start: u64, end: u64) -> String;
	}
}

