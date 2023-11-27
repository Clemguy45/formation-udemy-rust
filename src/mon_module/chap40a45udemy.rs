pub fn addition (x: i32, y: i32) -> i32{
    x+y
}

pub fn say (truc: String) {
    println!("{}", truc);
}

pub fn say2(truc: String) -> String {
    println!("{}", truc);
    truc
}

pub fn say3(truc: &String) {
    println!("{}", truc)
}

pub fn modification(arg: &mut i32){
    *arg *= 10;
}

//a ce stade il est impossible de faire cette fonction
/*pub fn retour() -> &u8{
    let a : &u8 = &10;
    &a
}*/