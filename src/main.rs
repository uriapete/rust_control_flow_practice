use core::num;
use std::{vec, io::stdin, string, result};

fn main() {
    println!("Hello, world!");
    // let mode = "ctof";
    // println!("{}",convert_temps("ftoc",40.0))
    // fibo_gen(377+610);
    
    // menu
    loop {
        println!("Hello! What would you like to do?");

        // initting var for main menu choice
        let mut main_menu_choice=String::new();

        // getting choice from user for what to do
        println!("1) Convert temperatures between Celcius and Fahrenheit");
        println!("2) Generate fibonacci numbers");
        println!("Please input either 1 or 2");
        stdin()
            .read_line(&mut main_menu_choice)
            .expect("Failed to read line");

        let main_menu_choice = main_menu_choice.trim();

        // println!("{}",(main_menu_choice.as_str()==("1")));
        println!("\"{}\"",main_menu_choice);
        // println!("steak");

        // please ignore my godawful code

        // logic for converting temps
        if main_menu_choice =="1"{
            println!("you picked #1");
        }else if main_menu_choice=="2" {
            println!("you picked #2");
        } else {
            println!("invalid choice");
        }

    }
}

// hi, i'm learning rust
// here's the prompt:

// from "The Book" (specifically grabbing from that experiment with the quizzes and stuff)
// To practice with [Rust's control flow], try building programs to do the following:

// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.


fn convert_temps(mode:&str,input_temp:f64)->f64 {
    if mode=="ftoc" {
        return (input_temp-32.0)*(5.0/9.0)
    }else if mode=="ctof" {
        return (input_temp*1.8)+32.0
    }else {
        println!("Please enter a valid mode!");
        return 1.0;
    }
}

// here's the fibo gen fn: print fibo nums until the max as set in args
// this fn already prints stuff out, so no return val and no printing val of this fn
fn fibo_gen(max_num:u128) {
    let mut fibo_list:Vec<u128>=vec![1,2];

    // i'm gonna use a vector to help with calcing fibo

    // below code ended up not working
    // i = index, num = elementnum
    // so i can access next or previous item
    // for (i,num) in fibo_list.iter().enumerate() {
    //     println!("{num}");
    //     if (i+1>=fibo_list.len()) {
    //         let nextNum:u128=fibo_list[i-1]+num;
    //         if nextNum<=max_num{
    //             fibo_list.push(nextNum);
    //         }
    //     }
    // }
    // end of code that doesn't work

    // i'll try a while loop this time

    // current index
    let mut curr_idx = 0;

    // while index does not exceed the length of the fibo_list
    'fibo_loop: while curr_idx<fibo_list.len() {
        // get current num
        let num = fibo_list[curr_idx];
        // print current num
        println!("{num}");

        // // TESTING: print current index
        // println!("{curr_idx}");

        // if the next num does not exist (current index is equal to or exceeds length of fibo_list)
        if (curr_idx+1)>=fibo_list.len() {
            // calc the next num
            let next_num:u128=fibo_list[curr_idx-1]+num;

            // if the next num does not exceed the max number as set per the argument
            if next_num<=max_num{
                // add the next num to the list
                fibo_list.push(next_num);
                // continue 'fibo_loop;
            } else {
                // do i need to explain this line of code?
                // fine i will anyways, if the next num does exceed the max, break the loop and finish the function, the function should return null
                break 'fibo_loop;
            }
        }
        // if code reaches here, that means it didn't break as the next num is still less than max
        // therefore, increment the idx
        curr_idx+=1;
        
    }
    
}