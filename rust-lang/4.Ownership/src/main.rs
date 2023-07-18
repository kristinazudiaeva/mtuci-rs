fn main() {
    //типы данных String
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    //clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    //копирование
    let x = 222;
    let y = x;

    println!("x = {}, y = {}", x, y);

    //ссылки
    let mut s = String::from("rust");
    let r1 = &s;

    let mut s1 = String::from("ссылки");
    let r2 = &s1;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    //массив
    let a = [5, 12, 56, 23, 78];
    let first = a[3];
    
    println!("Массив: {first}")

}

