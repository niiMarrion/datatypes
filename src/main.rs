
use operations::modulus;
use tuples::set_tuple; 
use arrays::set_array; 
mod operations;
mod tuples;
mod arrays;

fn main() {
    // let x:i8 = 10;
    // println!("Value of x is: {}", x);

    // let _y:u8 = 20; // this underscore is used to indicate that the variable is intentionally unused
    // println!("Value of y is: {}", y);  

    // let decimal = 02_55;
    // let hex = 0xff;
    // let octal = 0o77;
    // let binary = 0b1111_0000;
    // let byte = b'A';
    // println!("Decimal: {}, Hex: {}, Octal: {}, Binary: {}, Byte: {}", decimal, hex, octal, binary, byte);

    let x = 2.0;
    let y: f32 = 3.5;

    let t = true;
    let f: bool = false;    

    let c = 'z';

    println!("x: {}, y: {}, t: {}, f: {}, c: {}", x, y, t, f, c);

    let result = modulus(10, 3);
    println!("The modulus of 10 and 3 is: {}", result);
    set_tuple((500, 6.4, 'a')); 
    set_array([1, 2, 3]); 

    let mut nums = vec![1, 2, 3, 4, 5];
    nums.push(12);

    let mut vec = Vec::new(); // This is a vector of type Vec<i32>
    vec.push(10);
    vec.pop(); 

    let mut test_new_vec = Vec::with_capacity(10); // This creates a vector with a capacity of 10
    test_new_vec.push(5);
    println!("The length of test_new_vec is: {}, and its capacity is: {}", test_new_vec.len(), test_new_vec.capacity()); 

    let v: Vec<i32> = (0..5).collect(); // This creates a vector containing the integers from 0 to 4

    let sv: &[i32] = &v[2..4]; // This creates a slice of the vector v, containing the elements at indices 1 and 2
    println!("The slice sv contains: {:?}", sv);
}

