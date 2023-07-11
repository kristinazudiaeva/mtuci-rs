fn main() {  
  //new, push
    let mut v = Vec::new();

    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);

    println!("Вектора: {v:?}\n");

  //pop
    let x = v.pop();
    match x {
        Some(value) => println!("Удаление вектора: {value}."),
        None => println!(" None  "),
    }
    println!("{v:?}\n");

    //remove
    let y = v.remove(1);
    println!("Удалие второго вектора: {y}.");

    println!("{v:?}\n");

    //get
    let second: Option<&i32> = v.get(0);
    match second {
        Some(value) => println!("Первый вектор: {value}.\n"),
        None => println!(" None  "),
    }

    //resize
    let mut v = vec![1, 30, 50, 80, 100];

    println!("Вектора: {v:?}");

    v.resize(5, 10);
    println!("Увеличение: {v:?}");

    v.resize(2, 0);
    println!("Уменьшение: {v:?}\n");

    //with_capacity
    let mut v = Vec::with_capacity(2);

    println!("Ёмкость: {}", v.capacity());

    v.push(5);
    v.push(10);

    println!("Вектора: {:?}, Размер: {}.", v, v.capacity());

    v.push(15);

    println!("Вектора: {:?}, Размер: {}.", v, v.capacity());

    v.push(20);
    v.push(25);
    v.push(30);

    println!("Вектора: {:?}, Размер: {}.", v, v.capacity());
}
