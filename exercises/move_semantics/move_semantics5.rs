/*
 * @Author: saber
 * @Date: 2021-12-09 18:53:43
 * @LastEditTime: 2021-12-10 17:41:28
 * @LastEditors: saber
 * @Description: 
 */
// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)


fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut *y;
    *z += 1000;
    println!("y: {}", y);
    // println!("z: {}", z);
    assert_eq!(x, 1200);
}
