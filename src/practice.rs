#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_imports)]
pub fn main()
{
    enum EditorModes
    {
        Projects((bool, &'static str)),
        Main,
    }


    
    let mut editor_modes =
    [
        (true, EditorModes::Projects((false, "Create objects"))), 
        (false, EditorModes::Main),
    ];

}