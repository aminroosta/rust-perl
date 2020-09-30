use utils::expose;
use redis::Connection;

struct FastDecimate {
    con: Connection,
}

impl FastDecimate {
    fn new() -> FastDecimate {
        let client = redis::Client::open("redis://127.0.0.1:6360/").unwrap();
        let con = client.get_connection().unwrap();
        FastDecimate {
            con
        }
    }

    fn spot_min_max(&mut self, key: &str, start: u64, end: u64) -> String {
        let res : Vec<String> = redis::cmd("ZRANGEBYSCORE")
            .arg(key)
            .arg(start)
            .arg(end)
            .query(&mut self.con)
            .unwrap();

        let mut minimum = std::f64::MAX;
        let mut maximum = std::f64::MIN;
        res.iter().for_each(|v| {
            let spot = v.split("::")
                .next()
                .unwrap()
                .parse::<f64>()
                .unwrap();
            if spot < minimum {
                minimum = spot;
            }
            if spot > maximum {
                maximum = spot;
            }
        });

        // dbg!(minimum, maximum);
        format!("{} {}", minimum, maximum)
    }
}

expose!{
	FastDecimate {
        fn new() -> FastDecimate;
        fn spot_min_max(&self, key: &str, start: u64, end: u64) -> String;
	}
}

