use std::io;

fn main() {
    println!("Enter first numerator: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n1: i32 = input.trim().parse().expect("Invalid input");

    println!("Enter first denominator: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let d1: i32 = input.trim().parse().expect("Invalid input");

    println!("Enter second numerator: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n2: i32 = input.trim().parse().expect("Invalid input");

    println!("Enter second numerator: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let d2: i32 = input.trim().parse().expect("Invalid input");

    println!("Fractions1: {}/{} Fraction2: {}/{}", n1, d1, n2, d2);

    let fraction1 = n1/d1;
    let _fraction2 = n2/d2;

    let divnum = n1/n2;
    let divdenom = d1/d2;

    let multnum = n1*n2;
    let multdenom = d1*d2;

    let addnum = (n1*d2)+(n2*d1);
    let subnum = (n1*d2)-(n2*d1);

    let mixnum = n1%d1;
    let mixwhole = n1/d1;

    println!("Addition of fraction {}", addnum/multdenom);
    println!("Subtraction of fraction {}", subnum/multdenom);
    println!("Multiplication of fraction {}", multnum/multdenom);
    println!("Division of fraction {}", divnum/divdenom);

    if n1>d1 {
        println!("Fraction 1 is an improper Fraction");
        if mixnum == 0 {
            println!("Fraction 1 as a mixed number {}", mixwhole);
        } else {
            println!("Fraction 1 as a mixed number {} {}/{}", mixwhole, mixnum, d1);
        };
    } else {
        println!("Fraction 1 is not an improper Fraction");
    };
    println!("Decimal approximaation of fraction 1: {}", fraction1);

    println!("Enter  in a GPA for the class (0-4 integer): ");

    let mut input = String::new();
    println!("Enter an integer (1-4): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let grade: i16 = input.trim().parse().expect("Invalid input");

    if grade == 4 {
        println!("You got an A!");
    } else if grade == 3 {
        println!("You got an B!");
    } else if grade == 2 {
        println!("You got an C!");
    }  else if grade == 1 {
        println!("You got an D!");
    } else if grade == 0 {
        println!("You got an F!");
    } else {
        println!("That isn't a grade");
    }
}
