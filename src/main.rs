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



}
