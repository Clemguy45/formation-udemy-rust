pub fn matching(){
    let a  = 24;

    match a {
        0 => println!("zéro"),
        _ if a < 0 => println!("erreur"),
        1..=17 => println!("mineur"),
        19 | 24 | 32 => println!("ok"),
        _ => println!("majeur"),
    }
}

pub fn binding(){
    let (a,_, b) = (56,12, 125);
    println!("{} - {}", a, b)
}

pub fn match_binding(){
    let a = ("div", 50, 3);
    // on ne peut pas modifier la structure du match pour l'entrée et la sortie
    let (texte, valeur) = match a {
        (op,x,y) if op == "div" => ("Division",x/y),
        (op,x,y) if op == "add" => ("Addition", x+y),
        (op,x,_) if op == "pow" => ("Carrée", x*x),
        _ => ("impossible a calculer", 0),
    };
    // peut renvoyer un resultat en la stockant dans une variable
    println!("Opération : {} - Resultat : {}", texte, valeur);
}