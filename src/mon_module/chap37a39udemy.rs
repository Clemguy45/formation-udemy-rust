pub fn whileloop(){
    let mut a = 0;
    // loop = while true {}
    let result = loop {
        a += 1;
        if a >= 10 {
            break a; //on peut assigner une variable en mem temps que le break
        }
    };
    println!("{}", result);
}

pub fn forloop(){
    let a = [10,20,30,40,50,60,70];
    // parcours de tableau simple
    for elem in a.iter(){
        println!("{}", elem);
    }
    // parcours du tableau avec un tuplet
    for (index, elem) in a.iter().enumerate(){
        println!("{} - {}", index,elem);
    }
    //parcours avec range
    for b in 1..10 {
        println!("{}",b);
    }
}

pub fn labelloop(){
    //label boucle 'boucle.
    'boucle : for a in 1..10 {
        for b in 1..10 {
            if b == 3 { break 'boucle; }
            println!("{}-{}", a,b);
        }
    }
}