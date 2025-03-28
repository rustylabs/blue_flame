// Manages all projects and points to scenes
// Blue prints for a particular object's settings such as texture, color etc, essecially saving the flameboject's settings structure

use blue_flame_common::structures::structures::BlueEngineArgs;
pub mod blueprint
{
    use super::*;
    use std::io::Read;
    use blue_flame_common::structures::flameobject;
    use blue_engine::{ObjectStorage, Window, Renderer};
    use crate::filepath_handling;
    

    const VERSION: f32 = 0.1;

    pub fn save(flameobject_blueprint: &Option<flameobject::Settings>, filepath: &str, project_dir: &str)
    {
        match flameobject_blueprint
        {
            Some(value) =>
            {
                let data = postcard::to_stdvec(&(VERSION, value)).unwrap();
                //match std::fs::write(format!("{}/{}", filepath_handling::relativepath_to_fullpath(filepath, project_dir), value.label), &data)
                match std::fs::write(format!("{}", filepath_handling::relativepath_to_fullpath(filepath, project_dir)), &data)
                {
                    Ok(_)               => {println!("blueprints saved!")},
                    Err(e)       => {println!("blueprints save error: {e}")},
                }
            }
            None => {println!("flameobject_blueprint is None, NOT saving!")}
        }
    }

    pub fn load(flameobject_blueprint: &mut Option<flameobject::Settings>, filepath: &str, project_dir: &str, loadshape_2_scene: bool,
        blue_engine_args: &mut BlueEngineArgs, window: &Window)
    {
        let mut file = match std::fs::File::open(format!("{}", filepath_handling::relativepath_to_fullpath(filepath, project_dir)))
        {
            Ok(d) => {println!("blueprints: {} loaded!", filepath); d},
            Err(e) => {println!("Load error on blueprints: {}: {e}", filepath); return},
        };

        let mut data = Vec::new();
        match file.read_to_end(&mut data)
        {
            Ok(_)               => {},
            Err(e)       => println!("read_to_end error {e}"),
        }

        //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
        let value: (f32, flameobject::Settings) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                  => {println!("blueprint error (value) on load: {e}"); return},
        };

        let version = value.0;
        *flameobject_blueprint = Some(value.1);

        if loadshape_2_scene == true
        {
            blue_flame_common::object_actions::create_shape(flameobject_blueprint.as_ref().unwrap(), project_dir, blue_engine_args, window);
        }

        //println!("db version blueprints {FILE_NAME}: {}", version);
    }
}

pub mod projects
{
    use std::io::Read;


    const VERSION: f32 = 0.1;
    const FILE_NAME: &'static str = "projects";


    pub fn save(projects: &[crate::Project], filepath: &crate::FilePaths)
    {
        // This is where we actually save the file

        let data = postcard::to_stdvec(&(VERSION, projects)).unwrap();

        match std::fs::write(format!("{}/{FILE_NAME}", filepath.projects.display()), &data)
        {
            Ok(_)               => println!("Project file saved!"),
            Err(e)       => println!("Save error: {e}"),
        }
    }

    pub fn load(projects: &mut Vec<crate::Project>, filepath: &crate::FilePaths)
    {
        let mut file = match std::fs::File::open(format!("{}/{FILE_NAME}", filepath.projects.display(), ))
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
            Err(e)                   => {println!("Projects (value) error on load: {e}"); return;},
        };

        let version = value.0;
        *projects = value.1;

        println!("db version Project {FILE_NAME}: {}", version);
    }
}

// Scene manager
/*
pub mod scenes
{
    use std::io::Read;
    use blue_flame_common::structures::scene::Scene;

    const VERSION: f32 = 0.1;
    const FILE_NAME: &'static str = "scene_manager";

    pub fn save(scenes: &[Scene], filepath: &crate::FilePaths)
    {
        let data = postcard::to_stdvec(&(VERSION, scenes)).unwrap();

        match std::fs::write(format!("{}/{FILE_NAME}", filepath.scenes.display()), &data)
        {
            Ok(_)               => println!("File saved!"),
            Err(e)       => println!("Save error: {e}"),
        }
    }
    pub fn load(scenes: &mut Vec<Scene>, filepath: &crate::FilePaths)
    {
        let mut file = match std::fs::File::open(format!("{}/{FILE_NAME}", filepath.scenes.display()))
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
        let value: (f32, Vec<Scene>) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                                     => {println!("Error on load: {e}"); return;},
        };

        let version = value.0;
        *scenes = value.1;

        //println!("db version Objects {file_name}: {}", version);
    }


}
*/

pub mod project_config
{
    use std::io::Read;
    use blue_flame_common::structures::structures::project_config::ProjectConfig;
    use blue_flame_common::filepath_handling::*;


    const VERSION: f32 = 0.1;

    pub fn save(project_config: &mut ProjectConfig, filepaths: &mut crate::FilePaths, project_dir: &str,)
    {
        // filepaths.project_config: blue_flame/project.conf
        // filepaths.current_scene: this is the current scene filepath
        project_config.last_scene_filepath = filepaths.current_scene.clone();

        let data = postcard::to_stdvec(&(VERSION, project_config)).unwrap();

        match std::fs::write(format!("{}", relativepath_to_fullpath(filepaths.project_config, project_dir)), &data)
        {
            Ok(_)               => println!("project_config saved!"),
            Err(e)       => println!("project_config save error: {e}"),
        }
    }
    pub fn load(project_config: &mut ProjectConfig, filepaths: &mut crate::FilePaths, project_dir: &str)
    {
        // current_scene
        let mut file = match std::fs::File::open(format!("{}", relativepath_to_fullpath(filepaths.project_config, project_dir)))
        {
            Ok(d)               => {println!("project_config loaded!"); d},
            Err(e)             => {println!("project_config load error: {e}"); return;}
        };

        let mut data = Vec::new();
        match file.read_to_end(&mut data)
        {
            Ok(_)               => {},
            Err(e)       => println!("read_to_end error {e}"),
        }

        //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
        let value: (f32, ProjectConfig) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                    => {println!("project_config error (value) on load: {e}"); return;},
        };

        let version = value.0;
        *project_config = value.1;

        filepaths.current_scene = project_config.last_scene_filepath.clone();
    }
}