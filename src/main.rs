fn main() {
    
    question_1();
    question_2();
    question_3();
    question_4();
    question_5();
    question_6();
    question_7();
    question_8();
    question_9();
    question_10();
    question_11();
    question_12();
    question_13();

}

fn question_1() {
    let slogan = "PAKISTAN ZINDABAD";
    let length = slogan.len();

    println!("{}", slogan);
    println!("Length of PAKISTAN ZINDABAD is : {}\n", length);
}

fn question_2() {
    let number1: u64 = 85;
    let number2: i16 = -550;

    println!("Value 1 is : {}", number1);
    println!("Value 2 is : {}\n", number2);

}

fn question_3() {
    let number: f32 = 56.6;

    println!("Value is : {}\n", number);
}

fn question_4() {
    let x: i32 = 76;
    let y: i32 = 23;

    println!("x + y = {}", x + y);
    println!("x - y = {}", x - y);
    println!("x * y = {}", x * y);
    println!("x / y = {}", x / y);
    println!("x % y = {}\n", x % y);

}

fn question_5() {
    let array: [i32; 5] = [100, 150, 200, 250, 300];

    println!("{:#?}", array);
    println!("\n");
    println!("{}", array[1]);
    println!("{}", array[3]);
    println!("\n");
}

fn question_6() {
    let tup = ("IoT", "AI", "Cloud", 500.65, 8645, 65.4);
    let (a, b, c, d, e, f) = tup;

    println!("{:#?}", tup);
    println!("{}", c);
    println!("{}", e);
    println!("{}", f);
    println!("\n");
}

fn question_7() {
    println!("The sum is : {}\n", add(10, 20, 30));    

}

// function pertains to Question # 7

fn add(number1: i32, number2: i32, number3: i32) -> i32 {
    number1 + number2 + number3
}

fn question_8() {
    println!("Product is : {}\n", multiply(5.6, 2.4, 10.2));
}

// function pertains to Question # 8

fn multiply(number1: f32, number2: f32, number3: f32) -> f32 {
    number1 * number2 * number3
}

fn question_9() {
    let marks = 95;

    if marks > 80 {
        println!("Grade A+\n");
    } else if marks >= 70 && marks <= 80 {
        println!("Grade A\n");
    } else if marks >=60 && marks < 70 {
        println!("Grade B\n");
    } else if marks >=50 && marks < 60 {
        println!("Grade C\n");
    } else if marks >=40 && marks < 50 {
        println!("Grade D\n");
    } else {
        println!("Grade F\n");
    }
}

fn question_10() {
    let year = 2019;

    if year % 4 == 0 {
        println!("Year {} is a leap year\n", year);
    } else {
        println!("Year {} is not a leap year\n", year);
    }
}

fn question_11() {
    
    let mut counter = 0; 

    for even in 0..6 {
        counter = counter + 2;
        println!("{}", counter);
    }
println!("\n");
}

fn question_12() {
    
    let mut counter = 1; 

    println!("{}", counter);
    for odd in 1..6 {
        counter = counter + 2;
        println!("{}", counter);
    }
println!("\n");
}

fn question_13() {
    let table = 10;
    let mut multiple = 1;

    for number in 1..10 {
        let product = table * multiple;
        println!("{} x {} = {}", table, multiple, product);
        multiple = multiple + 1;
    }
    println!("\n");
}
