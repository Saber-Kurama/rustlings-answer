
// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
    let nice_slice = [2,  3, 4];
    let b = [ 2, 3, 4];
    let c = &a[2..5];
    let d = &a[2..5];
    
    // assert_eq!(b.as_ptr(), nice_slice.as_ptr());

<<<<<<< HEAD
    assert_eq!(d.as_ptr(), c.as_ptr());
    // let nice_slice = &a[1..4];
=======
    let nice_slice = [2, 3, 4];
>>>>>>> 674dd01a203fde58021e93cff2a9b07048ec5b4e

    // assert_eq!([2, 3, 4], *nice_slice)
}

fn main() {
    // info 的 mode = "test"
    println!("????")
}