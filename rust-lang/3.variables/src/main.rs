fn main() {
    //затемнение
    let x = 3;
    let x = x + 8;

    {
        let x = x * 4;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //кортеж
    let tup = (1, 901, 7.8);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    //массив и доступ
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{first}, {second}");

    //операторы и выражения
    let y = {
        let x = 8;
        x + 3
    };

    println!("The value of y is: {y}");

    //if
    let number = 7;

    if number < 2 { 
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    //loop + break
    let mut counter = 0;

    let result = loop {
        counter += 2;

        if counter == 12 {
            break counter * 3;
        }
    };
    println!("The result is {result}");

    //while
    let mut number = 5;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("Марш!");
}
