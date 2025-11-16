use std::{
    io::{self, Write},
    process,
};

use rand::seq::SliceRandom;

fn main() {
    println!(
        "Blackjack.

    * Rules:
        Try to get as close to 21 without going over.
        Kings, Queens, and Jacks are worth 10 points.
        Aces are worth 1 or 11 points.
        Cards 2 through 10 are worth their face value.
        (H)it to take another card.
        (S)tand to stop taking cards.

        On your first play, you can (D)ouble down to increase your bet
        but must hit exactly one more time before standing.
        In case of a tie, the bet is returned to the player.
        The dealer stops hitting at 17."
    );

    let mut money = 5000;

    loop {
        // Check if the player has run out of money:
        if money == 0 {
            println!("You're broke");
            println!("Good thing you weren't playing with real money.");
            println!("Thanks for playing!");
        }

        // Let player enter their bet for this round:
        println!("\nMoney: {}", money);
        let mut bet = get_bet(money);

        // Give the dealer and player two cards from the deck each:
        let mut deck = Deck::new();
        let mut dealer_hand = Hand::new();
        dealer_hand.add_card(deck.deal_card().expect("deck is empty!"));
        dealer_hand.add_card(deck.deal_card().expect("deck is empty!"));

        let mut player_hand = Hand::new();
        player_hand.add_card(deck.deal_card().expect("deck is empty"));
        player_hand.add_card(deck.deal_card().expect("deck is empty"));

        // Handle player actions:
        println!("Bet: {}\n", bet);

        loop {
            player_hand.display(true, "player");
            dealer_hand.display(false, "dealer");
            println!();

            // Check if the player has bust:
            if player_hand.value() > 21 {
                break;
            }

            // Get the player's move, either H, S, or D:
            let player_move = get_move(&player_hand, money - bet);

            // Handle the player actions:
            if player_move == "D" {
                // Player is doubling down, they can increase their bet:
                let additional_bet = get_bet(bet.min(money - bet));
                bet += additional_bet;
                println!("\nBet increased to {}", bet);
            }

            if ["H".to_string(), "D".to_string()].contains(&player_move) {
                // Hit/doubling down takes another card.
                let new_card = deck.deal_card().expect("deck is empty");
                println!(
                    "You drew a {} of {}.",
                    new_card.rank.display(),
                    new_card.suit.display()
                );
                player_hand.add_card(new_card);

                if player_hand.value() > 21 {
                    // The player has busted:
                    continue;
                }
            }

            if ["S".to_string(), "D".to_string()].contains(&player_move) {
                // Stand/doubling down stops the player's turn.
                break;
            }
        }

        // Handle the dealer's actions:
        if player_hand.value() <= 21 {
            while dealer_hand.value() < 17 {
                // The dealer hits:
                println!("\nDealer hits...");
                dealer_hand.add_card(deck.deal_card().expect("deck is empty"));
                player_hand.display(true, "player");
                dealer_hand.display(false, "dealer");

                if dealer_hand.value() > 21 {
                    // The dealer has busted.
                    break;
                }
                let mut input = String::new();
                print!("Press Enter to continue...");
                io::stdout().flush().ok();
                io::stdin().read_line(&mut input).unwrap();
            }
        }

        // Show the final hands:
        println!();
        player_hand.display(true, "player");
        dealer_hand.display(true, "dealer");

        let player_value = player_hand.value();
        let dealer_value = dealer_hand.value();

        // Handle whether the player won, lost, or tied:
        if dealer_value > 21 {
            println!("Dealer busts! You win ${}!", bet);
            money += bet;
        } else if player_value > 21 || player_value < dealer_value {
            println!("You lost!");
            money -= bet;
        } else if player_value > dealer_value {
            println!("You won ${}!", bet);
            money += bet;
        } else if player_value == dealer_value {
            println!("It's a tie, the bet is returned to you.");
        }
        let mut input = String::new();
        print!("Press Enter to continue...");
        io::stdout().flush().ok();
        io::stdin().read_line(&mut input).unwrap();
    }
}

fn get_move(player_hand: &Hand, money: u32) -> String {
    // Ask the player for their move, and returns "H" for hit, "S" for
    // stand, and "D" for double down.
    loop {
        // Determine what mvoes the player can make:
        let mut moves = vec!["(H)it".to_string(), "(S)tand".to_string()];

        // The player can double down on their first move, which we can
        // tell because they'll have exactly two cards:
        if player_hand.cards.len() == 2 && money > 0 {
            moves.push("(D)ouble down".to_string());
        }

        // Get the player's move:
        let mut player_move = String::new();
        let prompt = moves.join(", ");
        print!("{}\n> ", prompt);
        io::stdout().flush().ok();
        if io::stdin().read_line(&mut player_move).is_err() {
            println!("Failed to read input, please try again.");
        }
        player_move = player_move.trim().to_uppercase();

        if ["H".to_string(), "S".to_string()].contains(&player_move) {
            return player_move;
        }
        // Can only input "D" if player doubled down
        if moves.contains(&"(D)ouble down".to_string()) && player_move == "D" {
            return player_move;
        }
        if player_hand.cards.len() == 2 {
            println!("Invalid input, please only enter 'H', 'S' or 'D'");
        } else {
            println!("Invalid input, please only enter 'H' or 'S'");
        }
    }
}

fn get_bet(max_bet: u32) -> u32 {
    // Ask the player how much they want to bet for this round
    loop {
        print!("How much do you bet? (1-{}, or QUIT)\n> ", max_bet);
        io::stdout().flush().ok();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input, please try again.");
            continue;
        };

        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            print!("Thanks for playing!");
            process::exit(0);
        }

        match input.parse::<u32>() {
            Ok(bet) if bet != 0 && bet <= max_bet => return bet,
            Ok(_) => println!("\nPlease enter a valid number between 1 and {}", max_bet),
            Err(_) => println!("\nInvalid input. Enter a valid number or QUIT"),
        }
    }
}
struct Deck {
    deck: Vec<Card>,
}
impl Deck {
    fn new() -> Self {
        let mut deck = Vec::new();
        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs];
        let ranks = [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];

        for suit in suits.iter() {
            for rank in ranks.iter() {
                deck.push(Card {
                    rank: *rank,
                    suit: *suit,
                });
            }
        }
        deck.shuffle(&mut rand::rng());

        Self { deck }
    }

    fn deal_card(&mut self) -> Option<Card> {
        self.deck.pop()
    }
}

struct Hand {
    cards: Vec<Card>,
}
impl Hand {
    fn new() -> Self {
        Self { cards: Vec::new() }
    }
    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
    fn value(&self) -> u8 {
        // Return the value of the cards. Face cards are worth 10, aces are
        // worth 11 or 1 (this function picks the most suitable ace value).
        let mut value = 0;
        let mut ace_num = 0;

        // Add the value for the non-ace cards:
        for card in &self.cards {
            let rank = &card.rank;
            match rank {
                Rank::Ace => ace_num += 1,
                Rank::King | Rank::Queen | Rank::Jack => value += 10,
                Rank::Two => value += 2,
                Rank::Three => value += 3,
                Rank::Four => value += 4,
                Rank::Five => value += 5,
                Rank::Six => value += 6,
                Rank::Seven => value += 7,
                Rank::Eight => value += 8,
                Rank::Nine => value += 9,
                Rank::Ten => value += 10,
            }
        }

        value += ace_num;
        for _ in 0..ace_num {
            if value + 10 <= 21 {
                value += 10;
            }
        }
        value
    }

    fn display(&self, reveal_first: bool, holder: &str) {
        if !reveal_first {
            println!("{}: ???", holder.to_uppercase())
        } else {
            println!("{}: {}", holder.to_uppercase(), self.value())
        }

        let mut cards_display = [(); 5].map(|_| String::new());
        for (i, card) in self.cards.iter().enumerate() {
            cards_display[0] += " ___ ";
            if !reveal_first && i == 0 {
                // Print a card's back;
                cards_display[1] += "|## |";
                cards_display[2] += "|###|";
                cards_display[3] += "| ##|";
                cards_display[4] += " ‾‾‾ ";
            } else {
                // Print the card's front:
                let rank = card.rank.display();
                let suit = card.suit.display();

                let rank_left = format!("{:<2}", rank);
                let rank_right = format!("{:>2}", rank);

                cards_display[1] += &format!("|{} |", rank_left);
                cards_display[2] += &format!("| {} |", suit);
                cards_display[3] += &format!("| {}|", rank_right);
                cards_display[4] += " ‾‾‾ ";
            }
        }

        // Print each row on the screen
        for display in cards_display {
            println!("{}", display);
        }
    }
}

struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Clone, Copy)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    fn display(&self) -> String {
        match self {
            Suit::Hearts => "♥".to_string(),
            Suit::Diamonds => "♦".to_string(),
            Suit::Spades => "♠".to_string(),
            Suit::Clubs => "♣".to_string(),
        }
    }
}

#[derive(Clone, Copy)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
impl Rank {
    fn display(&self) -> String {
        match self {
            Rank::Ace => "A".to_string(),
            Rank::Two => "2".to_string(),
            Rank::Three => "3".to_string(),
            Rank::Four => "4".to_string(),
            Rank::Five => "5".to_string(),
            Rank::Six => "6".to_string(),
            Rank::Seven => "7".to_string(),
            Rank::Eight => "8".to_string(),
            Rank::Nine => "9".to_string(),
            Rank::Ten => "10".to_string(),
            Rank::Jack => "J".to_string(),
            Rank::Queen => "Q".to_string(),
            Rank::King => "K".to_string(),
        }
    }
}
