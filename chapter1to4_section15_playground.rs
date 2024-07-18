/*
Date: 1 to 18-July-2024

from my gist:
https://gist.github.com/MongkolchaiTue/f6d21e0f5880f38133489460a8784a39

for Note and code Exercise:
https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f6d21e0f5880f38133489460a8784a39

*/

fn main() {
    //Chapter 4 - Section 15 What Are Structs in Rust
    cpt4_sct15_structs();
    
    //Chapter 4 - Section 12 How to Slice in Rust
    cpt4_sct12_slice();
    
    //Chapter 4 - Section 7 Ownership in Rust - The Stack vs The Heap
    cpt4_sct7_stack_heap();
    
    //Chapter 4 - Section 4 What Are Arrays in Rust
    cpt4_sct4_arrays();
    
    //Chapter 4 - Section 1 What Are Tuples in Rust
    cpt4_sct1_tuples();
    
    // section 39 What Is a Function in Rust
    exsample10();
    
    // section 33 What Are Loops in Rust
    exsample9();
    
    // section 24 What Are Operators in Rust
    exsample8();
    
    // section 20 What Is the String Object in Rust-from() and len()
    exsample7();
    
    // section 19 What are String Literals str in Rust
    exsample6();
    
    // section 18 What Is Const in Rust-Constants
    exsample5();
    
    // section 16 What Are Variables in Rust
    exsample4();
    
    // Section 12 What Are Integers in Rust
    exsample3();
    
    // Section 10 What Is the Print Line Println macro in Rust
    
    exsample2();
    
    // Section 8 Solution write vaiable in rust
    exsample1();
}

    // Building A function example
    fn fn_function() {
        println!("Hello, I am a function");
    }
    
    // Some functions can have return statements that return value back to the caller
    fn fn_return() -> bool {
        return true;
    }
    
    // Paramaterized Function Example
    fn another_function(x:i32) {
        println!("the value of x is: {}", x);
    }
    
    // Section 41- Exercise
    fn plus_one(y:i32) -> i32 {
        return y + 1;
    }
    
fn exsample10() {
    // section 39 What Is a Function in Rust
    println!("exsample10...section 39 What Is a Function in Rust");
    /*
    What Are Functions in Rust
    Functions -
    - are the shells that contain the blocks of maintainable, and reusable code.
    - are "self contained" modules of code that accomplish a specific task.
    - usually "take in" data, process it, and "return" a result.
    
    Once defined, functions may be called to access code. This makes the code reusable.
    Moreover, functions make it easy to read and maintain the program's code.
    
    Many programming languages have built-in functions that you can access in their library,
    but you can also create your own functins.
    
    Function & Description
    1. Building a Fuction
        A function contains the instructions on what and how to perform code
    2. Calling or invoking a Function
        A function must be called so as to execute it.
    3. Returning Functions
        Functions may also return value back to the caller
    4. Parameterized Function
        Parameters are a mechanism to pass values to functions.
    */
    
    println!("Section 40");
    // calling...invoking  the fn_function
    fn_function();
    another_function(if fn_return() { 10000 } else {12000});
    
    
    println!("Section 41");
    /*
    Function Exercise
    
    Write a function called plus_one that takes a signed integer 32 as a parameter
    and returns the integer added with the value of 1.
    
    In a main function create a variable y which stores the result
    of invoking the plus_one function with the argument of 5 and print
    in the main function the resulf of y.
    Good luck!
    
    */
    
    let x:i32 = 5;
    let y:i32 = plus_one(x);
    println!("function plus_one({}) return value is {}", x, y);
    
    
    println!("exsample10...end");
}

fn exsample9() {
    // section 33 What Are Loops in Rust
    println!("exsample9...section 33 What Are Loops in Rust");
    /*
    How To Loop in Rust
    If you want to repeat a block of code multiple times, which there can be many instances in your program to do
    so such as basic caculations for example, you can use loops.
    
    In general, programming instructions happen sequentially:
    This mains that the first statement in a function would be executed first followed by the next and on
    and so forth.
    
    Languages in programming generally
    allow us to manipulate the execution paths and customize them more for preferrence
    
    Rust has three types of core loops for executing blocks of code
    
    1. while
    2. loop
    3. for
    
    */
    
    println!("section 34...For loop");
    // For loop example - we call this a definite loop because we run it until...
    
    for a in 1..20 {
        // 20 is not included
        if a == 2 {
            // the continue statement skips the subsequent statement
            // in the current iteration and takes the control back to the beginning
            // of the loop
            continue;
        }
        println!("a is {}", a);
    }
    
    
    println!("section 35...While loop");
    //an indefinite loop is used when the number of iterations in a loop
    //is indeterminte or unknown
    let mut b = 0;
    while b < 5 {
        b = b + 1;
        println!("loop b value is {}", b);
    }
    
    println!("section 36...loop");
    // the loop also acts while something is true
    let mut c = 0;
    loop {
        c -= 1; // c = c - 1 is the same
        println!("c = {}", c);
        
        if c == -10 {
            //break ends the loop
            break;
        }
    }
    
    println!("section 38...How to loop in Rust");
    /*
        Exercise: How To Loop in Rust
        1. Create a unsigned mutable variable called count of 32bits
        2. Write an infinite loop that increments count +1 and stored the value in count and has the following conditions:
        3. If count is equal to 3 then print the string literal "Welcome tho Miami!" in the console
        4. If the count equals 5 then print the stringliteral "Time to call it a day!" and then exit the loop.
    */
    // Exercise Solution
    let mut count: u32 = 0;
    loop {
        
        count += 1;
        if count == 3 {
            println!("Welcome to Miami!");
        }
        
        println!("{}", count);
        
        if count == 5 {
            println!("Time to call it a day!");
            break;
        }
    }
    
    
    println!("exsample9...end");
}

fn exsample8() {
// section 24 What Are Operators in Rust
println!("exsample8...section 24 What Are Operators in Rust");

/*
What are Operators in Rust
An operator is a character or characters that determine the action that is to be performed or considered.
The data on which operators are actioned are called operands. ?Consider the following expression -

a + b = c

Here, the values a, b, and c are operands, while + and = are operators.

The Major Operations in Rust to consider are the following:

Arithetic
Bitwise
Comparison
Logical
Conditional

*/

/*
Section 25
Arithmetic Operators

x = 12
y = 2

Show Examples
Sr.No   Operator    Description Example
1.  + (Addition) returns the sum of the operators x + y = 14
2.  - (Subtraction) returns the difference of the values x - y = 10
3.  * (Multiplication) returns the product of the values x * y = 24
4.  / (Division) performs division operation and returns the quotient x / y = 6
5.  % (Modulus) performs division operation and returns the remainder x % y = 0

*/

/*
Section 26
Relational Operators

Relational Operators check or define the relationship equivalency between two elements.
Relational operators are used to compare two or more values.
Relational operators return a Boolean value = true or false.

x = 5
y = 2

Show Examples
No. Operator    Description Example
1.  >   Greater than    ( x > y) is true
2.  <   Lesser than (x < y) is false
3.  >=  Greater than or equal to    (x >= Y) is true
4.  <=  Lesser than or equal to     (x <= y) is false
5.  ==  Equality    (x == y) is false
6.  !=  Not equal   (x != y) is true

Section 27
Logical Operators

Logical Operators are used to combine and check two or more conditions.
Logical Operators return a Boolean value.

Show Examples
x = 1
y = 2

No. Operator    Description Example
1.  &&  (And)    The operator returns true as long as all the expressions specified return true
                (x > 0 && y > 3) is false
2.  ||  (OR)     The operator returns true if at least one of the expressions specified return true
                (x > 2 || y > 3) is false
3.  !   (NOT)   The operator returns the inverse of the expression's result.
                For E.g.: !(y > 5) returns false !(x > 4)
                
Section 28
Decision Making Structures in Rust

Desision making structures check statement evaluations. and make various outputted decisions programatically
Based on the conditins of the statement - whether they may be be true or alternatives if they are false etc
1.
if statement
An if statement consists of a true or false expression followed by one or more statements.

2.
if...else statement
An if else statement in programing is a conditinal statement that runs a differrent
set of statements depending on whether an expression is ture or false - Boolean.

3.
else...if and nested if statement
You can nest one if or else if statement inside another if or else if statement(s) and so on and so forth.

4.
match statement
A match statement, similar to the Switch statement in C, allows a variable to be tested against a list of values.
*/

println!("Section 27-28");
// if statement example:
let user = "todd";
if user.len() == 4 {
    println!("Pass");
}

// if...else statement example:
let user2 = "fred";
if user2.len() == 3 {
    println!("Pass");
} else {
    println!("Fail!");
}

println!("Section 29");
//if...else...if and nested if statement example:
let password = "sunday";
if password.len() > 3 {
    println!("Thank you for creating a password!");
} else if password.len() > 2 {
    println!("Please add at least one more char to your password");
} else {
    println!("The password is too short! Please make a binger password!")
}

println!("Section 30");
// match statement example:
let microbiome = "xc12";
let body_part = match microbiome {
    "xc12" => {
        println!("Found match for microbime!"); "Tummy Biome"
    },
    "mpt1" => "Eyebiome",
    "ttw6" => "Fingerbiome",
    _ => "Uknown"
};
println!("The biome match is {}", body_part);


/*
Section 31
Exercise: Decision Making ?structures in Rust

1. Write two signed 32bit constants x and y and assign x the value 3 and y the value 4
2. Check to see whether or not x is less than y and x is greater than 6
3. If the check passes print into the console the following string: "fail"
4. If the check provides a false boolean conduct the following tests:
check if x is less than y or x is greater than 6
If the second test passes print the following string into the console: "success"
If the second test fails print the following string into the console: "please try again"

*/ 

println!("Section 31");
const X:i32 = 3;
const Y:i32 = 4;
if X < Y && X > 6 {
    println!("fail!");
} else if X < Y || X > 6 {
    println!("success!")
} else {
    println!("please try again")
}

println!("exsample8...end");
}

fn exsample7() {
    // section 20 What Is the String Object in Rust-from() and len()
    println!("exsample7...section 20 What Is the String Object in Rust-from() and len()");
    /*
    What is The String Object
    
    The String object type is derived from the Standard Library.
    While the string iteral is part of the core language, the string object type is not a part of the core language.
    String is in a growing collection and is mutable and UTF-8 endcoded type.
    The String object type can be used to represent string values that are provided at runtime.
    String object is allocated in the heap.
    */
    
    // Example: to create a string Object that is empty we can use new() and we can use
    // from() to initialize a value to a new string Object
    
    let nothing_within = String::new();
    //len() finds the length value of a string
    println!("{}",nothing_within.len());
    
    let great_movie = String::from("The Big Lebowski");
    println!("{}", great_movie.len());
    
    // push Example
    let mut greeting = String::from("Julia says, ");
    greeting.push_str("hello!");
    println!("{}", greeting);
    
    //Convert a string literal into a String Object
    let random_string = "Please make me into an object!".to_string();
    println!("{}", random_string);
    
    /*
    String Object Methods:
    new() pub cont fn new() -> String Creates a new empty String.
    to_string() fn to_string(&self) -> String Converts the given value to a String.
    replace() pub fn replace<'a, P>(&'a self, from: P, to: &str) -> String Replaces all matches of a pattern with another string.
    as_str()  pub fn as_str(&self) -> &str Extracts a string slice containing the entire string.
    push() pub fn push(&mut self, ch: char) Appends the given char to the end of this String.
    push_str() pub fn push_str(&mut self, string: &str) Appends a given string slice onto the end of this String.
    len() pub fn len(&self) -> usize Returns the length of this String, in bytes.
    trim() pub fn trim(&self) -> &str Returns a string slice with leading and trailing whitespace removed.
    split_whitespace() pub fn split_whitespace(&self) -> SplitWhitespace Splits a string slice by whitespace and returns an iterator.
    split() pub fn split<'a, P>(&'a self, pat: P) -> Split<'a, P>, where P is pattern can be &str, char, or a closure that determines the splite.
    Retuens an iterator over substrings of this string slice, separated by characters matched by a pattern.
    chars() pub fn chars(&self) -> Chars Returns an iterator over the chars of a string slice.
    */
    
    
    /*
    Section 22
    Exercise-Modifying String Literals in Rust
    1. In the main function, Create a string literal named password and assign it the value of "pokemon,"
    2. Using the String Object push method, modify the password so that it includes " gotta catch them all"
    3. Print the result and share your solution on the discord in the Rust channel!
    */
    
    //Solution
    let mut password = String::from("pokemon,");
    password.push_str(" gotta catch them all");
    println!("{}", password);
    
    println!("exsample7...end");
}

fn exsample6() {
    println!("exsample6...section 19 What are String Literals str in Rust");
    
    /*
    Strings: String Literal
    
    String literals are a set of characters, which are hardcoded into a variable.
    
    let user = "The Incredible Hulk". String literals are found in module std::str.
    
    String literals can also be called string slices.
    
    The String data type in Rust can be applied as such:
    
    String Literal (&str)
    String Object (String)
    
    */
    
    let greeting = "world!";
    println!("Hello {}", greeting);
    
    let bank:&str = "Citi Bank";
    let currency:&str = "Bitcoin";
    println!("The bank is : {} and the currency is: {}", bank, currency);
    
    
    /*
    String literals are static by default. This ensures that the string is valid
    for the entire duation of the program. You can explicitly declare a string as static
    */
    
    // Explicitly declare static strings
    let bank2:&'static str = "Sky";
    let currency2:&'static str = "Ethereum";
    println!("The bank is : {} and the currency is: {}", bank2, currency2);
    
    
    println!("exsample6...end");
}


fn exsample5() {
    println!("exsample5...section 18 What Is Const in Rust-Constants");
    /*
    Constants in Rust
    
    Constants ensures that values in variables cannot be changed.
    Constants must be explicitly stated with the const keyword.
    
    Naming Convention for Constants:
    All characters in a constant variable should be 
    in uppercase for proper convention.
    IE: const USER
    
    */
    
    // Overshadowing variables work for variables and will override the variable
    //let x = 5; //warning: unused variable: `x`
    let x = 6;
    println!("{}", x);
    
    const Y:i32 = 5;
    //const Y = 6; //error[E0428]: the name `Y` is defined multiple times
    println!("{}", Y);
    
    println!("exsample5...end");
}

fn exsample4() {
    println!("exsample4...section 16 What Are Variables in Rust");
    
    // section 16 What Are Variables in Rust
    /*
    The Variable Rule Book in Rust
    When it comes to Rust there are different ways to name a variable
    1. You can name variables with letters, digits, and the underscore character.
    2. Variables must begin with either an underscore or an letter
    3. Since Rust is case sensitive, upperscore and lowerscore variables are distinct
    
    Data types do not have to be staticall declared and are optional.
    The data type is applied form the value assigned to the variable.
    
    Variables are immutable by default
    Variables are read only in Rust by default.
    We can apply mut keyword in order to make the variable mutable.
    
    Immutability
    
    Variables are immutable by default
    
    Variables are read only in Rust by default.
    
    We can apply the mut keyword in order to make the variable mutable.
    
    */
    
    
    let mut x = 5;
    x = x + 1;
    println!("{}", x);
    
    /*
    Exercise
    1. Create a mutable numberical variable y of the value initialized to 12
    2. Change the value to 15 and print the result in the main fuction
    */
    let mut y = 12;
    // y = 15 //maybe it is overwritten before being read?
    y = y + 3; 
    println!("{}", y);
    
    
    println!("exsample4...end");
}


fn exsample3() {
    println!("exsample3...Section 12 What Are Integers in Rust");
    
    // Section 12 What Are Integers in Rust
    /*
        Integers In Rust
        Integers in Rust are numbers without decimals or number that are not fractionalized
        Simply put, integers are data types that represent whole numbers.
        
        Within the umbrella of integers you have signed and unsigned version that you can specify
        
        Signed integers store both negative and postivie values
        
        Unsigned integers may only store positive values
        
        In addition, the size of an integer can be set to arch.
        
        Setting an integer to arch derives the size of the integer to the architecture of the machine
        IE if you set it to 64bits then x64 machine
    */
    
    // Examples
    let total = 4; // i32 by default
    let height:u32 = 41;
    let deduction:i32 = 2-200;
    println!("the total is {}", total);
    println!("the height is {} and the deduction is {}", height, deduction);
    
    println!("exsample3...Section 13 Integer Ranbge and Integer Overflow in Rust");
    /*
    The Integer Range. (x axis - x1 to x2)
    
    Signed integers can store numbers from -2^(n-1) to 2^(n-1) -1,
    where n is the number of bits of the variant.
    For example, i16 can store number from -(2^15) to 2^15 -1, here we replaced n with 16.
    
    UnSigned integers can store numbers from 0 to 2^(n-1) -1,
    For example, u16 can store number from 0 to 2^16 -1, which is equal to 0 to 65535.
    
    The Integer Overflow
    
    An integer overflow occurs when the value assigned to an integer variable exceeds
    the Rust defined range for the data type and starts at 0.
    */
    
    /*
    Exercise:
    1. In the main function create an unsigned variable of 16 bits
    and assign the variable to the max total of the range forula applied to 16bits.
    
    2. Create two more variables, overtime_1 and overtime_2 where overtime_1 has an overflow value of 0
    and overtime_2 has an overflow value of 1.
    
    3. Print your results and run the code and then check yor console for errors.
    **Important hint: the console sulution should display warning errors since your code should experience overflow.
    
    Good luck!
    */
    
    let time:u16 =  65535; // Overflow 65536
    let overtime1:u16 = time-1; 
    let overtime2:u16 = time+0;
    println!(" the time is: {}", time);
    println!(" the over time 1 is: {}", overtime1);
    println!(" the over time 2 is: {}", overtime2);
    
    println!("exsample3...end");
}

fn exsample2() {
    println!("exsample2...Section 10 What Is the Print Line Println macro in Rust");
    
    // Section 10 What Is the Print Line Println macro in Rust
    // in general, {} will be automatically replaced with any arguments and stringified
    
    println!("{} years old", 36);
    
    // named arguments also work
    println!("{user1}{action}{user2}",
    user1="Mary Lewis",
    user2="Jon Wick",
    action=" karate kicked "
    );
    
    // Exercise: Within the function main add a println macro which takes two arguments,
    // x and y stringifies them where x equals "hello" and y equals "my friend".
    println!("{x},{y}",
    x="hello ",
    y="my friend");
    
    println!("exsample2...end");
}

fn exsample1() {
    println!("exsample1...Section 8 Solution write vaiable in rust");
    
    // Section 8 Solution write vaiable in rust
    let financial_management = "Bank Of America"; //string type
    let credit_score = "800"; //integer type
    // float types take decimals
    let account_active = true; // boolean type
    
    println!("My current investor portfolio is managed by: {}", financial_management);
    println!("My credit score is: {}", credit_score);
    println!("Account Active: {}", account_active);
    
    /*
    
    Exercise: Create two variables in the main function called switch and volume.
    Assign oneof the variables to false ant the other variable the value of 10 and print macro twice.
    Add strings to provide logical context to the printed results of your choosing
    
    */
    let switch = false;
    let volume = 10;
    
    println!("Launch Mode is set to: {}", switch);
    println!("Current Fuel Level is : {}", volume);
    
    println!("exsample1...end");
}


// Chapter 4 Beginner to Intermediate Lessons-playground.rs packtpub.com - Courses - Rust Programming 2023 - A Comprehensive Course for Beginner...By Clarian North
/*
Chapter 4
Beginner to Intermediate Lessons
*/

/*
** Tuples in Rust **
Tuples are compound data types. A scalar type, or the variables
we've seen so far, can store only one type of data.
For example, an u32 variable will only store a single integer value.
Compound types can store multiple values at the same time of different types.

Tuples have a fixed length - once declared they cannot grow or shrink in size.
The tuple index starts from 0.

*/

fn cpt4_sct1_tuples() {
    //Chapter 4 - Section 1 What Are Tuples in Rust
    println!("Chapter 4 - Section 1 - What Are Tuples in Rust");
    
    let tuple:(i8, f32, i32) = (2, 3.4, 100);
    println!("{:?}", tuple);
    println!("the first value is : {:?}", tuple.0);
    
    
    println!("Chapter 4 - Section 2");
    /*
    Exercise
    Destructing Tuples in Rust
    
    1. Create a function user_data which takes the tuple x as a parameter containing a signed integer 32bits,
    a boolean, and string literal (hint: use the &str keyword to point the reference)
    
    2. In the function assign a tuple to distinct variables by naming the integer: age,
    boolean: active, and string literal: name.
    Big hint: let (integer, bool, and string) = x
    what you are doing here is assigning a tuple to distinct types
    
    3. Write instructions in the function to print the age, active status and name
    
    4. In the main function, create a new tuple, user2 and set the user data so that the user is 30,
    active status is true, and his name is Jack
    
    5. Invoke the user_data passing in user2 as the argument and check the console for your result
    
    Good luck!
    
    */
    
    user_data(30, true, "Jack");
    
    
    
    user_data_solution((30, true, "Jack"));
    
    let user2:(i32, bool, &str) = (33, true, "User2");
    user_data_solution(user2);
    
    println!("Chapter 4 - Section 1-2...end");
}

fn user_data(age: i32, active: bool, name: &str) {
    let user = (age, active, name);
    println!("user: {:?} age: {:?} status: {:?}", user.2, user.0, user.1);
}

fn user_data_solution(x:(i32, bool, & str)) {
    let (age, active, name) = x;
    println!("age:{}, active:{}, name:{}", age, active, name);
}


fn cpt4_sct4_arrays() {
    //Chapter 4 - Section 4 What Are Arrays in Rust
    println!("Chapter 4 - Section 4 - What Are Arrays in Rust");
    /*
    ** What Are Arrays in Rust **
    
    An array is a collection of objects of the same type T,
    stored in contiguous memory. Arrays are created using brackets [],
    and their length, which is known at compile time,
    is part of their type signature [T; length].
    
    Defining Arrays:
    An array consists of sequential memory blocks.
    
    Arrays are static. Arrays connot be resized once initialized
    
    Each memory block represents an array element.
    
    Array elements are identified by the index
    
    Array element values can be updated or modified but cannot be deleted.
    
    */
    let arr:[&str; 4] = ["Jerry","George","Elaine","Kramer"];
    let mut arr2 = ["Jerry","George","Elaine","Kramer"];
    arr2[2] = "Elaine Benis";
    
    println!("the cast of seinfeld consist of {:?}", arr2);
    println!("the array's total length is {}", arr.len());
    
    //the iter function
    for value in arr.iter() {
        println!("value is {}", value);
    }
    
    println!("Section 5 Exercise - Loop Through and modify an array");
    /*
    Loop and mutate an array in Rust
    1. Create an array with the following integer values: 12, 2, 3, 2, 4, 5
    2. Write a for loop with can loop through the array and replace all the integer values of 2 with 0.
    Print the amended result tagged along with the index value of each integer
    
    Good luck and have fun!
    */
    
    let mut arr_ints:[i32; 6] = [12, 2, 3, 2, 4, 5];
    let mut i = 0;
    while i < arr_ints.len() {
        if arr_ints[i] == 2 {
            arr_ints[i] = 0;
        }
        println!("index is {}, value is {}", i, arr_ints[i]);
        i+=1;
    }
    
    println!("Chapter 4 - Section 4...end");
}

fn cpt4_sct7_stack_heap() {
 //Chapter 4 - Section 7 Ownership in Rust - The Stack vs The Heap
 println!("Chapter 4 - Section 7 Ownership in Rust - The Stack vs The Heap");
 /*
 Section 7
 The Stack Versus the Heap in Rust
 In Rust, memory management is a crucial part of writing efficient and safe programs.
 Understanding the differrences between the stack and the heap is important for managing memory effectively.
 In this video, you will learn about the stack and the heap in Rust and how to use them to allocate and manage memory in your programs.
 
 */
 
 /*
 
 ** Ownership in Rust **
 The Stack vs the Heap
 Each method has its own stack designed for promitives (when the method is done you can clean the stack out)
 
 Pointer on the stack will point to an object
 Structs could be on the heap (not as efficient and fast and have to find space to allocate it)
 
 Rust does not use a garbage collector,
 but rather achieves these properties through a sophisticated,
 but complex, type system. Doing so makes Rust very efficient, but makes Rust relatively hard to learn and use.
 
 --------------------
 
 The stack is very fast, and is where memory is allocated in Rust by default.
 But the allocations is local to function call, and is limited in size.
 
 The heap, on the other hand, is slower, and is explicitly allocated by your program.
 But is's effectively unlimited in size, and is globally accessible.
 
 Note: this maining of heap, which allocates arbitrary-sized blocks of memory in arbitrary order,
 is quite different from the heap data structure.
 
 when a function gets called, some memory gets allocated for all of its local
 variables and some other information. This is called a 'stack frame'
 
 So if the stack is faster and easier to manage, why do we neet the heap?
 A big reason is that Stack-allocation alone means you only have 'Last In First Out (LIFO)'
 semantics for reclaiming storage. Heap-allocation is strictly more general,
 allowing storage to be taken from and returned to the pool in arbitrary order, but at a complex
 
 Generally, you should prefer stack allocation, and so, Rust Stack-allocates by default.
 The LIFO model of the stack is simpler, at a fundamental level. This has two big impacts:
 runtime efficiency and semantic impact.
 
 
 */


 /*
 The Stack vs the Heap
 
 Stack : Fast
 - i = 23
 - bool = true
 -
 -
 Heap: slow
 --- -- --- Struct ---- ---
   --- -- ----
 */
 
  
 println!("Section 8..Ownership in Rust");
 /*
 
 Section 8
 Ownership in Rust
 Ownership is a key concept in Rust's memory management system, and it's essential for writing efficient
 and safe programs. in this video, you will learn about ownership in Rust and how it helps prevent common memory-related errors.
 
 ----
 Variables of a value an owner in Rust.
 All the data that is stroed in Rust has an owner associated to it.
 
 let time = 20 - time is the owner of the value 20
 
 Data only has one owner at a time.
 Multiple variables cannot point to the dame memory location
 Variables always point to different memory locations
 
 That being said, you can transfer ownership in Rust:
 - Assigning one variable's value to another variable.
 - Inputting value to a function.
 - Outputting value from a function.
 
 A vector is a data structure that contains zero or more items of the same type.
 The items have an order, and you can access the items by index (0, 1, ...).
 You can add and remove items. The items themselves are stored in a contiguous
 heap-allocated area.
 
 */
 
 let vector1 = vec![2, 4, 6];
 // vector vector1 owns the object in the heap
 // only a single variable owns the heap memory at a given time
 
 let vector2 = vector1;
 
 // Rust is going to check for memory access an that's really a key
 // selling point for Rust.
 
 println!("{:?}", vector2);
 
 
 println!("Section 9...Borrowing in Rust—The '&' Symbol in Rust");
 // Section 9 - Borrowing in Rust—The "&" Symbol in Rust
 /*
  Borrowing in Rust—The "&" Symbol in Rust
  
  In Rust, borrowing is an essential concept for managing memory
  and preventing common memory-related errors. The '&' symbol is used to create references to variables
  and pass them to functions, which allows efficient memory management without sacrificing safety
  In this video, you will learn about borrowing in Rust and how it helps prevent common memory-related errors.
 */
 
 /*
 Borrowing In Rust
 
 It can be quite a hustle transferring ownership of a variable to another function
 and then returning ownership. Through temporarily transferring ownership of a value,
 Rust supports borrowing ownership in which the ownership is then returned to the original owner.
 
 Functions can transfer their control of a value to another function temporarily
 and that is what we call borrowing in Rust. You can pass a reference to the variable with (& var _name)
 as opposed to passing the variable itself. The ownership of the variable / value is transferred to the original owner of the variable.
 
 */
 
  let vector = vec![1, 2, 3];
 //display(vector); /*error[E0382]: borrow of moved value: `vector`*/
 display(&vector);
 println!("{}", vector[1]);

 
 println!("Section 10...Borrowing and Referrences Exercise in Rust:");
 /*
 Create a function called display2 which takes a string object as a parameter
 and pushes onto the string an "F8 Tributo" which is an awesome type of Ferrari!
 
 Create variable car which is string object with the default "Ferrari".
 Call display2 passing in the car variable as the argument
 and print the modified version of the variable car.
 
 */
 
 // Exercise Solution
 let mut car:String = String::from("Ferrari");
 display2(&mut car);
 println!("Car updated value is {}", car);
 
 println!("Chapter 4 - Section 7-10...end");
}

fn display2(carname:&mut String) {
    println!("Car value is {}", carname);
    carname.push_str(" F8 Tributo");
}

//fn display(x:Vec<i32>) /*error[E0382]: borrow of moved value: `vector`*/
fn display(x:&Vec<i32>) {
    println!("{:?}", x);
}


fn cpt4_sct12_slice() {
    //Chapter 4 - Section 12 How to Slice in Rust
    println!("Chapter 4 - Section 12 How to Slice in Rust");
    /*
    Section 12
    How to Slice in Rust
    
    In Rust, slicing is a powerful tool for working with arrays, strings, and other collections.
    With slicing, you can extract specific parts of a collection, modify them,
    and reassemble them in new ways. In this video, you will learn how to slice in Rust,
    including how to create slices, how to use slice notation, and how to modify slices in place.
    */
    
    /*
    Slices in Rust
    
    Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
    A slice is a kind of reference, so it does not have ownership.
    
    Slices essentially point to data.
    They are passed by referrence to functions ie borrowing.
    You can use them to fetch portions of data and customize what you want to slice.
    
    */
    
    // Slice Example
    let game = "Mario Brothers".to_string();
    println!("length of the game is {}", game.len());
    
    let slice = &game[0..5];
    println!("slice {}", slice);
    
        
    println!("Section 13 - Exercise - How to Slice in Rust");
    // Section 13 - Exercise - How to Slice in Rust
    /*
    1. Create an array with the values 1 through 5 called nums.
    2. Print the values of nums.
    3. Write a function called slice_and_dice which takes an array as a parameter
        and prints the length of the array. It should also print the updated value of the array.
        The function should also replace the first index value in the array to 200.
    4. Call the slice and dice function in the main function and slice out the values 2 and 3 from the array.
    
    There are a few little tricks here so be vigilent and good luck!
    
    */
    
    // Exercise Solution
    let mut nums:[i32; 5] = [1, 2, 3, 4, 5];
    println!("nums values is {:?}", nums);
    
    slice_and_dice(&mut nums[1..3]);
    
    println!("nums updated values is {:?}", nums);
    //let slice_nums = &nums[1..3];
    //println!("slice_nums values is {:?}", slice_nums);
    
    println!("Chapter 4 - Section 12-13...end");
}

fn slice_and_dice(slice:&mut [i32]) {
    println!("the length of slice is {}", slice.len());
    println!("slice values is {:?}", slice);
    slice[0] = 200;
}

fn cpt4_sct15_structs() {
    //Chapter 4 - Section 15 What Are Structs in Rust
    println!("Chapter 4 - Section 15 What Are Structs in Rust");
    
    /*
    
    What are Structs in Rust
    
    Structs are similar to tuples, discussed in "The Tuple Type" section,
    in that both hold multiple related values. Like tuples, the pieces of a struct can be different types.
    Unlike with tuples, in a struct you'll name each piece of data so it's clear what the values mean.
    Adding these names means that structs are more flexible than tuples:
    you don't have to rely on the order of the data to specify or access the values of an instance.
    
    Structs use key value pairs to define data.
    
    Rust allows to combine data items of differrent types,
    including other structures.
    
    -------
    
    Methods are functions which belong in the scope within a structure.
    You declare methods with the impl keyword outside of the structure block.
    Important to note that the parameter of a method will be always self,
    which represents the calling instance of the structure.
    
    */

    /*
    Example:
    struct The_struct {
        
    }
    
    impl The_struct {
        fn function(&self) {
            
        }
    }
    
    */
    
    /*
    https://doc.rust-lang.org/rust-by-example/fn/methods.html#associated-functions--methods
    Some functions are connected to a particular type.
    These come in two forms:
    1. Associated functions
        are functions that are defined on a type generally,
    2. while methods
        are associated functions that are called on a particular instance of a type.

    */
    
    // Struct Example
    struct House {
        
        door: String,
        garden: String,
        property_value: u32
    }
    
    let home1 = House {
        door: String::from("Blue"),
        garden:String::from("Beautiful"),
        property_value:1000000
    };
    
    println!("door is: {}, garden is: {}, property value is: {}", home1.door, home1.garden, home1.property_value);
    
        println!("Section 16 - Write Methods and Structs in Rust - Build A Triangle Calculator in Rust with Structs");
    /*
    Build A Triangle Calculator in Rust with Structs
    
    1. Write a Triangle struct that takes the key pair values of base and height as unsigned integers
    2. Write a method for the Triangle with can calculate the area
    of the triangle (remember the triangle area formula is base * height / 2)
    3. Create a new triangle struct and set the base to 10 and the height to 30
    4. Print the method calculatin for the area of the new triange.
    
    Good Geometric luck!
    */
    
    struct Triangle {
        base: u32,
        height: u32
    }
    
    impl Triangle {
        fn area(&self) -> u32 {
            self.base * self.height / 2
        }
    }
    
    let triangle = Triangle{base: 10, height: 30};
    println!("the area of triange base:{}, height:{} is {}", triangle.base, triangle.height, triangle.area());
    
    println!("Section 18 - What Are Enums in Rust");
    
    /*
    What Are Enums in Rust
    
    An enum type is a special data type that enables for a variable to be a set of predefined constants.
    The variable must be equal to one of the values that have been predefined for it.
    Common examples include compass directions (values of NORTH, SOUTH, EAST, and WEST) and the days of the week.
    
    */
    
    // Example Enum
    // the derive attribute makes the enum printable
    #[derive(Debug)]
    enum TemperatureGrade {
        Hot, Cold, Medium
    }
    
    #[derive(Debug)]
    struct City {
        name:String,
        temperature:TemperatureGrade
    }
    
    let c1 = City {
        name:String::from("Alaska"),
        temperature:TemperatureGrade::Cold
    };
    
    let c2 = City {
        name:String::from("Miami"),
        temperature:TemperatureGrade::Hot
    };
    
    let c3 = City {
        name:String::from("Miami"),
        temperature:TemperatureGrade::Medium
    };
    
    println!("c1 is {:?}, name is {}, temperature is {:#?}", c1, c1.name, c1.temperature);
    println!("c2 is {:?}, name is {}, temperature is {:#?}", c2, c2.name, c2.temperature);
    println!("c3 is {:?}, name is {}, temperature is {:#?}", c3, c3.name, c3.temperature);
    
    println!("Section 19-20 Exercise - Matching Enums In Rust");
    /*
    Matching Enums In Rust Exercise
    
    1. Write an Enum for three differrent types of shoes: loafer, nike, and vans
    2. Using the match keyword, write a function that takes the enum as a value
       and can match each shoe with the following printed string values:
       nike: "Great for running", loafers: "Great for loafing around", vans: "Great for skateboarding"
    3. call the function and print each string match by passing in each enum value as an argument
       and printing the matched result
    */
    
    /*
    Extra hine:
    Match keyword -
    Match a single value
            1 => println!("One!"),
    */
    
    shoe_for(ShoeTypes::Loafer);
    shoe_for(ShoeTypes::Nike);
    shoe_for(ShoeTypes::Vans);
    
    println!("Chapter 4 - Section 15-17 Struct, 18-20 Enums...end");
}

#[derive(Debug)]
enum ShoeTypes {
    Loafer, Nike, Vans
}
    
fn shoe_for(shoe:ShoeTypes) {
    let shoe_for_info = match shoe {
        ShoeTypes::Loafer => "Great for loafing around",
        ShoeTypes::Nike => "Great for running",
        ShoeTypes::Vans => "Great for skateboarding",
    };
    
    println!("Shoe is : {:?}, Result : {}", shoe, shoe_for_info);
}
