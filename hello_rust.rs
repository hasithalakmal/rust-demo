
/* This is the
   Main function in my hello rust
*/
fn main() {

    // fn_hello();

    // play_with_data_types();

    // variables_and_constants();

    // play_with_strings();

    //conditional_programming();

    //loop_with_rust();

    //tupel_example();

    //arrays_and_paramter_passing();

    //transfer_ownership();

    //borrowing();

}

fn fn_hello(){
   //Let's print something
       let my_name = "Hasitha";
       let rating_float = 4.9;
       let icon_char = '♥';

       println!("Say Hello {} To Rust with {}. and give {} rating out of 5 !!!", icon_char, my_name, rating_float); //This is a predefined macro in Rust
}

fn play_with_data_types() {
    //Rust has four primary scalar types
    let int_type_value:u32 = 20;
    let float_type_value:f64 = 15000.600;
    let boolean_type_value:bool = true;
    let char_type_value:char = 'A';
    let char_type_value_for_fun:char = '😁';
    //option

    println!("int_type_value [{}] - float_type_value [{}] - boolean_type_value [{}] - char_type_value [{}] - char_type_value_for_fun [{}]",
    int_type_value, float_type_value, boolean_type_value, char_type_value, char_type_value_for_fun);

     // Just For Fun
     /*   let val1:u8 = 256;
        let val2:u8 = 257;
        let val3:u8 = 258;
        let val4:f32 = 8; */
}

fn variables_and_constants() {
    //By default, variables are immutable
    let sample_val_1 = 10;
    println!("sample_val_1 - [{}]", sample_val_1);
    //sample_val_1 = 20;
    println!("sample_val_1 after change - [{}]", sample_val_1);

    //Use mutable variables
    let mut sample_val_2 = 10;
    println!("sample_val_2 - [{}]", sample_val_2);
    sample_val_2 = 20;
    println!("sample_val_2 after change - [{}]", sample_val_2);

    //Shadowing of Variables
    let sample_val_3 = 10;
    println!("sample_val_3 - [{}]", sample_val_3);
    let sample_val_3 = 20;
    println!("sample_val_3 after change - [{}]", sample_val_3);

    //shadowung applies to data types as well
    let sample_val_4 = 10;
    println!("sample_val_4 - [{}]", sample_val_4);
    let sample_val_4 = "Changed Value";
    println!("sample_val_4 after change - [{}]", sample_val_4);

    //Constants
    const SAMPLE_VAL_1:&str = "This is a constant";
    println!("SAMPLE_VAL_1 - [{}]", SAMPLE_VAL_1);
   // const SAMPLE_VAL_1:&str = "Changed CONST value";
    println!("SAMPLE_VAL_1 after change - [{}]", SAMPLE_VAL_1);
}

fn play_with_strings() {
    //Basic String operations
    let empty_string = String::new();
    println!("length is {}",empty_string.len());

    let content_string = String::from("Sample String");
    println!("length is {}",content_string.len());

    let mut z = String::new();
    z.push_str("This is with string object");
    println!("length is {}",z.len());

    let name1 = "Hello Hasitha Gamge , Hello!".to_string();
    let name2 = name1.replace("Hello","Howdy");
    println!("{}",name2);
}

fn conditional_programming() {
    //IF Statements
    let num = 2 ;
    if num > 0 {
      println!("{} is positive",num);
    } else if num < 0 {
      println!("{} is negative",num);
    } else {
      println!("{} is neither positive nor negative",num) ;
    }

    //Match Statement
    let state_code = "COL";
    let state = match state_code {
       "COL" => {println!("Found match for COL"); "Colombo"},
       "JF" => "Jaffna",
       "KN" => "Kandy",
        "GA" => "Gampaha",
        _ => "Unknown"
    };
    println!("Province name is {}",state);
}

fn loop_with_rust() {
    //While loop
    let mut x = 0;
    while x < 10{
      x+=1;
      println!("inside loop x value is {}",x);
    }
    println!("outside loop x value is {}",x);

    //For loop
    let mut count = 0;
    for num in 0..21 {
       if num % 2==0 {
           continue;
       }
       count+=1;
    }
    println! (" The count of odd values between 0 and 20 is: {} ",count);
}

fn tupel_example() {
    let tuple:(i32,f64,u8) = (-325,4.9,22);
    println!("integer is :{:?}",tuple.0);
    println!("float is :{:?}",tuple.1);
    println!("unsigned integer is :{:?}",tuple.2);
}

fn arrays_and_paramter_passing() {
    //Arrays and  by value
    let arr = [10,20,30];
    update_array_with_pass_by_value(arr);
    println!("Inside parent after pass by value {:?}",arr);

    //Arrays and  by reference
    let mut arr2 = [10,20,30];
    update_array_with_pass_by_reference(&mut arr2);
    println!("Inside parent after pass by reference {:?}",arr2);
}

fn update_array_with_pass_by_value(mut arr:[i32;3]){
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("Inside update_array_to_zero {:?}",arr);
}

fn update_array_with_pass_by_reference(arr:&mut [i32;3]){
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("Inside update {:?}",arr);
}

fn transfer_ownership() {
    let v = vec![1,2,3];
    // vector v owns the object in heap

    //only a single variable owns the heap memory at any given time
    let v2 = v;
    // here two variables owns heap value,
    //two pointers to the same content is not allowed in rust

    //Rust is very smart in terms of memory access ,so it detects a race condition
    //as two variables point to same heap
    println!("V2 -> {:?}",v2);
//    println!("V -> {:?}",v);
}

fn borrowing() {
    // a list of nos
    let v1 = vec![10,20,30];
    print_vector_pass_by_value(v1);
//    println!("{}",v1[0]); // this line gives error

    // a list of nos
    let v2 = vec![10,20,30];
    print_vector_pass_by_reference(&v2); // passing reference
    println!("Printing the value from main() v[0]={}",v2[0]);

}

fn print_vector_pass_by_value(x:Vec<i32>){
    println!("Inside print_vector function {:?}",x);
}

fn print_vector_pass_by_reference(x:&Vec<i32>){
    println!("Inside print_vector function {:?}",x);
}

//rustc hello_rust.rs