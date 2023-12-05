use crate::utils;
use regex::Regex;

///Day 2 solution
pub fn day2() -> (usize, usize) {
    let games: Vec<String> = utils::parse_input("input/day2.txt");
    let game_re = Regex::new(r"Game (?<id>\d+): (?<grabs_list>.*)").unwrap();
    let blocks_re = Regex::new(r"(?<number>\d+) (?<color>\w+)").unwrap();
    // Each game in games is a string like this "Game 4: 2 red, 3 blue, 1 green; 10 blue, 3 green; 8 red, 1 blue"
    // For each game we want to build a Game struct with an id and a vector of Grabs. A Grab is the bit between the semi-colons
    let games = games
        .iter()
        .map(|game| {
            let captures = game_re.captures(game).unwrap();
            let mut game = Game::new(captures["id"].parse().unwrap());
            //Map grabs to Vec of Grab objects
            let grabs = &captures["grabs_list"]
                .split(';')
                .map(|grab| {
                    let grab = grab.trim();
                    let captures = blocks_re.captures_iter(grab);
                    let mut num_red: usize = 0;
                    let mut num_green: usize = 0;
                    let mut num_blue: usize = 0;
                    for capture in captures {
                        match &capture["color"] {
                            "red" => num_red += &capture["number"].parse().unwrap(),
                            "green" => num_green += &capture["number"].parse().unwrap(),
                            "blue" => num_blue += &capture["number"].parse().unwrap(),
                            _ => panic!("No clue what this color is!"),
                        }
                    }
                    Grab::new(num_red, num_green, num_blue)
                })
                .collect::<Vec<Grab>>();
            game.grabs = grabs.to_vec();
            game
        })
        .collect::<Vec<Game>>()
        .to_vec();

    let bag = Bag::new(12, 13, 14);

    //Sum the IDs of the possible games
    (
        //part 1
        games
            .iter()
            .filter(|game| {
                //Only include games that are possible
                game.is_possible(&bag)
            })
            .map(|game| game.id)
            .sum(),
        //part 2
        games
            .iter()
            .map(|game| {
                game.grabs.iter().map(|grab| grab.num_red).max().unwrap()
                    * game.grabs.iter().map(|grab| grab.num_green).max().unwrap()
                    * game.grabs.iter().map(|grab| grab.num_blue).max().unwrap()
            })
            .sum(),
    )
}

#[derive(Clone)]
pub struct Grab {
    num_red: usize,
    num_green: usize,
    num_blue: usize,
}

impl Grab {
    pub fn new(num_red: usize, num_green: usize, num_blue: usize) -> Grab {
        Grab {
            num_red,
            num_green,
            num_blue,
        }
    }

    pub fn is_possible(&self, bag: &Bag) -> bool {
        bag.num_red >= self.num_red
            && bag.num_green >= self.num_green
            && bag.num_blue >= self.num_blue
    }
}

#[derive(Clone)]
pub struct Game {
    id: usize,
    grabs: Vec<Grab>,
}

impl Game {
    pub fn new(id: usize) -> Game {
        Game {
            id,
            grabs: Vec::new(),
        }
    }

    pub fn is_possible(&self, bag: &Bag) -> bool {
        //A game is only possible if all Grabs are possible
        self.grabs.iter().all(|grab| grab.is_possible(bag))
    }
}

pub struct Bag {
    num_red: usize,
    num_green: usize,
    num_blue: usize,
}

impl Bag {
    pub fn new(num_red: usize, num_green: usize, num_blue: usize) -> Bag {
        Bag {
            num_red,
            num_green,
            num_blue,
        }
    }
}
