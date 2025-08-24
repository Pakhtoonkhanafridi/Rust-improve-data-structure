// const GLOBAL_CONSTANT: u8 = 100;
use std::io;
fn main() {


    // let mut num:u16 = 256;
    // println!("this the value of num: {}", num);
    // num = 512;
    // println!("this the value of num: {}", num);
    //let  string_literal: &str = "hi Pakhtoon tecnologies";;

    // tips: 
    //string  - Dynamix length string -Heap Allocated
    //$str - Fixed Length Strings - Special memory

    // let mut string_literal:String  = String::from("hi, pakhtoon technologies");
    // string_literal.push_str(" welcome to Pakhtoon technologies");
    // println!("the sentence is stored in the variable string_literal: {}", string_literal);

    //tuple
    // let emp_info:(&str,u8) = ("Pakhtoon",24);

    // let emp_name = emp_info.0;
    // let emp_age = emp_info.1;

    // //destructuring
    // let (employee_name, employee_age) = emp_info;

    // println!("Employee Name={}, Employee Age={}",employee_name,employee_age);
    // println!("Employee Name={}, Employee Age={}",emp_name,emp_age);

    //functions overview
   // print_values(5);
    // let num1: u8 = 10;
    // let num2: u8 = 20;
    // let result:u8 = add(num1,num2);
    // println!("the sum of num1 and num2 is {}", result);

    // let outside_variable= 5;

    // {
    //     let inside_variable = 10;
    //     println!("Inside variable: {}", inside_variable);
    //     println!("Outside variable: {}", outside_variable);
    // }  //  inside_variable goes out of scope here

    // println!("Outside variable: {}", outside_variable);

    // print_values();

    // let str1 = String::from("hello"); //str1 is the owner of Hello
    // let str2 = str1; // transfer the owenership. str2 is the new ower of the Hello
    //  //println!("str1={}",str1);
    //  println!("str2={}",str2);

    // let a = 5;
    // let b =a;
    // println!("a={}",a);
    // println!("a={}",b);
    // {
    //     let a = 15; // shadows the outer `a`
    //     let b = 20; // shadows the outer `b`
    //     println!("Inside block: a={}, b={}", a, b);
    // }

    // we are in ownership
    // let x:u8 = 5;
    // process_integer(x);
    // println!("The value of x in main() is: {}",x);

    // let x: String = String::from("hello"); //x is the owner of Hello
    // process_string(x);// tranfer of ownership
    //println!("the value of x in main() is {}", x);

    // let s1:String = get_string();
    // println!("this is s1: {}", s1);
    
    // let s2:String= String::from("world");
    // let s3: String = send_get_string(s2);

    // println!("this is s3:{}", s3);

//   let s1:String= String::from("hello");
//   let len:usize = calculate_length(&s1); // barrow Operation
//    println!("the lenght of {} is {}",s1, len);
//   let mut s1:String = String::from("hello");
//   append_string(&mut s1);
//   println!("The new string {}",s1);


// fn main() {
//     let mut s1:String = String::from("hello");
//     let w1 = &mut s1;
//     w1.push_str("World");
//     println!("w1={}", w1);
//     let w2 = &mut s1;
//     w2.push_str("Code");
//     println!("w2={}", w2);
//     println!("w1{} w2={}",w1,w2)

// }
//  let mut s1:String = String::from("hHello");
//  {
//     let r1 = &s1; // no problem
//     let r2 = &s1; // no problem
//     println!("{} and {}", r1, r2);
//  } // r1 and r2 are no longer used after this point

// let x = 50;
// println!("x={:p}", &x);
// let y= &x; //y is the reference of x, value of x is 5
// println!("y={:p}", y); //auto derfererencing

//  let mut x = 5;
//  x= x + 1; //6
//  let y = &mut x;
//  *y=*y +1;//7
//  println!("x={}",*y);

// Dangling Reference
// let reference_to_nothing = create_string_ref();

// let arr: [&str; 3] = ["hello", "World", "Coders"];
// write_arr(arr);  // array directly pass
// println!("arr={:?}",arr);

// let mut arr: [&str; 3] = ["hello", "World", "Coders"];
// write_arr(&mut arr);  // array directly pass
// println!("arr={:?}",arr);

// let mut v: Vec<i32> = Vec::new();// Declaration
// let mut v = Vec::<i32>::new();

//  v.push(1);
//  v.push(2);
//  v.push(3);
// let mut v = vec![1,2,3,4,5]; 
// v.push(10);
// v.pop();


// println!("Vector v={:?}",v);
// let vrr: Vec<&str> = vec!["hello","world","coders"];
// write_vrr(&vrr);
//   println!("vrr={:?}",vrr);

//shadowing
//   let x = 5;
//   println!("x={}",x);
//   let x="Hello world";
//   println!("x={}",x);
//   let x=x.len();

//   println!("x={}",x);


//for loop
// 

// fn is_even(num:i8)->bool{

//     return num%2==0;
//     // if num%2==0{
//     //     return true;
//     // }
//     // return false;
// }


// let number= 4;

// match number {

//     x if  is_even(x)=>println!("Even"),
//     x if  !is_even(x)=>println!("Odd"),

//     // 1=>println!{"one"},
//     // 2=>println!{"two"},
//     // 3=>println!{"thre"},
//     // 4|5=>println!{" 005"},
//     _=>println!{"other number not recongized"}
    
//   }

  let mut  input = String::new();
  println!("Please enter your name:");
  io::stdin()
  .read_line(&mut input)
  .expect("INpute failed");
  println!("User input: {}", input);

}

// fn write_vrr(vrr2: &Vec<$str>){
//     // vrr2[0]="Fellow"; // Error cannot assign to `vrr2[_]` which is behind a `&` reference
//       println!("vrr2={:?}",vrr2);

// }

// fn write_arr(arr2:&mut [&str; 3]){
//    //  arr2[0]="Fellow";
//      println!("arr1={:?}",arr2);


// }

//   fn write_arr(mut arr1:[&str; 3]){// arr1 new copy of arr
//     arr1[0]="Fellow";
//     println!("arr1={:?}",arr1);

//   }
// fn create_string_ref()->&String{
//     let s:String =String::from("hello");
//     return &s;

// }


// fn append_string(s3:&mut String){
//   s3.push_str("world");
// }

// fn  calculate_length(s2:&String)->usize{
//     return s2.len();
// }


// fn get_string()->String{
//     let new_string = String::from("hello");
//     return new_string; // tranfer of ownership
// }
// fn send_get_string(recieved_string:String)->String{
//     return recieved_string; // tranfer of ownership
// }

// fn process_string(item: String){// the new ower is item
//     println!("the value of item in process_string() is {}",item);
// }

// fn process_integer(x:u8){
//     println!("the value of x in process_integer() is {}", x)
// }


// fn print_values(){
//     println!("{}",GLOBAL_CONSTANT); // Error: cannot find value `outside_variable` in this scope
    
// }

//  fn add(item1: u8, item2: u8)->u8{
//     return item1 + item2;
//  }

// fn print_values(item: u8){
//     println!("Hello, world! {}", item);
// }
