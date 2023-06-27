// Manages all projects and points to scenes
pub mod projects
{
    use std::io::Read;


    const VERSION: f32 = 0.1;
    const FILE_NAME: &'static str = "projects";


    pub fn save(projects: &[crate::Project], file_paths: &crate::FilePaths)
    {

        // This is where we actually save the file

        let data = postcard::to_stdvec(&(VERSION, projects)).unwrap();

        match std::fs::write(format!("{}/{FILE_NAME}", file_paths.projects.display()), &data)
        {
            Ok(_)               => println!("Project file saved!"),
            Err(e)       => println!("Save error: {e}"),
        }
    }

    pub fn load(projects: &mut Vec<crate::Project>, file_paths: &crate::FilePaths)
    {
        let mut file = match std::fs::File::open(format!("{}/{FILE_NAME}", file_paths.projects.display(), ))
        {
            Ok(d)               => {println!("Project: {FILE_NAME} loaded!"); d},
            Err(e)             => {println!("Load projects error on {FILE_NAME} {e}"); return;}
        };

        let mut data = Vec::new();
        match file.read_to_end(&mut data)
        {
            Ok(_)               => {},
            Err(e)       => println!("read_to_end error {e}"),
        }

        //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
        let value: (f32, Vec<crate::Project>) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                                     => {println!("Error on load: {e}"); return;},
        };

        let version = value.0;
        *projects = value.1;

        println!("db version Project {FILE_NAME}: {}", version);
    }
}

// Scene manager
pub mod scenes
{
    use std::io::Read;

    const VERSION: f32 = 0.1;
    const FILE_NAME: &'static str = "scene_manager";

    pub fn save(scenes: &[(blue_flame_common::Scene, blue_flame_common::SceneSettings)], file_paths: &crate::FilePaths)
    {
        let data = postcard::to_stdvec(&(VERSION, scenes)).unwrap();

        match std::fs::write(format!("{}/{FILE_NAME}", file_paths.scenes.display()), &data)
        {
            Ok(_)               => println!("File saved!"),
            Err(e)       => println!("Save error: {e}"),
        }
    }
    pub fn load(scenes: &mut Vec<(blue_flame_common::Scene, blue_flame_common::SceneSettings)>, file_paths: &crate::FilePaths)
    {
        let mut file = match std::fs::File::open(format!("{}/{FILE_NAME}", file_paths.scenes.display()))
        {
            Ok(d)               => {println!("Scenes: {FILE_NAME} loaded!"); d},
            Err(e)             => {println!("Load error on {FILE_NAME} {e}"); return;}
        };

        let mut data = Vec::new();
        match file.read_to_end(&mut data)
        {
            Ok(_)               => {},
            Err(e)       => println!("read_to_end error {e}"),
        }

        //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
        let value: (f32, Vec<(blue_flame_common::Scene, blue_flame_common::SceneSettings)>) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                                     => {println!("Error on load: {e}"); return;},
        };

        let version = value.0;
        *scenes = value.1;

        //println!("db version Objects {file_name}: {}", version);
    }


}

/*
// These could be levels, however you want to interpret it as
pub mod objects
{
    //use super::*;
    use std::io::Read;

    const VERSION: f32 = 0.1;
    //const SAVE_FOLDER: &'static str = "blue_flame";
    //const FILE_NAME: &'static str = "project_save";

    // Destroys all shapes from the scene

    mod alter_shapes
    {
        pub fn delete_shapes(objects: &mut Vec<(blue_flame_common::Objects, blue_flame_common::ObjectSettings)>, objects: &mut crate::ObjectStorage)
        {
            // Destroys all shapes from the scene
            for object in objects.iter_mut()
            {
                crate::object_settings::object_actions::delete_shape(&object.0.label, objects);
            }
        }
        pub fn create_shapes(objects: &mut Vec<(blue_flame_common::Objects, blue_flame_common::ObjectSettings)>,
        /*Game engine shit*/    renderer: &mut crate::Renderer, objects: &mut crate::ObjectStorage, window: &crate::Window)
        {
            for object in objects.iter()
            {
                blue_flame_common::object_actions::create_shape(object, renderer, objects, window);
                /*
                for i in 0..object.1.object_type.len()
                {
                    if crate::object_settings::object_actions::create_shape(object, i, renderer, objects, window) == true
                    {
                        break;
                    }
                }
                */
            }
        }
    }


    pub fn save(objects: &[(blue_flame_common::Objects, blue_flame_common::ObjectSettings)], scene: &crate::Scenes /*Only used to determine object save dir location*/)
    {
        let data = postcard::to_stdvec(&(VERSION, objects)).unwrap();

        match std::fs::write(format!("{}/{}", scene.dir_save, scene.label), &data)
        {
            Ok(_)               => println!("File saved!"),
            Err(e)       => println!("Save error: {e}"),
        }

    }
    pub fn load(objects: &mut Vec<(blue_flame_common::Objects, blue_flame_common::ObjectSettings)>, scene: &crate::Scenes,
        /*Game engine shit*/ renderer: &mut crate::Renderer, objects: &mut crate::ObjectStorage, window: &crate::Window)
    {

        let mut file = match std::fs::File::open(format!("{}/{}", scene.dir_save, scene.label))
        {
            Ok(d) =>{println!("Objects: {} loaded!", scene.label); d},
            Err(e) => 
                            {
                                println!("Load error on objects: {}: {e}", scene.label);
                                
                                // Deletes shapes
                                alter_shapes::delete_shapes(objects, objects);
                                
                                // Creates new vector and pushes shit
                                *objects = Vec::new();
                                objects.push((blue_flame_common::Objects::init(0), blue_flame_common::ObjectSettings::init()));

                                // Creates new shapes
                                alter_shapes::create_shapes(objects, renderer, objects, window);
                                
                                return;
                            }
        };

        

        let mut data = Vec::new();
        match file.read_to_end(&mut data)
        {
            Ok(_)               => {},
            Err(e)       => println!("read_to_end error {e}"),
        }

        //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
        let value: (f32, Vec<(blue_flame_common::Objects, blue_flame_common::ObjectSettings)>) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                                     => {println!("Error on load: {e}"); return;},
        };

        // Deletes shapes
        alter_shapes::delete_shapes(objects, objects);

        *objects = Vec::new();


        let version = value.0;
        *objects = value.1;



        // Create all the shapes after loading into memory
        alter_shapes::create_shapes(objects, renderer, objects, window);

        //println!("db version Objects {}: {version}", scene.label);
    }
}
*/


/*
// Specifically creates a json file in a specific format for the user's project
pub mod individual_project
{
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

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Fields
    {
        pub file_location   : String,
        pub mode            : [bool; 3]
    }

    use std::io::Read;

    const VERSION: f32 = 0.1;

    // Ports crate::Objects, crate::ObjectSettings into Objects before saving it to db
    fn port_objects(objects: &[(crate::Objects, crate::ObjectSettings)], individual_project_objects: &mut Vec<Objects>)
    {
        for object in objects.iter()
        {
            individual_project_objects.push(Objects
            {
                label: object.0.label.clone(),

            })
        }
    }


    pub fn save(objects: &[(crate::Objects, crate::ObjectSettings)], scene: &crate::Scenes /*Only used to determine object save dir location*/)
    {
        let mut individual_project_objects: Vec<Objects> = Vec::new();
        /*
        individual_project_objects.push(Objects{label: String::from("Test"),
            object_type: [false, false, false],
            position: [10f32, 20f32, 30f32],
            rotation: [10f32, 20f32, 30f32],
            size: [10f32, 20f32, 30f32],
            texture: Fields{file_location: String::from("test"), mode: [false, false, false]},
            color: [10f32, 20f32, 30f32, 30f32],
        });
        */
        port_objects(&objects, &mut individual_project_objects);

        let data = postcard::to_stdvec(&(VERSION, objects)).unwrap();

        match std::fs::write(format!("{}/{}", scene.dir_save, scene.label), &data)
        {
            Ok(_)               => println!("File saved!"),
            Err(e)       => println!("Save error: {e}"),
        }

    }
}
*/