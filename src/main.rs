mod mon_module;

use crate::mon_module::chap17udemy::{clonage, owner_ship, scop};
use crate::mon_module::chap22a24udemy::{conditionnel, get_result_conditionnel};
use crate::mon_module::chap25a27udemy::{binding, match_binding, matching};
use crate::mon_module::chap28a30udemy::{bowrowing, pointer_mutable};
use crate::mon_module::chap31a36udemy::{arrays, hashmap, slice, tuplet, vector};
use crate::mon_module::chap37a39udemy::{forloop, labelloop, whileloop};
use crate::mon_module::chap40a45udemy::{addition, modification, say, say2, say3};

extern crate rand;
use rand::prelude::*;
use crate::mon_module::choix::{choisir, Choix};
use crate::mon_module::personne::Personne;
use std::thread;
use std::time::Duration;
use std::sync::*;

fn main() {
    //Ma première variable. Udemy Chap 8.
    let ma_variable = 12;
    println!("Ma variabe : {}", ma_variable);

    //Les différent type. Udemy Chap 9.
    // les différent entier : u8, u16, u32 et u128 (u signifie unsigned donc entier non signés)
    let entier : u8 = 0;
    println!("mon entier u8 : {}", entier);
    // les différent entier signés : i8, i16, i32 et i128 (i signifie integer donc entier signés)
    let entier_signes : i8 = -3;
    println!("mon entier signer i8 : {}", entier_signes);
    // type particulier le usize et le isize (c'est le compilateur qui définis la taille)
    // sur un systme 32 bit le isize et le usize sera a u32 et a i32
    // et pour les system 64 bit on sera en u64 et en i64
    let entier_usize : usize = 2;
    let entier_usize2 = 4;
    println!("mes entier usize : {}, {}", entier_usize, entier_usize2);
    // on a aussi le type float : f32, f64
    let float : f32 = 4.0;
    println!("mon float f32 : {}", float);
    println!("Bravo vous avez completer le chap 9");

    // Autres notation. Udemy Chap 10.
    // pour plus de lisibiliter
    let not = 150_000_000;
    println!("Entier lisible : {}", not);

    // notation hexadecimal
    let hexa = 0xff;
    println!("mon hexadecimal : {}", hexa);

    //notation binaire
    let binaire = 0b1011001;
    println!("mon binaire : {}", binaire);

    //notation supplémentaire des entier :
    let notation = 23u8;
    println!("autre notation : {}", notation);
    println!("Bravo vous avez complété le chap 10");

    //la Mutbilité. Udemy Chap 11
    let mut a = 23;
    println!("première variable mutable avant mutation : {}", a);
    a = 48;
    println!("Ma première variable mutable après mutation : {}", a);
    println!("Bravo vous avez complété le chap 11");

    // Convertir une valeur, le casting. Udemy Chap 12
    let b : u16 = 23u8 as u16;
    println!("{}", b);
    // Attention le cast marche qu'avec les type compatible.
    // Il n'arrondit pas mais il tronc
    println!("Bravo vous avez complété le chap 12");

    // Les constantes. Udemy Chap 13
    let c = 89; // la variable prends de la place en mémoire a compilation
    const MA_CONSTANTE : f32 = 3.14; // code en dure ça valeur
    // a la compilation la constante n'existe plus en on finis avec println!({}-{},c,3.14)
    println!("{}-{}",c, MA_CONSTANTE);
    println!("Bravo vous avez complété le chap 13");

    //Autre type et arithmétque de base. Udemy Chap 14
    let d = 23f32+25.34;
    println!("La somme de 23 + 25.34 = {}", d);
    let boolean : bool = true;
    // peut etre false.
    let charactere : char = 'a';
    // encodé sur 32 bit pour prendre des accétuation de les emojis
    println!("Boolean : {}", boolean);
    println!("character : {}", charactere);
    println!("Bravo vous avez complété le chap 14");

    //Les chaine de character. Udemy Chap 15
    let s = "Ma chaine de caractère"; // Immutable
    let mut s2 : String  = String::from("Une autre chaine de caractères"); // Mutable
    s2.push_str(" plus long");
    println!("{} \n{}",s,s2);
    println!("Bravo vous avez complété le chap 15");

    //Le shadowing. Udemy Chap 16.
    let i : u16 = 256;
    let i = i.to_string(); //le changement a été effectuer et a écrasée la précédente
    println!("{}",i);
    println!("Bravo vous avez complété le chap 16");

    //Ownership. Udemy chap 17 à 21
    owner_ship();
    clonage();
    scop();

    //Structures conditionnelles. Udemy Chap 22 à 24.
    conditionnel();
    get_result_conditionnel();

    //Pattern Matching. Udemy Chap 25 à 27
    matching();
    binding();
    match_binding();

    //Les référence. Udemy Chap 28 à 30
    bowrowing();
    pointer_mutable();

    //Les collections. Udemy Chap 31 à 36
    arrays();
    slice();
    tuplet();
    vector();
    hashmap();

    // Les boucles. Udemy Chap 37 à 39
    whileloop();
    forloop();
    labelloop();

    //les fonctions. Udemy Chap 40 à 45
    println!("{}", addition(4,5));
    let x = String::from("hello");
    say(x); // on peut pas doubler la fonction car on a le problem d'ownership
    // solution 1
    let truc = String::from("Hello world");
    let truc1 = say2(truc);
    let truc2 = say2(truc1);
    println!("{}",truc2);
    // solution 3 : Borrowing
    let y = String::from("Test3");
    say3(&y);
    say3(&y);
    let mut n = 5;
    modification(&mut n);
    println!("{}",n);

    //Modularité. Udemy Chap 46 et 47
    println!("{}", random::<i8>());

    //Les structures. Udemy Chap 48 à 50
    let pers : Personne = Personne::new("didier".to_owned(),58);
    println!("{:#?}", pers);
    pers.hello();
    let choice = Choix::Contre("pas d'accords".to_owned());
    choisir(choice);

    //Crash et Gestion des erreurs. 54 à 56
    //panic!("le programme a crash");
    //Comme Option on a un enum de Erreur qui se nome Result
    //sinon il y a unwrap() pour un code plus claire
    //pour l'option il faut utiliser except(str) pour des message plus personnalisé

    //Les Thread. Udemy Chap 70 à 74
    let (sender, receiver) = mpsc::channel();
    let sender2 = mpsc::Sender::clone(&sender);
    // est soumis a l'Ownership
    thread::spawn(move || {
        for i in 1..10 {
            sender.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        for i in 100..110 {
            sender2.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });


    for recu in receiver {
        println!("{}", recu);
    }

    let mut cpt = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..1000 {
        let c = Arc::clone(&cpt);
        let handler = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num +=1 ;
        });
        handles.push(handler);
    }

    for handle in handles {
        handle.join();
    }
    println!("{}", *cpt.lock().unwrap());
    println!("fin de programme")
}
