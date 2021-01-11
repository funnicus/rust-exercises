fn main() {
    for i in 1..=12 {
        if i == 1{
            println!("On the 1st day of Christmas");
        }
        else if i == 2 {
            println!("On the 2nd day of Christmas");
        }
        else {
            println!("On the {}th day of Christmas", i);
        }
        println!("my true love gave me");
        println!("{} something something...", i);
    }
}
