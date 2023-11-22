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
    println!("Bravo vous avez complété le chap 14");

}
