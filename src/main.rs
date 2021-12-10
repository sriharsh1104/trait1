struct Sheep {}
struct Dog {}
struct Cat {}

trait Animal {
    fn noise(&self) -> &'static str;

    fn name(&self) -> &'static str;
    
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
    fn name(&self) -> &'static str {
        "harry"
    }

}

impl Animal for Dog {
    fn noise(&self) -> &'static str {
        "boh boh!"
    }
    fn name(&self) -> &'static str {
        "marry"
    }

}
impl Animal for Cat {
    fn noise(&self) -> &'static str {
        "meow"
    }
    fn name(&self) -> &'static str {
         "garry"
    }

}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number <= 2.0 && random_number > 0.0 {
        Box::new(Sheep {})
    } else if random_number > 2.0 && random_number < 5.0 {
        Box::new(Dog {})
    } else {
        Box::new(Cat {})
    }
}
use std::io;

fn main() {
    let mut s = String::new();

    println!("Enter choice :");

    io::stdin()
       .read_line(&mut s)
       .expect("failed to read line");

    let s = s.trim().parse().ok().expect("invalid output");
    let random_num = s;
    let animal = random_animal(random_num);
    
    println!("You've randomly chosen an animal, and it's voice {} his name his {}",animal.noise(),animal.name());
        
}
