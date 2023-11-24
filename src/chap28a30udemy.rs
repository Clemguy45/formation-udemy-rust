pub fn bowrowing(){
    // Initialisation de la variable
    let a : String = String::from("Test");
    //Début du scope
    {
        let b = &a; // avec & on peut rendre la prepriété à a après le scope
        println!("{}", b);
    }
    // Fin de scope b est détruit
    println!("{}", a);
}

pub fn pointer_mutable(){
    let mut a = String::from("Hello");
    {
        let b = &mut a; // on ne peut avoir que un pointeur mutable sur a
        // on ne peut pas faire de pointeur immutable après un pointeur mutable
        b.push_str(" world"); // cela modifie a en même temps
        println!("{}", b);
    }
    //mais peut etre créer après le scope (donc après destruction de b)
    println!("{}",a);
}