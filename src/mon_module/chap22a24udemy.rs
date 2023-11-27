pub fn conditionnel(){
    let a = 18;
    if a >= 18 {
        println!("Majeur");
    } else if a > 0{
        println!("Mineur");
    } else {
        println!("erreur");
    }
}

pub fn get_result_conditionnel(){
    let a = 18;
    let result = if a >= 18 {
        "Majeur"
    } else if a > 0{
        "Mineur"
    } else {
        "erreur"
    };
    println!("{}", result);
}

// la structure conditionnel a ces limite on ne peut pas faire
// if a pour voir si la variable a exsiste car la valeur null n'existe pas dans Rust