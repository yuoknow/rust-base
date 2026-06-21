use rand::RngExt;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

fn main() {
    let rng = rand::rng().random_range(0..20);

    let file = File::open("words.txt").expect("words.txt unavaliable");
    let reader = BufReader::new(file);

    let read_result = reader.lines().nth(rng).expect("no word found");

    if let Ok(word) = read_result {
        start_game(word);
    } else {
        eprintln!("failed to read");
    }
}

fn start_game(word: String) {
    let mut tries = 5;
    let len = word.chars().count();
    let mut guess = "_".repeat(len);
    let mut entered: Vec<char> = Vec::with_capacity(len);

    let stdin = io::stdin();

    while tries > 0 {
        println!("Слово: {}", guess);
        println!("Осталось попыток: {}\n", tries);

        print!("Введите букву: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).expect("error on input");
        if input.trim().chars().count() != 1 {
            println!("Неверный ввод");
            continue;
        }

        let ch = input.chars().next().expect("failed to read char");
        let found_idx: Vec<usize> = word
            .chars()
            .enumerate()
            .filter_map(|(num, c)| if c == ch { Some(num) } else { None })
            .collect();

        if entered.contains(&ch) {
            continue;
        }

        if found_idx.is_empty() {
            println!("Буква не найдена");
            entered.push(ch);
            tries -= 1;
            continue;
        }

        guess = guess
            .chars()
            .enumerate()
            .map(|(idx, c)| if found_idx.contains(&idx) { ch } else { c })
            .collect();
        println!("{}", guess);

        if !guess.contains("_") {
            println!("\n🦀 Вы подебили! 🦀");
            break;
        }
    }
}
