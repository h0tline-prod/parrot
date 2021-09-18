use std::io;
use rand::Rng;

struct Parrot {
    name: String,
    famous_frazes: Vec<String>,
//    live: bool,
}

impl Parrot {
    fn live(&mut self) {
        loop {
            println!("New fraze, that parrot will learn: ");
            let mut ans = String::new();
            
            io::stdin()
                .read_line(&mut ans)
                .expect("Failed to read line!");
           
//            if ans == String::from("quit") {
//                self.live = false;
//            } else {
                let ans_copy = ans;
                self.famous_frazes.push(ans_copy);
//            }

//            if !self.live {
//                break;
//            }

            let mut ans = String::new();

            println!("You saying:");
            io::stdin()
                .read_line(&mut ans)
                .expect("Failed to read line!");
           
            println!("But {} interrupt you:", self.name);
            println!("- {}", self.famous_frazes[rand::thread_rng().gen_range(0..self.famous_frazes.len())]);
        }
    }
}
                

fn main() {
    let mut ans = String::new();

    println!("Give a name to your parrot:");
    io::stdin()
        .read_line(&mut ans)
        .expect("Failed to read line!");

    let mut parrot = Parrot {
        name: ans,
        famous_frazes: vec![String::from("Hello!"), String::from("Goodbay!")], 
//        live: true,
    };

    parrot.live();
}

