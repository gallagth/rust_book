use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    result: HashMap<u32, u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(function: T) -> Cacher<T> {
        Cacher{
            calculation: function,
            result: HashMap::new()
        }
    }

    fn result(&mut self, arg: u32) -> u32 {
        let closure = &self.calculation;
        *self.result.entry(arg).or_insert_with(|| {(closure)(arg)})
    }
}


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 3;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut business_logic = Cacher::new(|x| {
        println!("Doing complicated stuff.....");
        thread::sleep(Duration::from_secs(2));
        x
    });

    if intensity < 25 {
        println!("Do {} pushups!", business_logic.result(intensity));
        println!("Do {} situps!", business_logic.result(intensity+10));
        println!("Do {} sit downs!", business_logic.result(intensity));
    } else {
        if random_number == 3 {
            println!("Have a rest today!");
        } else {
            println!("Run for {} minutes", business_logic.result(intensity));
        }
    }
}
