use std::{default, fmt::format};

use rand::{distributions::Alphanumeric, Rng};

struct Tester {
    tickets: Vec<String>,
}

struct Ticket {
    id: String,
    flagged: bool,
}

struct Rocker {
    tickets: Vec<Ticket>,
    max: i32,
}

impl Default for Ticket {
    fn default() -> Self {
        Self {
            id: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(12)
                .map(char::from)
                .collect(),
            flagged: false,
        }
    }
}

impl Default for Rocker {
    fn default() -> Self {
        Self {
            tickets: vec![],
            max: 50,
        }
    }
}
impl Default for Tester {
    fn default() -> Self {
        Self { tickets: vec![] }
    }
}

fn spawn_ticket() -> Ticket {
    Ticket {
        id: rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect(),
        flagged: false,
    }
}

fn set_rocker(max: i32) -> Rocker {
    Rocker {
        tickets: vec![],
        max,
    }
}

fn set_tester() -> Tester {
    Tester { tickets: vec![] }
}

fn test_tickets(rocker: &mut Rocker, tester: &mut Tester) {
    for i in 0..rocker.max as usize {
        let x = tester.tickets[i].clone();
        let mut check = false;
        for n in 0..rocker.tickets.len() {
            if x == rocker.tickets[n].id {
                println!("ticket {} okay", i);
                check = true
            }
        }
        if check == false {
            println!("ticket {} IS FUCKED!", i);
        }
    }
}
fn test_faketickets(rocker: &mut Rocker, tester: &mut Tester) {
    for i in 0..rocker.max as usize {
        let mut x: String;
        if i % 3 == 0 {
            x = tester.tickets[i].clone();
        } else {
            x = spawn_ticket().id;
        };
        let mut check = false;
        for n in 0..rocker.tickets.len() {
            if x == rocker.tickets[n].id {
                println!("ticket {} okay", i);
                check = true
            }
        }
        if check == false {
            println!("ticket {} IS FUCKED!", i);
        }
    }
}

fn main() {
    let args = std::env::args()
        .nth(1)
        .expect("please input maximum ticket capacity");
    // Can add argument to read file of saved tickets
    // may be useful for permanent passes
    let max = match args.parse::<i32>() {
        Ok(val) => val,
        Err(e) => {
            println!("unable to parse number from argument {}", e);
            return;
        }
    };
    let mut i = 0;
    let mut rocker = set_rocker(max);
    let mut tester = set_tester();
    while i < max {
        let x = spawn_ticket();
        let y = x.id.clone();
        rocker.tickets.push(x);
        tester.tickets.push(y);
        i += 1;
    }
    //test_tickets(&mut rocker, &mut tester);
    //println!("\nevery third ticket is good");
    //test_faketickets(&mut rocker, &mut tester);
    //println!("\ntickets look like this: {}", spawn_ticket().id)
}
