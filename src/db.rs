// Manages all projects and points to scenes
pub mod projects
{
    use std::io::Read;


    const VERSION: f32 = 0.1;
    const FILE_NAME: &'static str = "projects";


    pub fn save(projects: &[crate::Projects], file_paths: &crate::FilePaths)
    {

        // This is where we actually save the file

        let data = postcard::to_stdvec(&(VERSION, projects)).unwrap();

        match std::fs::write(format!("{}/{FILE_NAME}", file_paths.projects.display()), &data)
        {
            Ok(_)               => println!("Projects file saved!"),
            Err(e)       => println!("Save error: {e}"),
        }
    }

    pub fn load(projects: &mut Vec<crate::Projects>, file_paths: &crate::FilePaths)
    {
        let mut file = match std::fs::File::open(format!("{}/{FILE_NAME}", file_paths.projects.display(), ))
        {
            Ok(d)               => {println!("Projects: {FILE_NAME} loaded!"); d},
            Err(e)             => {println!("Load projects error on {FILE_NAME} {e}"); return;}
        };

        let mut data = Vec::new();
        match file.read_to_end(&mut data)
        {
            Ok(_)               => {},
            Err(e)       => println!("read_to_end error {e}"),
        }

        //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
        let value: (f32, Vec<crate::Projects>) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                                     => {println!("Error on load: {e}"); return;},
        };

        let version = value.0;
        *projects = value.1;

        println!("db version Projects {FILE_NAME}: {}", version);
    }
}

// Scene manager
pub mod scenes
{
    use std::io::Read;

    const VERSION: f32 = 0.1;
    const SAVE_FOLDER: &'static str = "blue_flame";
    const FILE_NAME: &'static str = "scene_manager";

    pub fn save(scenes: &[(crate::Scenes, crate::SceneSettings)], file_paths: &crate::FilePaths)
    {
        let data = postcard::to_stdvec(&(VERSION, scenes)).unwrap();

        match std::fs::write(format!("{}/{SAVE_FOLDER}/{FILE_NAME}", file_paths.scenes.display()), &data)
        {
            Ok(_)               => println!("File saved!"),
            Err(e)       => println!("Save error: {e}"),
        }
    }
    pub fn load(scenes: &mut Vec<(crate::Scenes, crate::SceneSettings)>, file_paths: &crate::FilePaths)
    {
        let mut file = match std::fs::File::open(format!("{}/{SAVE_FOLDER}/{FILE_NAME}", file_paths.scenes.display()))
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
        let value: (f32, Vec<(crate::Scenes, crate::SceneSettings)>) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                                     => {println!("Error on load: {e}"); return;},
        };

        let version = value.0;
        *scenes = value.1;

        //println!("db version Objects {file_name}: {}", version);
    }


}

// These could be levels, however you want to interpret it as
pub mod objects
{
    //use super::*;
    use std::io::Read;

    const VERSION: f32 = 0.1;
    const SAVE_FOLDER: &'static str = "blue_flame";
    //const FILE_NAME: &'static str = "project_save";

    pub fn save(objects: &[(crate::Objects, crate::ObjectSettings)], file_paths: &crate::FilePaths, file_name: &str)
    {
        let data = postcard::to_stdvec(&(VERSION, objects)).unwrap();
        

        match std::fs::write(format!("{}/{SAVE_FOLDER}/{file_name}", file_paths.scenes.display()), &data)
        {
            Ok(_)               => println!("File saved!"),
            Err(e)       => println!("Save error: {e}"),
        }
    }
    pub fn load(objects: &mut Vec<(crate::Objects, crate::ObjectSettings)>, file_paths: &crate::FilePaths, file_name: &str)
    {
        let mut file = match std::fs::File::open(format!("{}/{SAVE_FOLDER}/{file_name}", file_paths.scenes.display()))
        {
            Ok(d)               => {println!("Objects: {file_name} loaded!"); d},
            Err(e)             => {println!("Load error on {file_name} {e}"); return;}
        };

        let mut data = Vec::new();
        match file.read_to_end(&mut data)
        {
            Ok(_)               => {},
            Err(e)       => println!("read_to_end error {e}"),
        }

        //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
        let value: (f32, Vec<(crate::Objects, crate::ObjectSettings)>) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                                     => {println!("Error on load: {e}"); return;},
        };

        let version = value.0;
        *objects = value.1;

        println!("db version Objects {file_name}: {}", version);
    }
}


// Specifically creates a json file in a specific format for the game engine
pub mod gameengine
{
    
}