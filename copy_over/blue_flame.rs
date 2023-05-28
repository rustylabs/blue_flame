pub fn load(file_path: &str,
/*Game engine shit*/ renderer: &mut crate::Renderer, gameengine_objects: &mut crate::ObjectStorage, window: &crate::Window)
{
    use std::io::Read;

    println!("Load function called!");

    mod alter_shapes
    {
        pub fn create_shapes(objects: &mut Vec<(crate::blue_flame::Objects, crate::blue_flame::ObjectSettings)>,
        /*Game engine shit*/    renderer: &mut crate::Renderer, gameengine_objects: &mut crate::ObjectStorage, window: &crate::Window)
        {
            for object in objects.iter()
            {
                for i in 0..object.1.object_type.len()
                {
                    if crate::blue_flame::object_actions::create_shape(object, i, renderer, gameengine_objects, window) == true
                    {
                        break;
                    }
                }
            }
        }
    }

    //let mut objects: Vec<(Objects, ObjectSettings)> = Vec::new();

    let mut file = match std::fs::File::open(format!("{}", file_path))
    {
        Ok(f) => {println!("Objects: {} loaded!", file_path); f},
        Err(e) => {println!("Load error on objects: {}: {e}", file_path); return;}
    };

    
    let mut data = Vec::new();
    match file.read_to_end(&mut data)
    {
        Ok(_)               => {},
        Err(e)       => println!("read_to_end error {e}"),
    }

    //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
    let value: (f32, Vec<(crate::blue_flame::Objects, crate::blue_flame::ObjectSettings)>) = match postcard::from_bytes(&data)
    {
        Ok(d)      => d,
        Err(e)                                     => {println!("Error on load: {e}"); return;},
    };

    let version = value.0;
    let mut objects: Vec<(Objects, ObjectSettings)> = value.1;

    //println!("objects: {:?}", objects);


    // Create all the shapes after loading into memory
    alter_shapes::create_shapes(&mut objects, renderer, gameengine_objects, window);

}

/*
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Objects
{
    label               : String, // "Object 0", "Object 1" etc
    object_type         : [bool; 3],
    position            : [f32; 3],
    size                : [f32; 3],
    rotation            : [f32; 3],
    texture             : Fields,
    color               : [f32; 4],
}
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Objects
{
    id          : u16,
    visible     : bool,
    selected    : bool,
    //label       : (String, issues::Issues),
    label       : String // "Object 0", "Object 1" etc
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ObjectSettings
{
    object_type         : [bool; 3],
    //position            : [object_settings::three_d_lables::Fields; 3],
    position            : [f32; 3],
    size                : [f32; 3],
    rotation            : [f32; 3],
    texture             : Fields,
    //texture             : [String; 3],
    color               : [f32; 4],
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Fields
{
    pub file_location   : String,
    pub mode            : [bool; 3]
}

pub mod object_actions
{

    use blue_engine::{primitive_shapes::{triangle, square}, Renderer, header, ObjectStorage, Window};
    use crate::blue_flame::{Objects, ObjectSettings};
    
    // Either puts new shape or changes shape
    pub fn create_shape(object: &(Objects, ObjectSettings), i: usize, renderer: &mut Renderer, gameengine_objects: &mut ObjectStorage, window: &Window) -> bool
    {

        // Square
        if crate::blue_flame::mapper::object_type(i) == "Square" && object.1.object_type[i] == true
        {
            //println!("object.1.size[0].value: {}", object.1.size[0].value);
            square(object.0.label.clone(), header::ObjectSettings::default(), renderer, gameengine_objects).unwrap();
            update_shape(object, gameengine_objects, window);
            
            return true;
        }
        else if crate::blue_flame::mapper::object_type(i) == "Triangle" && object.1.object_type[i] == true
        {
            triangle(object.0.label.clone(), blue_engine::header::ObjectSettings::default(), renderer, gameengine_objects).unwrap();
            update_shape(object, gameengine_objects, window);

            return true;
        }
        else if crate::blue_flame::mapper::object_type(i) == "Line" && object.1.object_type[i] == true
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
            update_shape::position(object, gameengine_objects);     // Error position gets fucked if enabled!
            update_shape::color(object, gameengine_objects);

            for (i, rotation) in object.1.rotation.iter().enumerate()
            {
                update_shape::rotation(&object.0.label, crate::blue_flame::mapper::three_d_lables(i), *rotation, gameengine_objects)
            }
            
        }
    }
    pub mod update_shape
    {
        use blue_engine::{ObjectStorage, Window};
        use crate::blue_flame::{Objects, ObjectSettings};

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

// Maps numbers with names i.e. 0 => Square etc!
mod mapper
{
    // position means position of array/Vector

    pub fn object_type(position: usize) -> &'static str
    {
        let shapes: &[&'static str] = &["Square", "Triangle", "Line"];
        return shapes[position];
    }
    // x, y, z
    pub fn three_d_lables(position: usize) -> u8
    {
        let axis: [u8; 3] = [b'x', b'y', b'z'];
        return axis[position];
    }
    pub fn texture(position: usize) -> &'static str
    {
        let textures: &[&'static str] = &["Clamp", "Repeat", "Repeat Mirror"];
        return textures[position];
    }
    pub fn view_mode(position: usize) -> &'static str
    {
        //let mut view_modes = object_settings::radio_options::init(&["Objects", "Scenes"]);
        let view_modes = ["Objects", "Scenes"];
        return view_modes[position];
    }
    pub fn game_type(position: usize) -> &'static str
    {
        //let mut view_modes = object_settings::radio_options::init(&["Objects", "Scenes"]);
        let game_types = ["2D", "3D"];
        return game_types[position];
    }
}