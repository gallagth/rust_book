fn main() {
    println!("main function");
    hello();
    let x = 7;
    let y = 35;
    println!("sum is {}", sum(x, y));
    expression();
    what_if();
    loopy();

    sing_a_song();
}

fn sing_a_song() {
    let gifts = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "12 Drummers Drumming",
    ];
    let mut given_gifts = Vec::new();
    for gift in gifts.iter() {
        given_gifts.push(gift);
        println!("##################################");
        println!("On the eleventh day of Christmas\nmy true love sent to me:");
        if given_gifts.len() == 1 {
            println!("{}", given_gifts[0]);
            continue;
        }
        for i in 0..given_gifts.len()-1 {
            println!("{}", given_gifts[i]);
        }
        println!("And {}", given_gifts[given_gifts.len()-1]);
        println!("##################################");
    }
}

fn hello() {
    println!("hello");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn expression() {
    let y = {
        let x = 41;
        x + 1
    };
    println!("y = {}", y);
}

fn what_if() {
    let n = 7;
    if n < 7 {
        println!("too small");
    } else if n == 7 {
        println!("wooo");
    } else {
        println!("too big");
    }

    let assigned = if true {
        42
    } else {
        -1
    };
    println!("assigned = {}", assigned);
}

fn loopy() {
    let mut val = 42;
    while val > 0 {
        println!("val = {}", { val = val - 1; val });
    }
}
