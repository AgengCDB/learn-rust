struct Student {
    name: String,
    level: u8,
    remote: bool,
}

struct Grades(char, char, char, f32);

fn main() {
    println!("Hello, {}!", "Will");

    let mut age = 33; // mut means mutable
    let birth_year = 1991;

    println!("I am {} years old", age);

    let birth_year = birth_year - 1;
    
    age = 34;

    println!("I am now {} years old", age);
    println!("I was born in {}", birth_year);

    let nephew_age: u32 = 42;
    println!("My nephew is {} years old", nephew_age);

    let float: f32 = 4.0;
    println!("{}", float);

    println!("1 + 2 = {}", 1+2);

    let is_bigger_number = 2 < 4;
    println!("Is 2 < 4? {}", is_bigger_number);

    let first_char: char = 'M';
    let last_char: char = 'g';
    let second_char: char = 'e';
    let my_name = "Meong";

    println!("{} is the first char, {} is the last char, {} is the second char of my name {}", first_char, last_char, second_char, my_name);

    let my_dog = ("Toby", 15, false);

    println!("My dog's name was {}, he was {} years old, is he alive? {}", my_dog.0, my_dog.1, my_dog.2);

    let student_1 = Student{
        name: String::from("Meong Cat"),
        remote: true,
        level: 5
    };

    let grades = Grades('A', 'A', 'B', 3.5);

    println!("{}, is a level {} programmer. Does he work remotely: {}",
        student_1.name, student_1.level, student_1.remote);
    println!("{}, {}, {}, GPA = {}", grades.0, grades.1, grades.2, grades.3);
    
}