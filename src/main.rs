use std::io;
use rand::Rng;

struct Parrot {
    known_frazes: Vec<String>,
}

impl Parrot {
    fn live(&mut self) {
        loop {
            println!("New fraze, that parrot will learn: ");
            let mut ans = String::new();
            
            io::stdin()
                .read_line(&mut ans)
                .expect("Failed to read line!");
           
            let ans_copy = ans;
            self.known_frazes.push(ans_copy);

            let mut ans = String::new();

            println!("You saying:");
            io::stdin()
                .read_line(&mut ans)
                .expect("Failed to read line!");

            println!("But parrot interrupt you:");
            println!("- {}", self.known_frazes[rand::thread_rng().gen_range(0..self.known_frazes.len() - 1)]);
        }
    }
}
                

fn main() {
    let mut parrot = Parrot {
        known_frazes: Vec::new(),
    };

    parrot.known_frazes.push(String::from("Hello!"));
    parrot.known_frazes.push(String::from("Goodbay!"));

    parrot.live();
}
