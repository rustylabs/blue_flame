// perform actions
/*
pub mod object_actions
{

    use blue_engine::{primitive_shapes::{triangle, square}, Renderer, header, ObjectStorage, Window};
    use blue_flame_common::{Objects, ObjectSettings};
    
    // Either puts new shape or changes shape
    pub fn create_shape(object: &(Objects, ObjectSettings), renderer: &mut Renderer, gameengine_objects: &mut ObjectStorage, window: &Window)
    {
        for (i, shape) in object.1.object_type.iter().enumerate()
        {
            if *shape == true
            {
                match i
                {
                    0       => square(object.0.label.clone(), header::ObjectSettings::default(), renderer, gameengine_objects).unwrap(),
                    1       => triangle(object.0.label.clone(), header::ObjectSettings::default(), renderer, gameengine_objects).unwrap(),
                    2       => println!("todo!: line()"),

                    _       => panic!("Shape number is out of bounds"),
                }
                update_shape(object, gameengine_objects, window, renderer);
            }
        }

        fn update_shape(object: &(Objects, ObjectSettings), gameengine_objects: &mut ObjectStorage, window: &Window, renderer: &mut Renderer)
        {
            update_shape::size(object, gameengine_objects, window);
            update_shape::position(object, gameengine_objects);
            update_shape::color(object, gameengine_objects);
            for (i, rotation) in object.1.rotation.iter().enumerate()
            {
                update_shape::rotation(&object.0.label, crate::mapper::three_d_lables(i), *rotation, gameengine_objects)
            }
            update_shape::texture(object, gameengine_objects, renderer);
            
        }
    }
    // Destroys old hashmap stored in game engine
    pub fn delete_shape(label: &str, gameengine_objects: &mut ObjectStorage)
    {
        gameengine_objects
            .remove(label);
    }
    pub mod update_shape
    {
        use blue_engine::{ObjectStorage, Window, Renderer};
        use blue_flame_common::{Objects, ObjectSettings};

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
        }
        pub fn texture(object: &(Objects, ObjectSettings), gameengine_objects: &mut ObjectStorage, renderer: &mut Renderer)
        {
            //let mut texture_mode: Result<blue_engine::TextureMode, &'static str> = blue_engine::TextureMode::Clamp;
            let mut texture_mode: blue_engine::TextureMode = blue_engine::TextureMode::Clamp;

            for (i, t) in object.1.texture.mode.iter().enumerate()
            {
                if *t == true
                {
                    texture_mode = crate::mapper::texture::enumm(i);
                    break;
                }
            }

            let texture = renderer.build_texture(
                "Main Player",
                //blue_engine::TextureData::Bytes(include_bytes!("/mnt/Windows10/Users/Nishant/Desktop/My made programs/Projects/Game Engine/Example projects/final_test/assets/main_player.png").to_vec()),
                //blue_engine::TextureData::Bytes(std::fs::read(&object.1.texture.file_location).unwrap()),
                blue_engine::TextureData::Bytes(match std::fs::read(&object.1.texture.file_location)
                {
                    Ok(v)       => v,
                    Err(e)               => {println!("TextureData error: {e}"); blue_engine::utils::default_resources::DEFAULT_TEXTURE.to_vec()}
                }),
                    //std::fs::read("/mnt/Windows10/Users/Nishant/Desktop/My made programs/Projects/Game Engine/Example projects/final_test/assets/main_player.png").unwrap()),
                texture_mode,
            );
            
            gameengine_objects
                .get_mut(&object.0.label)
                .unwrap()
                .set_texture(texture.unwrap())
                .unwrap();

        }
    }
}
*/

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