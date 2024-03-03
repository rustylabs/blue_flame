struct Dir
{
    pub subdir_level: u16,
    pub is_collapsed: bool,
    pub selected: bool,
    pub actual_content: String,
    pub child_contents: Option<Vec<Dir>>,
}

pub fn main()
{
    let mut dirs = Vec::new();
    dirs.push(Dir
    {
        subdir_level: 0,
        is_collapsed: false,
        selected: false,
        actual_content: String::from("1st"),
        child_contents: Some(vec![Dir
        {
            subdir_level: 1,
            is_collapsed: false,
            selected: false,
            actual_content: String::from("1st child"),
            child_contents: Some(vec![Dir
            {
                subdir_level: 2,
                is_collapsed: false,
                selected: false,
                actual_content: String::from("1st child's child"),
                child_contents: None,
            }]),
        }]),
    });
    dirs.push(Dir
    {
        subdir_level: 0,
        is_collapsed: false,
        selected: false,
        actual_content: String::from("2nd"),
        child_contents: None,
    });
    dirs.push(Dir
    {
        subdir_level: 0,
        is_collapsed: false,
        selected: false,
        actual_content: String::from("3rd"),
        child_contents: Some(vec![Dir
        {
            subdir_level: 1,
            is_collapsed: false,
            selected: false,
            actual_content: String::from("child"),
            child_contents: None,
        }]),
    });

    display(&mut dirs);
}

fn display(dirs: &mut Vec<Dir>)
{
    for dir in dirs.iter_mut()
    {
        println!("subdir_level: {} actual_content: {}", dir.subdir_level, dir.actual_content);
        if let Some(ref mut value) = dir.child_contents
        {
            display(value);
        }
    }
}