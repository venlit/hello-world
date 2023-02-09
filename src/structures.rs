struct person{
    alive: String,
    age: u32,
    first_name: String,
    last_name: String,
    online: String,
}
 
fn document_person(name: String, age: u32, alive: bool) -> person{
    let names: Vec<&str> = name.split(' ').collect();
    person{
        first_name: names[0].to_string(),
        last_name: names[1].to_string(),
        alive: if alive == true { "alive".to_string() }else { "dead".to_string() },
        age,
        online: names.join(""),
    }
}
 
pub fn run(){
    let tristan = document_person(String::from("Tristan Poirier"), 17, true);
    println!("status: {}", tristan.alive);
 
    let tristan_copy = person{
        last_name: "Poirier2".to_string(),
        ..tristan
    };
    println!("{} {}", tristan_copy.first_name, tristan_copy.last_name);
}

