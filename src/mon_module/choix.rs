pub enum Choix {
    Pour,
    Contre(String),
}

pub fn choisir(choix: Choix){
    match choix {
        Choix::Pour => println!("Plutôt pour"),
        Choix::Contre(x) => println!("Plutôt contre parce que {}",x),
    }
}