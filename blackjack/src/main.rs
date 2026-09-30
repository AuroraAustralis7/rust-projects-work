use std::collections::HashMap; // Adds Hashmaps
use rand::seq::SliceRandom; // Adds shuffle method
use rand::Rng; // Adds random number generator

fn main() {
    let deck_map = deck_map();
    let mut deck = create_deck();
    let mut rng = rand::rng();
    deck.shuffle(&mut rng);
    // println!("{:?}", deck);
    println!("Hello, gambler!");
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