use pyu_rust_util as pyu;
use rand::prelude::SliceRandom;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut money: f32 = 0.0;
    let mut rng = rand::thread_rng();
    let mut increase: f32 = 4.547;
    let decrease = 3.43;
    let work_delay = Duration::new(1, 0);

    let items = vec![
        "Cool McDude Shirt",
        "Cup",
        "PC",
        "Rocketship",
        "Inflatable Baby",
    ];

    let mut collection: Vec<&str> = Vec::new();

    loop {
        let c = pyu::input("Enter choice: work, money, buy, items, exit: ");

        match c.trim() {
            "work" => {
                println!("You are working");
                money += increase;
                sleep(work_delay);
                println!("You finished working. You have earned {}", increase);
                increase += 0.83;
            }

            "exit" => {
                return;
            }

            "money" => {
                println!("Your balance is: {}", money);
                if 0.0 > money {
                    println!("Your in debt IDIOT.");
                }
            }

            "buy" => {
                if 0.0 > money {
                    println!("You are in debt btw, WAIT WHY ARE YOU STEALING!!11!1!")
                }

                let choice = items.choose(&mut rng);

                println!("You bought a {}", choice.expect("fail").trim());

                money -= decrease;
                collection.insert(0, &choice.expect("fail").trim());
            }

            "items" => {
                println!("Your items: ");

                for v in &collection {
                    println!("{}", v);
                }
            }

            _ => {}
        }
    }
}
