// Solutions for day21 of Advent of Code

use super::common::run_and_print_time;
use std::collections::HashMap;

struct Player {
    pawn: u32,
    score: u32,
}
impl Player {
    fn new(pawn: u32) -> Player {
        Player { pawn, score: 0 }
    }
}

struct DeterministicDie {
    next_value: u32,
}
impl DeterministicDie {
    fn new() -> DeterministicDie {
        DeterministicDie { next_value: 1 }
    }
    fn roll(&mut self) -> u32 {
        let ret = self.next_value;
        self.next_value = self.next_value % 100 + 1;
        ret
    }
}

fn p1(input: &[String]) -> u32 {
    let mut p1 = Player::new(input[0].rsplit_once(" ").unwrap().1.parse::<u32>().unwrap());
    let mut p2 = Player::new(input[1].rsplit_once(" ").unwrap().1.parse::<u32>().unwrap());
    let mut p1turn = true;

    let mut die = DeterministicDie::new();
    let mut numrolls = 0;

    while p1.score < 1000 && p2.score < 1000 {
        let roll1 = die.roll();
        let roll2 = die.roll();
        let roll3 = die.roll();
        numrolls += 3;

        let sum = roll1 + roll2 + roll3;

        let mut player = if p1turn { &mut p1 } else { &mut p2 };

        let newpawn = (player.pawn + sum - 1) % 10 + 1;
        player.score += newpawn;
        player.pawn = newpawn;

        p1turn = !p1turn;
    }

    if p1.score < p2.score {
        p1.score * numrolls
    } else {
        p2.score * numrolls
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct GameState(u32, u32, u32, u32);

fn process_single_turn(
    p1turn: bool,
    gamestate: &GameState,
) -> Vec<(GameState, u128)> {
    let dirac_die_map = HashMap::from([
        (3, 1), // 1,1,1
        (4, 3), // 1,1,2
        (5, 6), // 1,2,2 / 1,1,3
        (6, 7), // 2,2,2 / 1,2,3
        (7, 6), // 2,2,3 / 1,3,3
        (8, 3), // 2,3,3
        (9, 1), // 3,3,3
    ]);
    let GameState(p1start, p1score, p2start, p2score) = *gamestate;
    dirac_die_map
        .iter()
        .map(|(k, v)| {
            if p1turn {
                let newpawn = (p1start + k - 1) % 10 + 1;
                let newscore = p1score + newpawn;
                (GameState(newpawn, newscore, p2start, p2score), *v)
            } else {
                let newpawn = (p2start + k - 1) % 10 + 1;
                let newscore = p2score + newpawn;
                (GameState(p1start, p1score, newpawn, newscore), *v)
            }
        })
        .collect()
}

fn process_all_turns(
    p1turn: bool,
    state_counts: HashMap<GameState, u128>,
) -> (HashMap<GameState, u128>, u128, u128) {
    let mut new_state_counts = HashMap::new();
    let mut p1wins = 0;
    let mut p2wins = 0;

    for (k, v) in state_counts.iter() {
        for (newstate, count) in process_single_turn(p1turn, k) {
            let GameState(_, p1score, _, p2score) = newstate;
            if p1score < 21 && p2score < 21 {
                *new_state_counts.entry(newstate).or_insert(0) += count * v;
            } else if p1score > p2score {
                p1wins += count * v;
            } else {
                p2wins += count * v;
            }
        }
    }

    (new_state_counts, p1wins, p2wins)
}

fn p2(input: &[String]) -> (u128, u128) {
    let p1start = input[0].rsplit_once(" ").unwrap().1.parse::<u32>().unwrap();
    let p2start = input[1].rsplit_once(" ").unwrap().1.parse::<u32>().unwrap();
    let gamestate = GameState(p1start, 0, p2start, 0);
    let mut state_counts = HashMap::from([(gamestate, 1)]);
    let (mut p1wins, mut p2wins) = (0, 0);

    let mut p1turn = true;
    while !state_counts.is_empty() {
        let (newstatecounts, newp1wins, newp2wins) = process_all_turns(p1turn, state_counts);
        state_counts = newstatecounts;
        p1wins += newp1wins;
        p2wins += newp2wins;
        p1turn = !p1turn;
    }
    (p1wins, p2wins)
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 21 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    println!("Part1: {}", a);

    let (b, timeb) = run_and_print_time(p2, &input);
    println!("Part2: {:?}", b);

    timea + timeb
}
