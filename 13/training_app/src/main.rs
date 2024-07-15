use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, K, V>
    where
        T: Fn(K) -> V,
        K: Eq + Hash + Clone,
        V: Clone,
{
    calculation: T,
    value: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
    where
        T: Fn(K) -> V,
        K: Eq + Hash + Clone,
        V: Clone,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        // match self.value {
        //     Some(v) => v,
        //     None => {
        //         let v = (self.calculation)(arg);
        //         self.value = Some(v);
        //         v
        //     },
        // }
        if let Some(v) = self.value.get(&arg) {
            v.clone()
        } else {
            let v = (self.calculation)(arg.clone());
            self.value.insert(arg.clone(), v.clone());
            v
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

// fn simulated_expensive_calc(intensity: u32) -> u32 {
//     println!("ゆっくり計算しています");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("ゆっくり計算しています");
        thread::sleep(Duration::from_secs(2));
        num    
    });

    if intensity < 25 {
        println!(
            "今日は腕立て伏せを{}回してください",
            expensive_result.value(intensity)
        );
        println!(
            "次に腹筋を{}回してください",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("今日は休憩してください");
        } else {
            println!(
                "今日は{}分間走ってください",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}
