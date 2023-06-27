use std::vec;

struct Flameobject
{
    id              : u16,
    visible         : bool,
    standard_fields : StandardFields,
    //label       : (String, issues::Issues),
}

struct Scene
{
    id                  : u16,
    dir_save            : String,
    standard_fields     : StandardFields,
}
impl VecExts for Vec<Scene>
{
    fn return_selected_dir(&self) -> Option<&String>
    {
        for item in self.iter()
        {
            if item.standard_fields.is_selected() == true
            {
                return Some(&item.dir_save);
            }
        }
        return None;
    }
}

trait VecExts
{
    fn return_selected_dir(&self) -> Option<&String>;
}

struct StandardFields
{
    selected    : bool,
    label       : String,
}
impl StandardFields
{
    fn is_selected(&self) -> bool
    {
        return self.selected;
    }
}

pub fn main()
{
    let scenes = vec![
        Scene{id: 0, dir_save: String::from("asd"), standard_fields: StandardFields{selected: false, label: String::from("asd")}},
        Scene{id: 0, dir_save: String::from("asdasdadss"), standard_fields: StandardFields{selected: true, label: String::from("asdadsasdasd")}}
    ];

    println!("current scene is: {}", scenes.return_selected_dir().unwrap());
}