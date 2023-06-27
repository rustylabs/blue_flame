use std::path::PathBuf;

pub fn main()
{
    //let file_path       = "/mnt/Windows10/Users/Nishant/Desktop/My made programs/Projects/Game Engine/blue_flame/scenes";
    let project_dir     = "/mnt/Windows10/Users/Nishant/Desktop/My made programs/Projects/Game Engine/blue_flame";
    let file_path       = "/mnt/Windows10/Users/Nishant/Desktop/My made programs/Projects/Game Engine/Assets/scenes/";
    //let file_path       = "/mnt/Windows10/Users/Nishant/Desktop/My made programs/Projects/Game Engine/blue_flame/Assets/scenes/";

    let mut relative_path = PathBuf::new();
    relative_path.push("..");
    

    //println!("relative_path: {}", relative_path.display().to_string());
    
    //println!("{}", blue_flame_common::file_path_handling::fullpath_to_relativepath(file_path));
    return;


    return;
    let file_path = "/mnt/Windows10/Users/Nishant/Desktop/My made programs/Projects/Game Engine/Assets/scenes";

    println!("{}", blue_flame_common::file_path_handling::fullpath_to_relativepath(file_path));
    return;
    //let file_path = "$HOME/Desktop";

    let tmp = blue_flame_common::file_path_handling::fullpath_to_relativepath(file_path);
    let tmp = tmp.as_str();

    println!("tmp: {tmp}");
    /*
    //blue_flame_common::file_path_handling::relativepath_to_fullpath(file_path);

    use std::env;

    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display());
    */
}