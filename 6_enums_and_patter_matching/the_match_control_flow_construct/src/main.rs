fn plus_one_or_not(passed: Option<i32>) -> Option<i32> {
    match passed {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("-------------");
    println!("Basic matches");
    println!("-------------");
    let penny = Coin::Penny;
    println!("Value of penny is {} cent", value_in_cents(penny));
    let nick = Coin::Nickel;
    println!("Value of nickel is {} cents", value_in_cents(nick));
    let quarter = Coin::Quarter(UsState::Alaska);
    println!("Value of quarter is {} cents", value_in_cents(quarter));
    {
        println!("----------------------");
        println!("Now using Option match");
        println!("----------------------");
        let five: Option<i32> = Some(5);
        let six = plus_one_or_not(five);
        // arms of match need to handle all posible values
        match six {
            None => {
                println!("No value");
            }
            Some(i) => {
                println!("Value is {i}");
            }
        }
    }
    println!("---------------------");
    println!("And Catch-All Pattern");
    println!("---------------------");
    let dice = 3;
    match dice {
        // calling functions
        3 => fancy(),
        7 => {
            println!("You lost your hat");
        }
        // other is just a var name
        other => {
            println!("The value was diff than 3 or 7 -> {other}");
        }

    }
    match dice {
        1 => {
            println!("Rolled {dice}");
        }
        _ => {
            println!("Unused value taking care of anything other than 1");
        }
    }
}

fn fancy() {
    println!("You have a fancy hat");
}

/*
 * When using match compiler makes sure every value is handled
 * checks if any of the patterns are compatible, switch like
 * example below
*/
enum Coin {
    Penny,
    Nickel,
    _Dime,
    // adding() to hold value
    Quarter(UsState),
 }

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // pattern => value / code -> val
        Coin::Penny => 1,
        Coin::Nickel => {
            println!("EXTRA TEXT FOR NICKEL");
            5
        }
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from state {state:?}");
            25
        },
    }
}

#[derive(Debug)]
enum UsState {
    _Alabama,
    Alaska,
}

