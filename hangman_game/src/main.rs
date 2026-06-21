use rand::RngExt;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use crate::{
    GameResult::{Lost, Won},
    GuessOutcome::{AlreadyTried, WrongInput},
};

enum GameResult {
    Won(String),
    Lost,
}

enum GuessOutcome {
    Correct(char, Vec<usize>),
    AlreadyTried,
    WrongInput,
    WrongGuess(char),
}

fn main() {
    let rng = rand::rng().random_range(0..20);

    let file = File::open("words.txt").expect("words.txt unavaliable");
    let reader = BufReader::new(file);

    let read_result = reader.lines().nth(rng).expect("no words found");

    if let Ok(word) = read_result {
        let stdin = io::stdin();
        match game(stdin.lock(), word) {
            Ok(Won(word)) => println!("\n🦀 Победа! Вы отгадали слово <<{}>> 🦀", word),
            Ok(Lost) => println!("\n🐹 Вы проиграли! 🐹"),
            Err(e) => eprintln!("Ошибка: {}", e),
        }
    } else {
        eprintln!("failed to read");
    }
}

fn game(mut reader: impl BufRead, word: String) -> io::Result<GameResult> {
    let mut tries = 5;
    let len = word.chars().count();
    let mut guess = "_".repeat(len);
    let mut entered: Vec<char> = Vec::with_capacity(len);

    while tries > 0 {
        println!("Слово: {}", guess);
        println!("Осталось попыток: {}\n", tries);

        print!("Введите букву: ");
        io::stdout().flush()?;
        let mut input = String::new();
        reader.read_line(&mut input)?;

        match process_input(&input, &word, &entered) {
            Ok(GuessOutcome::Correct(ch, found_idx)) => {
                guess = guess
                    .chars()
                    .enumerate()
                    .map(|(idx, c)| if found_idx.contains(&idx) { ch } else { c })
                    .collect();
                println!("{}", guess);
            }
            Ok(AlreadyTried) => {
                continue;
            }
            Ok(WrongInput) => {
                println!("\n⚠️ Неверный ввод!\n");
                continue;
            }
            Ok(GuessOutcome::WrongGuess(ch)) => {
                println!("Буква не найдена");
                entered.push(ch);
                tries -= 1;
                continue;
            }
            Err(e) => return Err(e),
        }

        if guess == word {
            return Ok(Won(word));
        }
    }

    Ok(Lost)
}

fn process_input(input: &str, word: &str, entered: &[char]) -> io::Result<GuessOutcome> {
    let mut trimmed = input.trim().chars();
    let ch = trimmed.next();

    if ch.is_none() || trimmed.next().is_some() {
        return Ok(GuessOutcome::WrongInput);
    }

    let ch = ch.unwrap();

    let found_idx: Vec<usize> = word
        .chars()
        .enumerate()
        .filter_map(|(num, c)| if c == ch { Some(num) } else { None })
        .collect();

    if entered.contains(&ch) {
        return Ok(GuessOutcome::AlreadyTried);
    }

    if found_idx.is_empty() {
        return Ok(GuessOutcome::WrongGuess(ch));
    }

    Ok(GuessOutcome::Correct(ch, found_idx))
}
