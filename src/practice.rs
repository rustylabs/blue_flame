pub fn main()
{
    let file_path = "/home/$USER/Desktop/something";
    blue_flame_common::file_path_handling::fullpath_to_relativepath(file_path);
    //blue_flame_common::file_path_handling::relativepath_to_fullpath(file_path);

    use std::env;

    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display());
}