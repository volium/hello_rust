fn main() {
    // let mut x: u8 = 25.35;
    // println!("x is {}", x);
    // x += 1;
    // println!("x is {}", x);

    // let x: f32 = 10.263637347384746346373737356225253737;
    // println!("x is {}", x);

    // let a = 10.0;
    // let b = 3.0;
    // let c = a / b;
    // println!("c is {0:08.3}\na is {1}\nonce again, c is {1}", c, a);

    // let mut value = 0b1111_0101u8;
    // println!("value is {}", value);
    // println!("value is {:08b}", value);

    // value = !value;
    // println!("value is {:08b}", value);

    // value = value & 0b0000_0111;
    // println!("value is {:08b}", value);
    // println!("bit 2 is {}", (value & 0b0000_0010) != 0);
    // println!("bit 6 is {}", (value & 0b0100_0000) != 0);

    // value = value | 0b0100_0000;
    // println!("value is {:08b}", value);
    // println!("bit 6 is {}", (value & 0b0100_0000) != 0);

    // value = value ^ 0b0101_0101;
    // println!("value is {:08b}", value);

    // value = value << 4;
    // println!("value is {:08b}", value);

    // value = value >> 2;
    // println!("value is {:08b}", value);

    // let a = true;
    // let b = false;

    // println!("a is {} and b is {}", a, b);
    // println!("NOT a is {}", !a);
    // println!("a AND b is {}", a & b);
    // println!("a OR b is {}", a | b);
    // println!("a XOR b is {}", a ^ b);

    // let c = (a ^ b) && panic!();
    // println!("c is {}", c);

    // let a = 1;
    // let b = false;

    // println!("a is {} and b is {}", a, b);
    // println!("a EQUAL TO b  is {}", a == b);
    // println!("a NOT EQUAL TO b  is {}", a != b);
    // println!("a GREATER THAN b  is {}", a > b);
    // println!("a GREATER THAN OR EQUAL TO b  is {}", a >= b);
    // println!("a LESS THAN b  is {}", a < b);
    // println!("a LESS THAN OR EQUAL TO b  is {}", a <= b);


    // let letter = 'a';
    // let number = '1';
    // let finger = '\u{261D}';
    // println!("{}\n{}\n{}", letter, number, finger);

    // let a = 13;
    // let b = 2.3;
    // let c: f32 = 120.0;

    // let average = (a as f64 + b as f64 + c as f64) / 3.0;

    // assert_eq!(average, 45.1);
    // println!("Test passed");


    // let mut letters = ['a', 'b', 'c'];
    // letters[0] = 'x';
    // let first_letter = letters[0];
    // println!("first_letter is {}", first_letter);


    // let numbers: [i32; 5];
    // numbers = [0; 5];
    // let index: usize = numbers.len();
    // println!("last number is {}", numbers[index]);


    // let parking_lot = [[1,2,3],
    //                    [4,5,6]];

    // let number = parking_lot[1][2];
    // println!("number is {}", number);

    // let garage =  [[[0; 100]; 20]; 5];


    // let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    // stuff.0 += 3;
    // let first_item = stuff.0;
    // println!("first_item is {}", first_item);

    // let (a, b, c) = stuff;
    // println!("b is {}", b)

    // say_hello();
    // say_hello();
    // let x = 1;
    // let y = 2;
    // say_the_sum(x, y);
    // say_a_number(x as i32);

    // let result = square(13);
    // println!("result is {:?}", result;

    // let celsius_temp = 23.0;
    // let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    // assert_eq!(fahrenheit_temp, 73.4);
    // println!("Test passed");

    // let x = 4;

    // if x + 1 != 3 {
    //     println!("x + 1 is NOT 3!");
    // }


    // let x = 3;
    // let y = 5;

    // if x > y {
    //     println!("x is greater than y");
    // } else if x < y {
    //     println!("x is less than y");
    // } else {
    //     println!("x is equal to y");
    // }


    // let make_x_odd = true;
    // let x = if make_x_odd {1} else {2.0};

    // // if make_x_odd {
    // //     x = 1;
    // // } else {
    // //     // x = 2;
    // // }

    // println!("x is {}", x)

    // let mut count = 0;

    // let result = loop {
    //     if count == 10 {
    //         break count * 10;
    //     }
    //     count += 1;
    //     println!("count is {}", count);
    // };`

    // println!("After the loop!");
    // println!("result is {}", result);

    // let mut count = 0;
    // let letters = ['a', 'b', 'c'];

    // while count < letters.len() {
    //     println!("letter is {}", letters[count]);
    //     count += 1;
    // }

    // let message = ['h', 'e', 'l', 'l', 'o'];

    // for (index, &item) in message.iter().enumerate() {
    //     println!("item {} is {}", index, item);
    //     if item == 'e' {
    //         break;
    //     }
    // }


    // for number in 0..5 {
    //     println!("number is {}", number)
    // }

    // let mut matrix = [[1, 2, 3],
    //               [4, 5, 6],
    //               [7, 8, 9]];

    // for row in matrix.iter_mut() {
    //     for number in row.iter_mut() {
    //         *number += 10;
    //         print!("{}\t", number)
    //     }
    //     println!()
    // }


    // let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    // let mut max: i32;
    // let mut min: i32;
    // let mean: f64;

    // max = <i32>::min_value();
    // min = <i32>::max_value();
    // let mut sum = 0.0;

    // for i in numbers {
    //     if i > max {
    //         max = i;
    //     }
    //     if i < min {
    //         min = i;
    //     }
    //     sum += i as f64;
    // }
    // mean = sum / numbers.len() as f64;

    // assert_eq!(max, 56);
    // assert_eq!(min, -18);
    // assert_eq!(mean, 12.5);

    // println!("Test passed");

    // let _array: [i32; <u32>::max_value() as usize] = [0; <u32>::max_value() as usize];
    // println!("array length is {}", _array.len());

    // let mut message = String::from("Earth");
    // println!("message is {}", message);
    // message.push_str(" is home.");
    // println!("message is {}", message);

    // let outer_planet: i32;
    // {
    //     let mut inner_planet = 1; //String::from("Mercury");
    //     // outer_planet = inner_planet.clone();
    //     outer_planet = inner_planet;
    //     // inner_planet.clear();
    //     inner_planet += 1;
    //     println!("inner_planet is {}", inner_planet);
    // }
    // println!("outer_planet is {}", outer_planet);

    // let mut rocket_fuel = String::from("RP-1");
    // let length = process_fuel(&mut rocket_fuel);
    // println!("rocket_fuel is {} and length is {}", rocket_fuel, length);

    // let rocket_fuel = produce_fuel();
    // println!("rocket fuel is {}", rocket_fuel);

    // let message = String::from("Greetings from Earth!");
    // println!("message is {}", message);

    // let last_word = &message[15..];
    // println!("last_word is {}", last_word);

    // let planets = [1, 2, 3, 4, 5, 6, 7, 8]; // sorry, Pluto!
    // let inner_planets: &[i32] = &planets[..4];
    // println!("inner_planets are {:?}", inner_planets);

    // let message = String::from("Greetings from Earth!");
    // let first_word = get_first_word(&message);
    // println!("first_word is {}", first_word);

    // let test1 = "We need more space.";
    // assert_eq!(trim_spaces(test1), "We need more space.");

    // let test2 = String::from("   There's space in front.");
    // assert_eq!(trim_spaces(&test2), "There's space in front.");

    // let test3 = String::from("There's space to the rear. ");
    // assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    // let test4 = "  We're surrounded by space!    ";
    // assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    // let test5 = "     ";
    // assert_eq!(trim_spaces(test5), "");

    // let test6 = "";
    // assert_eq!(trim_spaces(test6), "");

    // let test7 = " 🚀 ";
    // assert_eq!(trim_spaces(test7), "🚀");
    // println!("Tests passed!");


    // use std::io;

    // let mut buffer = String::new();
    // println!("Enter a message:");
    // io::stdin().read_line(&mut buffer);
    // println!("buffer is {}", buffer);

    // let number: i32 = buffer.trim().parse().unwrap();
    // println!("number + 1 is {}", number + 1);

    use rand::prelude::*;

    let number = random::<f64>();
    println!("number is {}", number);

    let number = thread_rng().gen_range(1..11);
    println!("number is {}", number);

}

// fn trim_spaces(s: &str) -> &str {
//     // println!("----------------------");
//     // println!("{}", s);
//     let bytes = s.as_bytes();

//     let mut first = -1;
//     let mut last = -1;

//     for (index, &letter) in bytes.iter().enumerate(){
//         // println!("current index is {} and letter is {}", index, letter as char);
//         if letter != b' ' && first == -1 {
//             first = index as i32;
//             last = index as i32;
//             // println!("Updating first and last to {}", index);
//         }
//         else if letter != b' ' && last != -1 {
//             // println!("current letter is {}", letter as char);
//             last = index as i32;
//             // println!("Updating last to {}", last);
//         }
//     }

//     if first == last {
//         return &s[0..0]
//     }


//     // println!("first is {} and last is {}", first, last);
//     return &s[first as usize .. (last + 1) as usize];
// }

// fn get_first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (index, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..index]; // found a space!
//         }
//     }

//     &s // no space found; input is a single word
// }

// fn produce_fuel() -> String {
//     let new_fuel = String::from("RP-1");
//     new_fuel
// }

// fn process_fuel(propellant: &mut String) -> usize {
//     println!("processing propellant {}...", propellant);
//     propellant.push_str(" is highly flammable!");
//     let length = propellant.len();
//     length
// }


// fn celsius_to_fahrenheit(celsius: f64) -> f64 {

//     (celsius * 1.8) + 32.0
// }



// fn square(x: i32) -> (i32, i32) {
//     println!("squaring {}", x);
//     return (x, x * x);
//     println!("End of function");
// }




// fn say_hello() {
//     println!("Hello!");
//     say_a_number(13);
// }

// fn say_a_number(number: i32) {
//     println!("number is {}", number);
// }

// fn say_the_sum(a: u8, b: u8) {
//     let sum = a + b;
//     println!("sum is {}", sum);
// }
