/*
 * @Author: saber
 * @Date: 2021-12-09 18:53:43
 * @LastEditTime: 2021-12-16 20:47:13
 * @LastEditors: saber
 * @Description: 
 */
// structs1.rs
// Address all the TODOs to make the tests pass!


struct ColorClassicStruct<'a>  {
    // TODO: Something goes here
    name: String,
    hex: &'a str,
}

struct ColorTupleStruct(String, String);

// struct User {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct{
            name: "green".to_string(),
            hex: "#00FF00"
        };

        // let user1 = User {
        //     email: "someone@example.com",
        //     username: "someusername123",
        //     active: true,
        //     sign_in_count: 1,
        // };
        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct("green".to_string(), "#00FF00".to_string());

        // assert_eq!(green.0, "green");
        // assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        #[derive(Debug)]
        struct unit_struct;

        let message = format!("{:?}s are fun!", unit_struct);

        // assert_eq!(message, "UnitStructs are fun!");
    }
}
