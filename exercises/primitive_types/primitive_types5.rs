/*
 * @Author: saber
 * @Date: 2021-12-09 18:53:43
 * @LastEditTime: 2021-12-13 10:18:45
 * @LastEditors: saber
 * @Description: 
 */
// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!


fn main() {
    let cat = ("Furry McFurson", 3.5);
    // 元组 解构 
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
