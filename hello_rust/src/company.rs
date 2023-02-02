use std::collections::HashMap;


fn add(map: &mut HashMap<String, Vec<String>>, name: String, department: String){
    match map.get_mut(&department){
        Some(person) => {
            person.push(name);
        }
        None => {
            map.insert(department, vec![name]);
        }
    }
}


fn remove(map: &mut HashMap<String, Vec<String>>, thing: String){
    if map.contains_key(&thing) { map.remove(&thing); }
    for (&_, &i) in map.iter(){
        let index = i
            .iter()
            .position(|x| *x == &thing)
            .unwrap();
        i.remove(index);
    }


}


pub fn run(){
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    company.insert(String::from("Engineering"), Vec::new());
    company.insert(String::from("Resources"), Vec::new());
    company.insert(String::from("IT"), Vec::new());
    company.insert(String::from("Sales"), Vec::new());


    let employee = "Johnathan".to_string();
    let department = "IT".to_string();


    add(&mut company, employee, department);
   
    for i in &company{
        println!("{:?}", i);
    }

    remove(&mut company, "Johnathan".to_string());


    for i in &company{
        println!("{:?}", i);
    }
}
