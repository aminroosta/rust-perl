use utils::expose;
use redis::Connection;

struct FastDecimate {
    con: Connection,
}

impl FastDecimate {
    fn new(redis_url: &str) -> FastDecimate {
        let client = redis::Client::open("redis://127.0.0.1:6360/").unwrap();
        let con = client.get_connection().unwrap();
        FastDecimate {
            con
        }
    }

    fn spot_min_max(&mut self, key: &str, start: u64, end: u64) -> String {
        let res : Vec<String> = redis::cmd("ZRANGEBYSCORE").arg(key).arg(start).arg(end).query(&mut self.con).unwrap();
        dbg!(res);

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
}

expose!{
	FastDecimate {
        fn new(redis_url: &str) -> FastDecimate;
        fn spot_min_max(&self, key: &str, start: u64, end: u64) -> String;
	}
}

