// perform actions
pub mod object_actions
{

    use blue_engine::{primitive_shapes::{triangle, square}, Renderer, ShaderSettings, utils, uniform_type::Array4, header, ObjectStorage, Window, Engine};
    use crate::{Objects, ObjectSettings};
    
    // Either puts new shape or changes shape
    pub fn create_shape(object: &(Objects, ObjectSettings), i: usize, renderer: &mut Renderer, gameengine_objects: &mut ObjectStorage, window: &Window) -> bool
    {
        //println!("create_shape() Object's type: {}\t\t Object's status: {}", object.1.object_type[i].name, object.1.object_type[i].status);
        //println!("object's name: {}\tobject's status: {}", object.1.object_type[i].name, object.1.object_type[i].status);
        if object.1.object_type[i].name == "Square" && object.1.object_type[i].status == true
        {
            //println!("object.1.size[0].value: {}", object.1.size[0].value);
            square(object.0.label.0.clone(), header::ObjectSettings::default(), renderer, gameengine_objects).unwrap();
            /*
            square(
                object.0.label.0.clone(),
                header::ObjectSettings
                {
                    size                : (0.5f32, 0.5f32, 0.5f32),
                    //scale               : (object.1.scale[0].value, object.1.scale[1].value, object.1.scale[2].value),
                    scale               : (1f32, 1f32, 1f32),
                    position            : (object.1.position[0].value, object.1.position[1].value, object.1.position[2].value),
                    color               : Array4{data: utils::default_resources::DEFAULT_COLOR},
                    camera_effect       : true,
                    shader_settings     : ShaderSettings::default(),
                },
                renderer,
                gameengine_objects
                ).unwrap();
            */
            update_shape(object, gameengine_objects, window);

            //update_shape(object, gameengine_objects, window);
            
            return true;
        }
        else if object.1.object_type[i].name == "Triangle" && object.1.object_type[i].status == true
        {
            triangle(object.0.label.0.clone(), blue_engine::header::ObjectSettings::default(), renderer, gameengine_objects).unwrap();
            update_shape(object, gameengine_objects, window);
            return true;
        }
        else if object.1.object_type[i].name == "Line" && object.1.object_type[i].status == true
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

            for rotation in object.1.rotation.iter()
            {
                update_shape::rotation(&object.0.label.0, rotation, gameengine_objects)
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
                .get_mut(&object.0.label.0)
                .unwrap()
                .resize(object.1.size[0].value, object.1.size[1].value, object.1.size[2].value, window.inner_size());
        }
        pub fn position(object: &(Objects, ObjectSettings), gameengine_objects: &mut ObjectStorage)
        {
            gameengine_objects
                .get_mut(&object.0.label.0)
                .unwrap()
                .position(object.1.position[0].value, object.1.position[1].value, object.1.position[2].value);
        }
        pub fn color(object: &(Objects, ObjectSettings), gameengine_objects: &mut ObjectStorage)
        {
            gameengine_objects
                .get_mut(&object.0.label.0)
                .unwrap()
                .set_uniform_color(object.1.color[0], object.1.color[1], object.1.color[2], object.1.color[3])
                .unwrap();
        }
        pub fn rotation(object_label: &str, rotation: &crate::object_settings::three_d_lables::Fields, gameengine_objects: &mut ObjectStorage)
        {
            let axis = match rotation.axis
            {
                b'x'        => blue_engine::RotateAxis::X,
                b'y'        => blue_engine::RotateAxis::Y,
                b'z'        => blue_engine::RotateAxis::Z,

                _           => panic!(),
            };

            gameengine_objects
                .get_mut(object_label)
                .unwrap()
                .rotate(rotation.value, axis);
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