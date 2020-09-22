use utils::expose;

struct Person {
    name: String,
    lucky_number: i32,
}

impl Person {
    fn new(name: &str, lucky_number: i32) -> Person {
        Person {
            name: String::from(name),
            lucky_number: lucky_number,
        }
    }

    fn name(&self) -> String {
        String::from(&self.name)
    }

    fn lucky_number(&self) -> i32 {
        self.lucky_number
    }
}

expose!{
	Person {
		fn new(name: &str, lucky_number: i32) -> Person;
		fn name(&self) -> String;
        fn lucky_number(&self) -> i32;
	}
}

