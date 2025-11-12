use std::{
    collections::HashSet,
    io::{self, Write},
};

use chrono::NaiveDate;
use rand::Rng;

fn main() {
    println!(
        "Birthday Paradox

The Birthday Paradox shows us that in a group of N people, the odds
that two of them have matching birthday is surprisingly large.
This program does a Monte Carlo simulation (that is, repeated random
simulations) to explore this concept.

(It's not actually a paradox, it's just a surprising result.)"
    );

    println!();

    let mut num_birthdays = String::new();
    loop {
        println!("How many birthdays shall I generate? (Max 100)");
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut num_birthdays)
            .expect("Failed to read line!");
        let num_birthdays_i32 = num_birthdays.trim().parse::<i32>();
        if num_birthdays_i32.is_ok() && num_birthdays_i32.unwrap() <= 100 {
            break;
        } else {
            println!("Invalid input!\n");
            num_birthdays.clear();
        }
    }
    let num_birthdays: i32 = num_birthdays.trim().parse().unwrap();

    let random_birthdays = paradox_simulation(num_birthdays);
    println!("Here are {} birthdays:", num_birthdays);
    for (i, day) in random_birthdays.iter().enumerate() {
        match i {
            0 => {}
            n if n % 10 == 0 => println!(),
            _ => print!(", "),
        }
        print!("{}", day.format("%b %d"));
    }

    println!("\n");
    print!("In this simulation, ");
    let dup_birthday = check_dup_birthdays(random_birthdays);
    if dup_birthday.is_some() {
        println!(
            "multiple people have a birthday on {}.",
            dup_birthday.unwrap().format("%b %d")
        )
    } else {
        println!("there are no matching birthdays.")
    }

    const SIMU_NUMS: i32 = 100_000;
    // Run through SIMU_NUMS simulations
    println!(
        "\nGenerating {} random birthdays {} times...",
        num_birthdays, SIMU_NUMS
    );
    println!("Press Enter to begin...");
    io::stdout().flush().unwrap();

    _ = io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line.");
    println!("Let's rung another {} simulations.", SIMU_NUMS);

    let mut simu_done = 0;
    let mut counter = 0;
    while simu_done < SIMU_NUMS {
        if simu_done % (SIMU_NUMS / 10) == 0 {
            println!("{} simulations run...", simu_done);
        }
        if check_dup_birthdays(paradox_simulation(num_birthdays)).is_some() {
            counter += 1;
        }
        simu_done += 1;
    }
    let probability = counter as f32 * 100.0 / SIMU_NUMS as f32;
    println!(
        "Out of {} simulations of {} people, there was a",
        SIMU_NUMS, num_birthdays
    );
    println!(
        "matching birthday in that group {} times. This means",
        counter
    );
    println!(
        "that {} people have a {:.2}% chance of",
        num_birthdays, probability
    );
    println!("having a matching birthday in their group.");
    println!("That's probably more than you would think!");
}

fn check_dup_birthdays(random_birthdays: Vec<NaiveDate>) -> Option<NaiveDate> {
    let mut dup_birthdays = HashSet::new();
    random_birthdays
        .into_iter()
        .find(|&day| !dup_birthdays.insert(day))
}

fn paradox_simulation(num_birthdays: i32) -> Vec<NaiveDate> {
    let mut random_birthdays = Vec::new();
    let mut rng = rand::rng();

    while random_birthdays.len() < num_birthdays.try_into().unwrap() {
        let day = NaiveDate::from_yo_opt(2001, rng.random_range(1..=365)).unwrap();

        random_birthdays.push(day);
    }
    random_birthdays
}
