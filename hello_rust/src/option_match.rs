#[derive(Debug)]

enum state{
    alabama,
    alaska,
    arizona
}
enum coin{
    nickel,
    dime,
    penny,
    quarter(state)
}

fn cent_value(coin: coin) -> u8{
    match coin{
        coin::penny => 1,
        coin::dime => 10,
        coin::quarter(state) => {
            // println!("{:?}", state);
            25
        }
        coin::nickel => 5
    }
}

fn option_add(x: Option<u8>, y: u8) -> Option<u8>{
    match x{
        None => Some(y),
        Some(i) => Some(i+y)
    }
}

pub fn run(){
    // let some_number = some(10);
    // let some_char = some('c');
    
    // let nothing: option<i32> = none;

    let george = coin::quarter(state::alabama);
    let tiny = coin::dime;

    println!("Some(10) + 20 = {:?}", option_add(Some(10), 20));

}