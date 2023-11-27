use std::collections::HashMap;
pub fn arrays(){
    //Création
    let mut a  = [[1,2],[3,4]];
    //modification
    a[0][0] *= 10;
    //Affichage
    println!("{:?}",a);
    println!("{}", a.len());
}

pub fn slice(){
    // on pointe une partie du tableaux
    let mut a = [12,35,64,78,126];
    let b = &mut a[0..3];
    b[0] = 1034;
    println!("{:?}", b);
    println!("{:?}",a);
    //on ne peut pas modifier a avant b a cause de la règle de propriété
}

pub fn tuplet(){
    let t1 = (1u8,23.4,26,"test");
    let t2 : (u8, i32, (usize, f32)) = (12,45,(89,20.0));
    println!("le deuxième élément de t1 : {} ", t1.1);
    println!("le deuxième élément du tuplet dans le tuplet : {}", (t2.2).1);
}

pub fn vector(){
    let mut vec = vec![12,45,78,45];
    vec.push(125);
    println!("{:?}", vec.get(1));
    // Si on utilise la méthode get on renvoie none si il existe pas
    // d'erreur lever
    match vec.get(23) {
        Some(x) => println!("{}", x),
        None => println!("pas de valeur disponible"),
    }
    let mut vecempty = Vec::new();
    vecempty.push("debout");
    println!("{:?}",vecempty);
}

pub fn hashmap(){
    let mut a = HashMap::new();
    a.insert(1,"voleur");
    a.insert(2, "magicien");
    println!("{:?}",a);
    println!("{:?}", a.get(&2));
}