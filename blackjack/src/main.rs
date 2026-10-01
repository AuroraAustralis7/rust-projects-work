use std::collections::HashMap; // Adds Hashmaps
use rand::seq::SliceRandom; // Adds shuffle method
use std::io::{self, Write};

fn main() {
    // deck init
    let deck_map = deck_map();
    let mut deck = create_deck();
    let mut rng = rand::rng();
    deck.shuffle(&mut rng);

    //money dealing phase
    let mut money: u32 = 100;
    println!("You have {money}. You started out with 100.");
    let bet_info = bet(&money);
    money = bet_info.0;
    let bet_amount = bet_info.1;
    println!("You have {money}");
    println!("You bet {bet_amount}");

    // dealer drawing phase
    let draw_info_dealer = draw(deck);
    let host_hand = vec![draw_info_dealer.0, draw_info_dealer.1];
    deck = draw_info_dealer.2;

    println!("HOST HAND: {}, UNKNOWN", &host_hand[0]);

    // player drawing phase
    let draw_info_player = draw(deck);
    let mut player_hand = vec![draw_info_player.0, draw_info_player.1];
    deck = draw_info_player.2;

    // player playing phase
    let player_info = player_play_phase(player_hand, deck);
    player_hand = player_info.0;
    deck = player_info.1;

    // Host playing phase

    println!("YOUR HAND: {}, {}", &player_hand[0],  &player_hand[1]);

    // println!("{:?}", deck);
}

// clears deck and makes new deck
fn deck_map() -> HashMap<String, u32> {
    let mut deck: HashMap<String, u32> = HashMap::new();

    // Dealing with the numbers
    for i in 2..11 {
        deck.insert(format!("{i} OF SPADES"), i);
        deck.insert(format!("{i} OF HEARTS"), i);
        deck.insert(format!("{i} OF CLUBS"), i);
        deck.insert(format!("{i} OF DIAMONDS"), i);
    }

    // Aces
    deck.insert("ACE OF SPADES".to_string(), 1);
    deck.insert("ACE OF HEARTS".to_string(), 1);
    deck.insert("ACE OF CLUBS".to_string(), 1);
    deck.insert("ACE OF DIAMONDS".to_string(), 1);

    // Jacks
    deck.insert("JACK OF SPADES".to_string(), 10);
    deck.insert("JACK OF HEARTS".to_string(), 10);
    deck.insert("JACK OF CLUBS".to_string(), 10);
    deck.insert("JACK OF DIAMONDS".to_string(), 10);

    // Queens
    deck.insert("QUEEN OF SPADES".to_string(), 10);
    deck.insert("QUEEN OF HEARTS".to_string(), 10);
    deck.insert("QUEEN OF CLUBS".to_string(), 10);
    deck.insert("QUEEN OF DIAMONDS".to_string(), 10);

    // Kings
    deck.insert("KING OF SPADES".to_string(), 10);
    deck.insert("KING OF HEARTS".to_string(), 10);
    deck.insert("KING OF CLUBS".to_string(), 10);
    deck.insert("KING OF DIAMONDS".to_string(), 10);

    /*
    for card_pair in deck {
        let card = card_pair.0;
        let value = card_pair.1;
        println!("{card}, {value}");
    }
    */

    deck
}

// creates the deck with all 54 cards
fn create_deck() -> Vec<String> {
    let mut new_deck: Vec<String> = Vec::new();
    for i in 2..11 {
        new_deck.push(format!("{i} OF SPADES"));
        new_deck.push(format!("{i} OF HEARTS"));
        new_deck.push(format!("{i} OF CLUBS"));
        new_deck.push(format!("{i} OF DIAMONDS"));
    }

    // ACES
    new_deck.push("ACE OF SPADES".to_string());
    new_deck.push("ACE OF HEARTS".to_string());
    new_deck.push("ACE OF CLUBS".to_string());
    new_deck.push("ACE OF DIAMONDS".to_string());

    // JACKS
    new_deck.push("JACK OF SPADES".to_string());
    new_deck.push("JACK OF HEARTS".to_string());
    new_deck.push("JACK OF CLUBS".to_string());
    new_deck.push("JACK OF DIAMONDS".to_string());

    // QUEENS
    new_deck.push("QUEEN OF SPADES".to_string());
    new_deck.push("QUEEN OF HEARTS".to_string());
    new_deck.push("QUEEN OF CLUBS".to_string());
    new_deck.push("QUEEN OF DIAMONDS".to_string());

    // KINGS
    new_deck.push("KING OF SPADES".to_string());
    new_deck.push("KING OF HEARTS".to_string());
    new_deck.push("KING OF CLUBS".to_string());
    new_deck.push("KING OF DIAMONDS".to_string());

    new_deck
}

fn draw(deck: Vec<String>) -> (String, String, Vec<String>) {

    let new_deck = deck[0..deck.len()-2].to_vec();
    (deck[deck.len() - 2].to_string(), deck[deck.len()-1].to_string(), new_deck)
}

fn bet(money: &u32) -> (u32, u32) {

    let mut accepted = false;

    let mut return_pair = (0, 0);

    while(accepted == false) {
    println!("How much are you going to bet?");
    println!("You must bet 5 or more, and you cannot bet more than you have.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let bet = input.trim().parse::<u32>().unwrap();

    if bet <= *money && bet >= 5
    {
        println!("Bet accepted.");
        return_pair = (*money - bet, bet);
        accepted = true;
    }
    else {
        println!("Wrong bet amount.");
        return_pair = (*money, 0);
    }
}
return_pair
}

fn player_play_phase(hand: Vec<String>, mut deck: Vec<String>) -> (Vec<String>, Vec<String>) {
    // Takes in the current hand and deck, then returns the modified hand and deck after the player acts.

    let mut player_continuing = true;

    let mut current_hand = hand.clone();

    let mut current_deck = deck.clone();

    while(player_continuing) {
        println!("Draw or pass?");
        println!("To draw, type in 'draw' or 'd'. To pass, type in 'pass' or 'p'.");

        // Take the input from the player. 
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        // trim anything unwanted from the input
        input = input.trim().parse::<String>().unwrap();

        if input == "draw" || input == "d" {
            println!("Drew one card!");
            let mut card = deck.remove(deck.len()-1);
            current_hand.push(card);
            println!("New hand: {:?}", current_hand)
        }
        if input == "pass" || input == "p" {
            println!("Passed.");
            player_continuing = false;
        }
    }

    (current_hand, current_deck)
}

fn host_play_phase(hand: Vec<String>, mut deck: Vec<String>) -> (Vec<String>, Vec<String>){
    // Takes in the current hand and deck, then returns the modified hand and deck after the host acts.
    // Host stops drawing at the standard 17.

    let mut host_continuing = true;

    let mut current_host_hand = hand.clone();

    let mut current_deck = deck.clone();

    let mut hand_value;

    for 

    while(host_continuing) {
        println!("Draw or pass?");
        println!("To draw, type in 'draw' or 'd'. To pass, type in 'pass' or 'p'.");

        
    }

    (current_hand, current_deck)
}