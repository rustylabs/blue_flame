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
    const FILE_NAME: &'static str = "scene_manager";

    pub fn save(scenes: &[(crate::Scenes, crate::SceneSettings)], file_paths: &crate::FilePaths)
    {
        let data = postcard::to_stdvec(&(VERSION, scenes)).unwrap();

        match std::fs::write(format!("{}/{FILE_NAME}", file_paths.scenes.display()), &data)
        {
            Ok(_)               => println!("File saved!"),
            Err(e)       => println!("Save error: {e}"),
        }
    }
    pub fn load(scenes: &mut Vec<(crate::Scenes, crate::SceneSettings)>, file_paths: &crate::FilePaths)
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
    //const SAVE_FOLDER: &'static str = "blue_flame";
    //const FILE_NAME: &'static str = "project_save";

    fn find_selected_scene(scenes: &[(crate::Scenes, crate::SceneSettings)]) -> Option<&crate::Scenes>
    {
        for scene in scenes.iter()
        {
            if scene.0.selected == true
            {
                return Some(&scene.0);
            }
        }
        return None;
    }

    pub fn save(objects: &[(crate::Objects, crate::ObjectSettings)], scenes: &[(crate::Scenes, crate::SceneSettings)] /*Only used to determine object save dir location*/)
    {
        let data = postcard::to_stdvec(&(VERSION, objects)).unwrap();
        let scene = find_selected_scene(scenes);

        let scene = match scene
        {
            Some(v)             => v,
            None                         => {println!("No value"); return},
        };

        match std::fs::write(format!("{}/{}", scene.dir_save, scene.label), &data)
        {
            Ok(_)               => println!("File saved!"),
            Err(e)       => println!("Save error: {e}"),
        }


    }
    pub fn load(objects: &mut Vec<(crate::Objects, crate::ObjectSettings)>, scenes: &[(crate::Scenes, crate::SceneSettings)])
    {
        let scene = find_selected_scene(scenes);

        let scene = match scene
        {
            Some(v)             => v,
            None                         => {println!("No value"); return},
        };

        let mut file = match std::fs::File::open(format!("{}/{}", scene.dir_save, scene.label))
        {
            Ok(d)               => {println!("Objects: {} loaded!", scene.label); d},
            Err(e)             => {println!("Load error on {}: {e}", scene.label); return;}
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

        println!("db version Objects {}: {version}", scene.label);
    }
}


// Specifically creates a json file in a specific format for the game engine
pub mod gameengine
{
    
}