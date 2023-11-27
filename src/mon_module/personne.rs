#[derive(Debug)]
pub struct Personne {
    nom : String,
    age : u8,
}

impl Personne {
    //Méthode associer (constructeur)
    pub fn new(n: String, a: u8)-> Personne {
        Personne {nom : n, age : a }
    }
    //Méthode d'instance
    pub fn hello(&self){
        println!("Hello je suis {} et j'ai {} ans", self.nom, self.age);
    }
}