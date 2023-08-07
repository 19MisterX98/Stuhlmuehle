use std::collections::HashMap;
use std::collections::hash_map::Entry;
use crate::logic::board::Board;

pub fn start() {
    let board = Board::default();
    let mut already_encountered: HashMap<Board, Winnable> = HashMap::new();

    let e = step(&board, &mut already_encountered, 0);
    println!("{:#?}", e);

    println!("all: {}", already_encountered.len());
    println!("unknown: {}", already_encountered.iter().filter(|(_, &x)| x == Winnable::Unknown).count());
    println!("winning: {}", already_encountered.iter().filter(|(_, &x)| x == Winnable::Winning).count());
    println!("losing: {}", already_encountered.iter().filter(|(_, &x)| x == Winnable::Losing).count());

    //already_encountered.iter().filter(|(_, &winnable)| winnable == Winnable::Losing).for_each(|(x,_)| x.print());
}

fn step(board: &Board, already_encountered: &mut HashMap<Board, Winnable>, recursion_lvl: usize) -> Winnable {
    if recursion_lvl == usize::MAX { // optionally limit the amount of steps
        return Winnable::Unknown;
    }
    let mut unknown_or_lose = Winnable::Losing;
    for i in 0..9 {
        if board.get_field(i) == 0 {
            let mut new_board = board.clone();
            new_board.turn(i);

            if let Entry::Occupied(entry) = already_encountered.entry(new_board.clone()) {
                match entry.get() {
                    Winnable::Winning => {return Winnable::Winning}
                    Winnable::Losing => {}
                    Winnable::Unknown => {unknown_or_lose = Winnable::Unknown}
                }
            } else if new_board.is_won() {
                already_encountered.insert(new_board, Winnable::Winning);
                return Winnable::Winning
            } else { // board is unknown and we need to recurse
                already_encountered.insert(new_board.clone(), Winnable::Unknown); // insert this up front so we the program doesnt create recursive loops

                match step(&new_board, already_encountered, recursion_lvl + 1) {
                    Winnable::Winning => { // opponent is winning -> we lose
                        already_encountered.insert(new_board, Winnable::Losing);
                    }
                    Winnable::Losing => { // opponent is losing -> we win
                        already_encountered.insert(new_board, Winnable::Winning);
                        return Winnable::Winning;
                    }
                    Winnable::Unknown => { // opponent isn't winning -> we arent losing
                        unknown_or_lose = Winnable::Unknown;
                    }
                }
            }
        }
    }
    return unknown_or_lose
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Winnable {
    Winning,
    Losing,
    Unknown
}