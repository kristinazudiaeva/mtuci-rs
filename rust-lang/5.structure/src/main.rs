struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("vandegir"),
        email: String::from("vandegir@yandex.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.username);
    println!("{}", user1.email);

    main2();
}

//сокращённая инициализация поля
//fn build_user(email: String, username: String) -> User {
   // User {
        //active: true,
        //username,
        //email,
        //sign_in_count: 1,
    //}
//}

//fn main() {
    //let user1 = build_user(
        //String::from("someone@example.com"),
        //String::from("someusername123"),
    //);
//}

//Кортежные структуры
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main2() {
    let black = Color(6, 4, 9);
    let origin = Point(3, 2, 4);

    println!("{}, {}", black.0, origin.2);

    main3();
}

//пример
fn main3() {
    let width1 = 22;
    let height1 = 63;

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(width1, height1)
    );

    main4()
}

fn area3(width: u32, height: u32) -> u32 {
    width * height
}

fn main4() {
    let rect1 = (39, 12);

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );

    main5();
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//методы
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main5() {
    let rect1 = Rectangle {
        width: 56,
        height: 89,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}



