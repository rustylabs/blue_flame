// perform actions
pub mod object_actions
{
    use std::collections::HashMap;

    use blue_engine::{header::{Engine, /*ObjectSettings,*/ WindowDescriptor, PowerPreference}, primitive_shapes::{triangle, self, square}, Renderer};
    use crate::{Objects, ObjectSettings};
    // Either puts new shape or changes shape
    pub fn create_shape(object: &(crate::Objects, crate::ObjectSettings), i: usize, renderer: &mut Renderer, gameengine_objects: &mut HashMap<&'static str, blue_engine::header::Object>) -> bool
    {
        //println!("object's name: {}\tobject's status: {}", object.1.object_type[i].name, object.1.object_type[i].status);
        if object.1.object_type[i].name == "Square" && object.1.object_type[i].status == true
        {
            square(std::stringify!(object.0.label.0), blue_engine::header::ObjectSettings::default(), renderer, gameengine_objects).unwrap();
            return true;
        }
        else if object.1.object_type[i].name == "Triangle" && object.1.object_type[i].status == true
        {
            triangle(std::stringify!(object.0.label.0), blue_engine::header::ObjectSettings::default(), renderer, gameengine_objects).unwrap();
            return true;
        }
        else if object.1.object_type[i].name == "Line" && object.1.object_type[i].status == true
        {
            //line(std::stringify!(object.0.label.0), blue_engine::header::ObjectSettings::default(), renderer, gameengine_objects).unwrap();
            return true;
        }
        else
        {
            println!("object's name: {}\tobject's status: {}", object.1.object_type[i].name, object.1.object_type[i].status);
            panic!("Object Type's names are not right in the if statement comparison");
        }
    }
}

// Radio related stuff
pub mod radio_options
{
    pub struct Fields
    {
        pub name            : &'static str,
        pub status          : bool,
    }


    // Triangle, Square etc
    // Clamp, Repeat, Repeat Mirror etc
    pub fn init(names: &'static [&str]) -> Vec<Fields>
    {
        let mut vec = Vec::new();

        for (i, name) in names.iter().enumerate()
        {
            if i == 0
            {
                vec.push(Fields{status: true, name})
            }
            else
            {    
                vec.push(Fields{status: false, name})
            }
            
        }

        vec
    }


    pub fn change_choice(list: &mut [Fields], choice_true: u8)
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            if i as u8 == choice_true
            {
                item.status = true;
            }
            else
            {
                item.status = false;
            }
        }
    }

    // Returns the selected option's index/id
    pub fn enabled_index(list: &[Fields]) -> u8
    {
        let mut index: u8 = 0;
        for (i, item) in list.iter().enumerate()
        {
            if item.status == true
            {
                index = i as u8;
            }
        }
        index
    }
    
}

pub mod three_d_lables
{
    pub struct Fields
    {
        pub axis            : u8, // is this either x, y or z?
        pub value           : f32,
    }
    impl Fields
    {
        pub fn init(value: f32) -> [Self; 3]
        {
            [
                Self{axis: b'x', value},
                Self{axis: b'y', value},
                Self{axis: b'z', value},
            ]
        }
    }
}


pub mod texture
{
    use crate::object_settings::radio_options;

    pub struct Fields
    {
        pub data            : String,
        pub mode            : Vec<radio_options::Fields>,
    }
    impl Fields
    {
        pub fn init() -> Self
        {
            Self
            {
                data            : String::new(),
                mode            : radio_options::init(&["Clamp", "Repeat", "Repeat Mirror"]),
            }
        }
    }
}