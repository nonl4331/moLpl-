use colour::*;
use crossterm::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::Rng;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

#[derive(Copy, Clone, FromPrimitive, PartialEq)]
enum Letter {
    A = 'a' as isize,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, PartialEq)]
enum ColourState {
    Grey,
    Yellow,
    Green,
}

impl Letter {
    pub fn get_letter(input: u8) -> Self {
        FromPrimitive::from_i32(input as i32).expect(format!("Invalid Letter: {}", input).as_str())
    }
}

type Word = [Letter; 5];

fn generate_word(word_list: &Vec<Word>) -> [Letter; 5] {
    let mut rng = rand::thread_rng();
    word_list[rng.gen_range(0..word_list.len())]
}

fn string_word(word: Word) -> String {
    let mut ret_word: String = "".to_string();
    for character in word {
        ret_word.push(character as u8 as char);
    }
    ret_word
}

fn get_word_list(filepath: &str) -> Vec<Word> {
    let file = File::open(filepath).unwrap();

    let mut lines = BufReader::new(file).lines();

    let mut word_list = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let word = line.as_bytes();
        if word.len() != 5 {
            panic!("Invalid word in wordlist: {:?}", line);
        }
        let word = [
            Letter::get_letter(word[0]),
            Letter::get_letter(word[1]),
            Letter::get_letter(word[2]),
            Letter::get_letter(word[3]),
            Letter::get_letter(word[4]),
        ];
        word_list.push(word);
    }
    word_list
}

fn get_word_colours(word: &Word, answer: &Word) -> [ColourState; 5] {
    let mut a_vec = Vec::new();

    // default grey state (incorrect)
    let mut colours = [ColourState::Grey; 5];

    // check for correct (green) letters and add the rest to a vec to decide on the colours for them
    for (i, (&letter, &a_letter)) in word.iter().zip(answer).enumerate() {
        if letter == a_letter {
            colours[i] = ColourState::Green;
        }
        a_vec.push((a_vec.len(), a_letter));
    }

    // check for wrong place letters (yellow)
    word.iter().enumerate().for_each(|(l_i, letter)| {
        // check if a_vec contains letter
        let find = a_vec.iter().find(|(_, a_letter)| a_letter == letter);
        let find = match find {
            Some((index, _)) => Some(*index),
            None => None,
        };

        if find.is_some() && colours[l_i] == ColourState::Grey {
            colours[l_i] = ColourState::Yellow;
            a_vec.remove(find.unwrap());
        }
    });
    colours
}

fn print_coloured_word(word: &Word, answer: &Word) {
    let print = get_word_colours(word, answer);

    for (&letter, &print) in word.iter().zip(print.iter()) {
        match print {
            ColourState::Green => {
                green!("{}", letter as u8 as char);
            }
            ColourState::Yellow => {
                yellow!("{}", letter as u8 as char);
            }
            ColourState::Grey => {
                print!("{}", letter as u8 as char);
            }
        }
    }
    println!("");
}

fn string_to_word(string: &String) -> Option<Word> {
    let word = string.as_bytes();
    if word.len() != 5 {
        return None;
    }

    Some([
        Letter::get_letter(word[0]),
        Letter::get_letter(word[1]),
        Letter::get_letter(word[2]),
        Letter::get_letter(word[3]),
        Letter::get_letter(word[4]),
    ])
}

struct WordleGame {
    guesses: [Option<Word>; 6],
    answer: Word,
    word_list: Vec<Word>,
    pub guessed: bool,
}

impl WordleGame {
    pub fn new(word_list: Vec<Word>) -> Self {
        WordleGame {
            guesses: [None; 6],
            answer: generate_word(&word_list),
            word_list,
            guessed: false,
        }
    }
    pub fn guess(&mut self, word: Word) {
        if !self.can_guess() {
            println!("You ran out of guesses!");
            return;
        }

        if !self.word_list.contains(&word) {
            println!("Invalid word: {}", string_word(word));
            println!("please try again");
        } else {
            for guess in self.guesses.iter_mut() {
                if guess.is_none() {
                    *guess = Some(word);
                    self.print_guesses();
                    if word == self.answer {
                        self.guessed = true;
                    }
                    return;
                }
            }
        }
    }

    pub fn print_guesses(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        let size = terminal::size();
        match size {
            Ok((width, height)) => {
                let term_width = width.max(6) as usize;
                let term_height = height.max(7) as usize;

                let width_gap_size = ((term_width - 6) / 10).min(3);
                let height_gap_size = ((term_height - 7) / 12).min(1);

                let lr_gap = (term_width as i32 - (width_gap_size * 10 + 6) as i32) as usize / 2;
                let ud_gap = (term_height as i32 - (height_gap_size * 12 + 7) as i32) as usize / 2;

                let mut cursor_y = ud_gap + 1;

                std::io::stdout()
                    .execute(cursor::MoveTo((lr_gap + 1) as u16, cursor_y as u16))
                    .unwrap();

                print!("{}", "-".repeat((width_gap_size * 2 + 2) * 5 + 1));
                cursor_y += 1;
                std::io::stdout()
                    .execute(cursor::MoveTo((lr_gap + 1) as u16, cursor_y as u16))
                    .unwrap();

                for row in 0..6 {
                    for _ in 0..height_gap_size {
                        print!(
                            "{}",
                            ("|".to_owned() + &" ".repeat(2 * width_gap_size + 1)).repeat(5)
                        );
                        print!("|");
                        cursor_y += 1;
                        std::io::stdout()
                            .execute(cursor::MoveTo((lr_gap + 1) as u16, cursor_y as u16))
                            .unwrap();
                    }

                    print!("|");
                    match self.guesses[row] {
                        Some(guess) => {
                            let colours = get_word_colours(&guess, &self.answer);
                            for (colour, letter) in colours.iter().zip(guess.iter()) {
                                print!("{}", " ".repeat(width_gap_size));
                                match colour {
                                    ColourState::Green => {
                                        green!("{}", *letter as u8 as char);
                                    }
                                    ColourState::Yellow => {
                                        yellow!("{}", *letter as u8 as char);
                                    }
                                    ColourState::Grey => {
                                        print!("{}", *letter as u8 as char);
                                    }
                                }
                                print!("{}|", " ".repeat(width_gap_size));
                            }
                        }
                        None => {
                            print!(
                                "{}",
                                format!("{}|", " ".repeat(width_gap_size * 2 + 1)).repeat(5)
                            );
                        }
                    }
                    cursor_y += 1;
                    std::io::stdout()
                        .execute(cursor::MoveTo((lr_gap + 1) as u16, cursor_y as u16))
                        .unwrap();

                    for _ in 0..height_gap_size {
                        print!(
                            "{}",
                            ("|".to_owned() + &" ".repeat(2 * width_gap_size + 1)).repeat(5)
                        );
                        print!("|");
                    }
                    cursor_y += 1;
                    std::io::stdout()
                        .execute(cursor::MoveTo((lr_gap + 1) as u16, cursor_y as u16))
                        .unwrap();
                    print!("{}", "-".repeat((width_gap_size * 2 + 2) * 5 + 1));
                    cursor_y += 1;
                    std::io::stdout()
                        .execute(cursor::MoveTo((lr_gap + 1) as u16, cursor_y as u16))
                        .unwrap();
                }

                cursor_y += 1;
                std::io::stdout()
                    .execute(cursor::MoveTo((lr_gap + 1) as u16, cursor_y as u16))
                    .unwrap();
                std::io::stdout().flush().unwrap();
            }
            Err(_) => {
                let mut iter = self.guesses.iter();
                while let Some(option_guess) = iter.next() {
                    if let Some(guess) = option_guess {
                        print_coloured_word(&guess, &self.answer);
                    }
                }
            }
        }
    }
    pub fn can_guess(&self) -> bool {
        self.guesses[5].is_none()
    }
}

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    std::io::stdout().flush().unwrap();
    let word_list = get_word_list("words.txt");
    let mut game = WordleGame::new(word_list);
    while game.can_guess() && !game.guessed {
        let mut input = "".to_string();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(e) => {
                println!("Invalid input: {}!", e);
                continue;
            }
        }
        input.truncate(input.len() - 1);
        match string_to_word(&input) {
            Some(word) => game.guess(word),
            None => println!("Word not on length 5!"),
        }
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    if game.guessed {
        println!("You won!");
    } else {
        println!(
            "You ran out of guesses! The word was {}",
            string_word(game.answer)
        );
    }
}
