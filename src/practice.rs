use std::process::Command;
struct Test;
impl Test
{
    const TEST: u8 = 10;
    const ONE: u8 = 20;
}
pub fn main()
{
    let test = Test;

    //println!("{}", test.TEST);
}