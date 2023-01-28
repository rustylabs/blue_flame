#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_imports)]
pub fn main()
{
    let test = String::from("oSDbject1_sd");
    
    

    for (i, chars) in test.chars().enumerate()
    {
        let chars_num = chars as u8;

        if (chars_num >= 58 && chars_num <= 64) || chars_num <= 47 || (chars_num >= 91 && chars_num <= 96 && chars_num != b'_') || chars_num >= 123
        {
            println!("not allowed {i}");
            break;
        }
        // If number as first character
        if i == 0 && (chars_num >= 48 && chars_num <= 57)
        {
            println!("numers at first char");
            break;
        }
    }

    //println!("{:?}", tokens);

}