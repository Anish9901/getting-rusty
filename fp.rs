// Trying formatted prints
fn main(){
    println!("There are {} days in {} year", 365, 1);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("This is {0}, in binary: {0:b}, in octal: {0:o}, in hex {0:x}", 69420);
    println!("{x} {y} {z}.", x="Apple's", y="stock ticker is", z="AAPL"); 
}