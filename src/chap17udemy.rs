pub fn owner_ship(){
    let a : u32 = 89;
    let mut b = a;
    println!("{},{}", a , b);
    b = 45;
    println!("{},{}", a , b);
}

pub fn clonage(){
    // resolution du problème de pointeur
    let c = "Hello".to_owned();
    let mut d = c.clone();
    d.push_str("world");
    println!("{} - {}", c , d);
}

pub fn scop() {
    let a = "test".to_owned();
    {
        // b a été construit
        let b = a;
        println!("{}",b)
    }
    // b a été détruit le pointeur ne pointe plus rien
}
