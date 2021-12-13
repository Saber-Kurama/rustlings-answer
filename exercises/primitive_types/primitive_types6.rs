/*
 * @Author: saber
 * @Date: 2021-12-09 18:53:43
 * @LastEditTime: 2021-12-10 19:07:18
 * @LastEditors: saber
 * @Description: 
 */
// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put the expression for the second element where ??? is so that the test passes.
// Execute `rustlings hint primitive_types6` for hints!


#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
