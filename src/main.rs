use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Record {
    number_of: i32,
    win: i32,
    lose: i32,
    win_rate: f32,
    lose_rate: f32,
}

impl Record {
    fn new() -> Self {
        Self {
            number_of: 0,
            win: 0,
            lose: 0,
            win_rate: 0.0,
            lose_rate: 0.0,
        }
    }

    fn number_of(&mut self) {
        self.number_of += 1;
        self.win_rate = (self.win / self.number_of * 100) as f32;
        self.lose_rate = (self.lose / self.number_of * 100) as f32;
    }
    fn win(&mut self) {
        self.win += 1;
        println!("勝ち")
    }
    fn lose(&mut self) {
        self.lose += 1;
        println!("負け")
    }
}

fn main() {
    let mut record = Record::new();

    println!("1: ぐー\n2: ちょき\n3: パー");

    for _ in 0..3 {
        match (input::<i8>() - thread_rng().gen_range(1..=3) + 3) % 3 {
            1 => record.lose(),
            2 => record.win(),
            _ => println!("引き分け"),
        }

        record.number_of();
    }
    println!("{:?}", record)
}

fn input<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.trim().parse().ok().unwrap()
}
