/*
 * @Author: saber
 * @Date: 2021-12-09 18:53:43
 * @LastEditTime: 2021-12-10 18:14:07
 * @LastEditors: saber
 * @Description: 
 */
// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!


fn main() {
    let mut a:[i32; 3] = [0; 3];
    a[2] = 2;
    println!("a: {:?}", a);
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
