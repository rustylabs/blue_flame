#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_imports)]
pub fn main()
{
    let mut x = vec![0,1,2,3,4];
    println!("Before: {:?}", x);

    x.remove(2);
    println!("After: {:?}", x);
}