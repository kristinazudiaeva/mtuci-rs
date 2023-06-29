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
}
