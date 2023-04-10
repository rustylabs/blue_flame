// perform actions
pub mod object_actions
{

    use blue_engine::{primitive_shapes::{triangle, square}, Renderer, header, ObjectStorage, Window};
    use crate::{Objects, ObjectSettings};
    
    // Either puts new shape or changes shape
    pub fn create_shape(object: &(Objects, ObjectSettings), i: usize, renderer: &mut Renderer, gameengine_objects: &mut ObjectStorage, window: &Window) -> bool
    {

        // Square
        if crate::mapper::object_type(i) == "Square" && object.1.object_type[i] == true
        {
            //println!("object.1.size[0].value: {}", object.1.size[0].value);
            square(object.0.label.clone(), header::ObjectSettings::default(), renderer, gameengine_objects).unwrap();
            update_shape(object, gameengine_objects, window);
            
            return true;
        }
        else if crate::mapper::object_type(i) == "Triangle" && object.1.object_type[i] == true
        {
            triangle(object.0.label.clone(), blue_engine::header::ObjectSettings::default(), renderer, gameengine_objects).unwrap();
            update_shape(object, gameengine_objects, window);

            return true;
        }
        else if crate::mapper::object_type(i) == "Line" && object.1.object_type[i] == true
        {
            //line(std::stringify!(object.0.label.0), blue_engine::header::ObjectSettings::default(), renderer, gameengine_objects).unwrap();
            return true;
        }
        else
        {
            //println!("Error on create_shape()");
            //println!("Object's name: {}\t\tObject's type: {}\t\tObject's status: {}", object.0.label.0, object.1.object_type[i].name, object.1.object_type[i].status);
            return false;
            //panic!("Object Type's names are not right in the if statement comparison");
        }

        fn update_shape(object: &(Objects, ObjectSettings), gameengine_objects: &mut ObjectStorage, window: &Window)
        {
            update_shape::size(object, gameengine_objects, window);
            update_shape::position(object, gameengine_objects);
            update_shape::color(object, gameengine_objects);

            for (i, rotation) in object.1.rotation.iter().enumerate()
            {
                update_shape::rotation(&object.0.label, crate::mapper::three_d_lables(i), *rotation, gameengine_objects)
            }
            
        }
    }
    // Destroys old hashmap stored in game engine
    pub fn destroy_hashmap(label_backup: &str, gameengine_objects: &mut ObjectStorage)
    {
        gameengine_objects
            .remove(label_backup);
    }
    pub mod update_shape
    {
        use blue_engine::{ObjectStorage, Window};
        use crate::{Objects, ObjectSettings};

        pub fn size(object: &(Objects, ObjectSettings), gameengine_objects: &mut ObjectStorage, window: &Window)
        {
            gameengine_objects
                .get_mut(&object.0.label)
                .unwrap()
                .resize(object.1.size[0], object.1.size[1], object.1.size[2], window.inner_size());
        }
        pub fn position(object: &(Objects, ObjectSettings), gameengine_objects: &mut ObjectStorage)
        {
            gameengine_objects
                .get_mut(&object.0.label)
                .unwrap()
                .position(object.1.position[0], object.1.position[1], object.1.position[2]);
        }
        pub fn color(object: &(Objects, ObjectSettings), gameengine_objects: &mut ObjectStorage)
        {
            gameengine_objects
                .get_mut(&object.0.label)
                .unwrap()
                .set_uniform_color(object.1.color[0], object.1.color[1], object.1.color[2], object.1.color[3])
                .unwrap();
        }
        pub fn rotation(object_label: &str, axis: u8, rotation: f32, gameengine_objects: &mut ObjectStorage)
        {
            let axis = match axis
            {
                b'x'        => blue_engine::RotateAxis::X,
                b'y'        => blue_engine::RotateAxis::Y,
                b'z'        => blue_engine::RotateAxis::Z,

                _           => panic!(),
            };

            gameengine_objects
                .get_mut(object_label)
                .unwrap()
                .rotate(rotation, axis);
            /*
            gameengine_objects
                .get_mut(&object.0.label.0)
                .unwrap()
                .rotate(10f32, blue_engine::RotateAxis::X)
            */
            
        }
    }
}


// Radio related stuff
pub mod radio_options
{
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Fields
    {
        //pub name            : &'static str,
        pub name            : std::borrow::Cow<'static, str>,
        pub status          : bool,
    }


    // Triangle, Square etc
    // Clamp, Repeat, Repeat Mirror etc
    //pub fn init(names: &'static [&str]) -> Vec<Fields>
    /*
    pub fn init(values: &[&mut bool])
    {
        for (i, value) in values.iter_mut().enumerate()
        {
            if i == 0
            {
                **value = true;
            }
            else
            {
                **value = false;
            }
        }
    }
    */
    


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
    //#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
    //use crate::object_settings::radio_options;

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Fields
    {
        pub file_location   : String,
        pub mode            : [bool; 3]
    }
    impl Fields
    {
        pub fn init() -> Self
        {
            Self
            {
                file_location   : String::new(),
                mode            : [true /*Clamp*/, false /*Triangle*/, false /*Line*/],
            }
        }
    }
}