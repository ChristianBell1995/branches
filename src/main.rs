fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

// Condition in control flow must be a bool
// e.g. if number would throw an error as its an int

// because if is an expression we can use it on the right side of a let statement.

// Something like this:
  // let condition = true;

  // let number = if condition {
  //     5
  // } else {
  //     "six"
  // };

  // println!("The value of number is: {}", number);
// Would error as Rust has to know the type of number
// => the else branch would have to return an int.
