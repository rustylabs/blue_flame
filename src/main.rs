use blue_engine::{header::{Engine, Renderer, ObjectStorage, /*ObjectSettings,*/ WindowDescriptor, PowerPreference}, Window, VirtualKeyCode};
use blue_engine_egui::{self, egui::{self, Context, Response, Ui}};
use blue_flame_common::{db::scene, emojis::Emojis, filepath_handling, structures::{flameobject::{self, Flameobject}, project_config::ProjectConfig, scene::Scene, BlueEngineArgs, FileExplorerContent, FilePaths, GameEditorArgs, MouseFunctions, Project, WhatChanged, WidgetFunctions, WindowSize}};
use blue_flame_common::radio_options::{ViewModes, object_type::ObjectType, ObjectMouseMovement};
use blue_flame_common::undo_redo;
use blue_flame_common::structures::StringBackups;
use editor_mode_variables::EditorMode;
use rfd::FileDialog;
use serde::de::value;
use std::process::exit;
use blue_flame_common::EditorSettings;
use blue_flame_common::radio_options;
use blue_flame_common::FileExtensionNames;

mod object_settings;
mod db;
mod practice;

mod editor_modes;


// Directory related libraries
use std::path::{Path, PathBuf};
use dirs;


// Generic way to create either flameobject or blueprint
struct CreateNewFlameObject;
impl CreateNewFlameObject
{
    pub fn flameobject(object_type_captured: Option<&ObjectType>, scene: &mut Scene, widget_functions: &mut WidgetFunctions, string_backups: &mut StringBackups, project_dir: &str,
        editor_settings: &EditorSettings, blue_engine_args: &mut BlueEngineArgs, window: &Window, blueprint: Option<&flameobject::Settings>)
    {
        let len = scene.flameobjects.len() as u16;
        //let id = Flameobject::get_available_id(&mut scene.flameobjects);
        let id = scene.flameobject_highest_id;
        scene.flameobject_highest_id += 1;
        //println!("id: {}", id);

        match blueprint
        {
            // Is blueprint
            Some(value) =>
            {
                scene.flameobjects.push(Flameobject::init(id, None));
                scene.flameobjects[len as usize].settings = value.clone();
                scene.flameobjects[len as usize].settings.blueprint_key = Some((String::from(format!("{}", value.label)), true));
            }
            // If new object then do regular
            None => scene.flameobjects.push(Flameobject::init(id, Some(*object_type_captured.unwrap()))),
        }

        // Check if label is the same then change label name
        loop
        {
            let flameobject_label = scene.flameobjects[len as usize].settings.label.clone();
            let mut change_label = false;
            for (i, flameobject) in scene.flameobjects.iter_mut().enumerate()
            {
                // Skip to compare the last object we just created
                if i == len as usize
                {
                    continue;
                }

                // Change label
                if flameobject.settings.label == flameobject_label
                {
                    change_label = true;
                    //flameobject.settings.label.push_str("1");
                    break;
                }
            }

            // Change label
            if change_label == true
            {
                scene.flameobjects[len as usize].settings.label.push_str("1");
            }
            // Nothing to change break out of loop
            else
            {
                break;
            }
        }
        
        Flameobject::change_choice(&mut scene.flameobjects, len);
        scene.flameobject_selected_parent_idx = scene.flameobjects.len() as u16 - 1;
        scene.undo_redo.save_action(
            undo_redo::Action::Create(
                (
                    scene.flameobjects[scene.flameobject_selected_parent_idx as usize].settings.object_type,
                    scene.flameobjects[scene.flameobject_selected_parent_idx as usize].id)),
                    editor_settings);
        blue_flame_common::object_actions::create_shape(&scene.flameobjects[scene.flameobject_selected_parent_idx as usize].settings, project_dir, blue_engine_args, window);
        string_backups.label = scene.flameobjects[scene.flameobject_selected_parent_idx as usize].settings.label.clone();
        /*
        for (i, flameobject) in scene.flameobjects.iter().enumerate()
        {
            if flameobject.selected == true
            {
                scene.flameobject_selected_parent_idx = i as u16;
                blue_flame_common::object_actions::create_shape(&flameobject.settings, project_dir, renderer, objects, window);
            }
        }
        */

        if scene.flameobjects.len() > 0
        {
            widget_functions.flameobject_old = Some(scene.flameobjects[scene.flameobjects.len() - 1].settings.clone());
        }
        else
        {
            widget_functions.flameobject_old = None;
        }
    }
    // This is to create the blueprint in the blueprint tab, not in the objects view tab
    pub fn blueprint(object_type_captured: &ObjectType, flameobject_blueprint: &mut Option<flameobject::Settings>, project_dir: &str,
        blue_engine_args: &mut BlueEngineArgs, window: &Window)
    {
        //flameobject_blueprint = Some(Flameobject::init(len, Some(*object_type_captured)));
        *flameobject_blueprint = Some(flameobject::Settings::init(0, Some(*object_type_captured)));
        blue_flame_common::object_actions::create_shape(flameobject_blueprint.as_ref().unwrap(), project_dir, blue_engine_args, window);
    }
}


trait VecExtensions
{
    fn return_selected_dir(&self) -> Option<&String>;
    fn change_choice(&mut self, choice_true: u16);
}

impl VecExtensions for Vec<Project>
{
    fn return_selected_dir(&self) -> Option<&String>
    {
        for list in self.iter()
        {
            if list.status == true
            {
                return Some(&list.dir);
            }
        }
        return None;
    }
    fn change_choice(&mut self, choice_true: u16)
    {
        for (i, item) in self.iter_mut().enumerate()
        {
            if i as u16 == choice_true
            {
                item.status = true;
                //*current_project_dir = item.dir.clone();
            }
            else
            {
                item.status = false;
            }
        }
    }
    
}

pub struct AlertWindow
{
    label       : String,
    state       : bool,
}
impl AlertWindow
{
    fn init(emojis: &Emojis) -> [Self; 6]
    {
        [
            Self{label: "Open".to_string(), state: false},
            Self{label: "New".to_string(), state: false},
            Self{label: format!("{} Save", emojis.save), state: false},
            Self{label: "Export settings".to_string(), state: false},
            Self{label: "Import settings".to_string(), state: false},
            Self{label: format!("{} Settings", emojis.settings), state: false},
        ]
    }

    fn whats_enabled(alert_window: &[Self]) -> &str
    {
        for list in alert_window.iter()
        {
            if list.state == true
            {
                return &list.label;
            }
        }

        return "";
    }
}


// Stores all the projects you are working on


// For flameobjects


mod issues
{
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Issues
    {
        pub warning         : bool,
        pub error           : bool,
    }
    impl Issues
    {
        pub fn init() -> Self
        {
            Self
            {
                warning     : false,
                error       : false,
            }
        }
    }

    pub fn output_symbols() -> (&'static str, &'static str)
    {
        ("⚠", "⛔")
    }

    /*
    pub mod issue_checks
    {
        pub fn labels(flameobjects: &mut [(crate::Objects, crate::ObjectSettings)])
        {
            if flameobjects.len() == 1
            {
                flameobjects[0].0.label.1.error = false;
                return;
            }
            for i in 0..flameobjects.len()
            {
                for j in 0..flameobjects.len()
                {
                    if i != j
                    {
                        if flameobjects[i].0.label.1.error != true && flameobjects[i].0.label.0 == flameobjects[j].0.label.0
                        {
                            flameobjects[i].0.label.1.error = true;
                            break;
                        }
                        else
                        {
                            flameobjects[i].0.label.1.error = false;    
                        }
                    }
                }
            }
        }
    }
    */
}

struct Debug
{
    practice            : bool,
    resolution          : bool,
}



// Creates library dir if it does not exist and also statically builds src dir into binary executable
/*
fn init_lib(lib_path: &PathBuf)
{
    match
        Command::new("sh")
        .arg("-c")
        .arg(format!("cargo new \"{}\" --lib", lib_path.to_str().unwrap()))
        //.arg("cargo new \"../testing\" --bin")
        .output()
        {
            Ok(_)              => println!("Created lib dir"),
            Err(e)      => println!("Unable to create lib dir: {e}"),
        }
}
*/
// Is it horrizontal or vertical key press movement
enum KeyMovement
{
    Vertical,
    Horizontal,
}
// Based on key presses should we return 1 or -1
fn move_direction_keys(key_movement: KeyMovement, input: &blue_engine::InputHelper) -> i8
{
    let mut move_direction: i8 = 0;

    match key_movement
    {
        KeyMovement::Vertical =>
        {
            //if ui.input(|i| i.key_pressed(egui::Key::ArrowDown))
            if input.key_pressed_os(VirtualKeyCode::Down)
            {
                move_direction = 1;
            }
            else if input.key_pressed_os(VirtualKeyCode::Up)
            {
                move_direction = -1;
            }
        }
        KeyMovement::Horizontal =>
        {
            if input.key_pressed_os(VirtualKeyCode::Right)
            {
                move_direction = 1;
            }
            else if input.key_pressed_os(VirtualKeyCode::Left)
            {
                move_direction = -1;
            }
        }
    }

    return move_direction;
}
// Converts from fullpath to relativepath and vice versa
//fn invert_pathtype(filepath: &str, projects: &Vec<Project>) -> String
fn invert_pathtype(filepath: &str, current_project_dir: &str) -> String
{
    let mut newpath = String::new();
    //let mut filepath = String::from(format!("{filepath}"));

    // Convert from relativepath to fullpath
    if Path::is_relative(&PathBuf::from(format!("{filepath}"))) == true
    {
        newpath = blue_flame_common::filepath_handling::relativepath_to_fullpath(filepath, &current_project_dir);
    }
    // Convert from fullpath to relativepath
    else if Path::is_relative(&PathBuf::from(format!("{filepath}"))) == false
    {
        newpath = blue_flame_common::filepath_handling::fullpath_to_relativepath(filepath, &current_project_dir);
    }
    /*
    for project in projects.iter()
    {
        if project.status == true
        {
            // Convert from relativepath to fullpath
            if Path::is_relative(&PathBuf::from(format!("{filepath}"))) == true
            {
                newpath = blue_flame_common::filepath_handling::relativepath_to_fullpath(filepath, &project.dir);
            }
            // Convert from fullpath to relativepath
            else if Path::is_relative(&PathBuf::from(format!("{filepath}"))) == false
            {
                newpath = blue_flame_common::filepath_handling::fullpath_to_relativepath(filepath, &project.dir);
            }
        }
    }
    */
    return newpath.to_string();
}


// Has radio buttons and creates flameobject
fn object_management(flameobject_settings: &mut flameobject::Settings, projects: &mut [Project], blue_engine_args: &mut BlueEngineArgs, ui: &mut Ui, window: &Window)
{
    use blue_flame_common::radio_options::object_type::{shape, light};

    let mut change_shape = false;

    match flameobject_settings.object_type
    {
        ObjectType::Empty => {},
        ObjectType::Shape(ref mut dimension) => match dimension
        {
            shape::Dimension::D2(ref mut shape) =>
            {
                for (element, label) in shape::Shape2D::elements()
                {
                    if ui.radio_value(shape, element, label).changed()
                    {
                        change_shape = true;
                    }
                }
            }
            shape::Dimension::D3(ref mut shape) => match shape
            {
                shape::Shape3D::Cube => {},
            }
        }
        ObjectType::Light(ref mut light) => match light
        {
            light::Light::Direction => {},
        }
    }
    if change_shape == true
    {
        blue_flame_common::object_actions::create_shape(&flameobject_settings, &Project::selected_dir(&projects), blue_engine_args, window);
    }
    
}

// Used for either loading already existing project or a brand new project
fn load_project_scene(is_loaded: bool, scene: &mut Scene, projects: &mut [Project], game_editor_args: &mut GameEditorArgs, blue_engine_args: &mut BlueEngineArgs, window: &Window)
{
    // If we have already loaded up the project just load the stuff from memory rather than db
    if is_loaded == true
    {
        for (i, flameobject) in scene.flameobjects.iter().enumerate()
        {
            if flameobject.selected == true
            {
                scene.flameobject_selected_parent_idx = i as u16;
            }
            blue_flame_common::object_actions::create_shape(&flameobject.settings, &Project::selected_dir(projects), blue_engine_args, window);
        }
        return;
    }

    let projects_len = projects.len() - 1;

    //Project::change_choice(projects, projects_len);
    for (i, project) in projects.iter_mut().enumerate()
    {
        if i == projects_len
        {
            project.status = true;
            *game_editor_args.current_project_dir = project.dir.clone();
        }
        else
        {
            project.status = false;
        }
    }

    db::projects::save(projects, game_editor_args.filepaths);



    for project in projects.iter()
    {
        if project.status == true
        {
            //filepaths.scene.push(format!("{}", project.dir));

            // Creates blue_flame dir in project_dir
            match std::fs::create_dir(format!("{}/blue_flame", project.dir))
            {
                Ok(_)              => println!("Config dir for project created succesfully in {}/blue_flame", project.dir),
                Err(e)      => println!("Unable to create config dir for project due to: {e}"),
            }

            //filepaths.scene.push("blue_flame");
            //filepaths.create_project_config(); // Creates blue_flame dir in project dir

            // Changing editor mode
            /*
            game_editor_args.editor_modes.projects.0 = false;
            game_editor_args.editor_modes.projects.1 = false;
            game_editor_args.editor_modes.main.0 = true;
            */

            db::project_config::load(game_editor_args.project_config, game_editor_args.filepaths, game_editor_args.current_project_dir);

            //db::scenes::load(scenes, filepaths);
            blue_flame_common::db::scene::load(scene, &Project::selected_dir(&projects),
            &game_editor_args.project_config.last_scene_filepath , true, blue_engine_args, window);

            // Matching the length size issue for undoredo
            {

            }

            // If this is a new project we just created
            /*
            if is_new_project == true
            {
                loaded_scene.scene.dir_save = String::from(format!("{}",
                filepath_handling::fullpath_to_relativepath(&filepaths.scenes.display().to_string(), current_project_dir)));
            }
            */

            for (i, flameobject) in scene.flameobjects.iter().enumerate()
            {
                if flameobject.selected == true {scene.flameobject_selected_parent_idx = i as u16}
            }
            // Determines the current flameobject's name and the puts the name in the label_backup
            for flameobject in scene.flameobjects.iter()
            {
                if flameobject.selected == true
                {
                    game_editor_args.string_backups.label = flameobject.settings.label.clone();
                    //println!("label_backup: {}", label_backup);
                    break;
                }
            }
        }
    }

    //widget_functions.flameobject_old = Some(scene.flameobjects[scene.flameobject_selected_parent_idx as usize].settings.clone());
    if scene.flameobjects.len() > 0
    {
        game_editor_args.widget_functions.flameobject_old = Some(scene.flameobjects[scene.flameobject_selected_parent_idx as usize].settings.clone());
    }
    else
    {
        game_editor_args.widget_functions.flameobject_old = None;
    }
}


// For example show a popup window to create new project etc
pub mod editor_mode_variables
{
    //use super::*;
    pub enum EditorMode{Project(Project), Main(main::Main)}
    pub mod main
    {
        pub struct Main
        {
            pub create_new_object_window: bool,
            pub file_explorer: FileExplorer,
        }
        impl Main
        {
            pub fn init() -> Self
            {
                Self
                {
                    file_explorer: FileExplorer::init(),
                    create_new_object_window: false,
                }
            }
        }

        pub struct FileExplorer
        {
            pub show_rightclick_menu: bool,
            pub show_newfolder_wind: bool,
            pub show_deleteitem_wind: bool,
            pub new_folder_name: String,
        }
        impl FileExplorer
        {
            fn init() -> Self
            {
                Self
                {
                    show_rightclick_menu: false,
                    show_newfolder_wind: false,
                    show_deleteitem_wind: false,
                    new_folder_name: String::new(),
                }
            }
        }
    }

    pub struct Project
    {
        pub new_project_window: bool,
        pub new_project_label: String,
        pub selected_project_before_new: Option<String>,
        pub create_new_project_with_cargo_new: bool,
        pub del_proj_win: bool,
        pub del_entire_proj_checkbox: bool,
    }
    impl Project
    {
        pub fn init() -> Self
        {
            Self
            {
                new_project_window: false,
                new_project_label: String::new(),
                selected_project_before_new: None,
                create_new_project_with_cargo_new: true,
                del_proj_win: false,
                del_entire_proj_checkbox: true,
            }
        }
    }

}

// Flameobject blueprint stuff
pub struct Blueprint
{
    flameobject: Option<flameobject::Settings>,
    save_file_path: String,
}

pub const FILE_EXTENSION_NAMES: FileExtensionNames = FileExtensionNames
{
    scene: ".bsce",
    blueprint: ".bprint",
};

//const FLAMEOBJECT_BLUEPRINT_LABEL: &'static str = "FLAMEOBJECT_BLUEPRINT";
fn main()
{
    let mut file_explorer_contents: (bool, Option<Vec<FileExplorerContent>>) = (false, None);

    //use editor_mode_variables::EditorMode;

    const DEBUG: Debug = Debug
    {
        practice        : false,
        resolution      : true,
    };

    if DEBUG.practice == true
    {
        println!("\n--------------Practice Mode!!!!!--------------\n");
        practice::main();
        println!("\n--------------End of practice!!!!!--------------\n");
        exit(0);
    }

    let emojis = Emojis::init();
    let mut filepaths: FilePaths = FilePaths::init();

    //editor_modes::main::test();
    
    //let mut widget_opened = false;


    //let mut blueprint_savefolderpath = String::new();

    // So that I don't have to keep finding out what is the current project dir
    let mut current_project_dir = String::new();

    // If we are typing in text fields we do not want to enable shortcuts such as select all 'a' and delete 'x'
    let mut enable_shortcuts = true;

    // 0 not clicked, 1 clicked, 2 sub menu
    let mut mouse_functions = MouseFunctions
    {
        is_right_clicked: false,
        object_type_captured: None,
        captured_coordinates: (0f32, 0f32),
        object_mouse_movement: None,
    };

    // Used for flameobject's blueprints
    let mut blueprint = Blueprint
    {
        flameobject: None,
        save_file_path: String::new(),
    };
    //let mut flameobject_blueprint: Option<flameobject::Settings> = None;

    // Creates lib dir
    //init_lib(&filepaths.library);

    // flameobject's previous label just before it is modified
    //let mut label_backup = String::new();
    let mut string_backups = StringBackups{label: String::new(), texture: String::new()};

    // Show load screen or main game screen?
    /*
    let mut editor_modes = EditorModes
    {
        projects: (true, false /*Create new project*/,
            (true /*Create new project with "cargo new"*/, String::new() /*Dir name <project_name>*/),
            (false /*Window for delete project*/, true /*Delete entire project dir*/),
        ),
        //main: (false, ViewModes::Objects, false /*Create new object window*/),
        main: (false, false /*Create new object window*/),
    };
    */
    let mut viewmode = ViewModes::Objects;
    let mut previous_viewmode = viewmode.clone();
    //let mut previous_viewmode = editor_modes.main.1.clone();

    // i.e. Are we in projects or main scene mode?
    let mut editor_mode = EditorMode::Project(editor_mode_variables::Project::init());


    let editor_settings = EditorSettings::init();

    //let mut view_modes = object_settings::radio_options::init(&["Objects", "Scenes"]);
    //let mut view_modes = [true, false];


    
    let mut alert_window = AlertWindow::init(&emojis);

    let mut engine = Engine::new_config(
        WindowDescriptor
        {
            width               : if DEBUG.resolution == true {1280} else {1920},
            height              : if DEBUG.resolution == true {720} else {1080},
            title               : "Blue Flame",
            decorations         : true,
            resizable           : true,
            power_preference    : PowerPreference::LowPower,
            backends            : blue_engine::Backends::VULKAN,
        }).unwrap();


    // DB Variables

    // flameobjects & scenes
    let mut scene = Scene::init(0);
    let mut widget_functions = WidgetFunctions{has_changed: None, flameobject_old: None};
    //let mut flameobjects: Vec<Flameobject> = Vec::new();
    //let mut scenes: Vec<Scene> = Vec::new();
    let mut projects: Vec<Project> = Vec::new();
    let mut project_config = ProjectConfig::init();




    // Load all dbs into memory
    db::projects::load(&mut projects, &filepaths);
    //db::scenes::load(&mut scenes);

    


    // Start the egui context
    let gui_context = blue_engine_egui::EGUI::new(&engine.event_loop, &mut engine.renderer);

    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.plugins.push(Box::new(gui_context));

    // init selected_project_before_new
    if let EditorMode::Project(ref mut sub_editor_mode) = editor_mode
    {
        for project in projects.iter_mut()
        {
            if project.status == true
            {
                sub_editor_mode.selected_project_before_new = Some(project.dir.clone());
                //project.status = false;
                break;
            }
        }
    }



    println!("----------Start of update_loop----------");
    engine.update_loop(move
    |
        renderer,
        window,
        objects,
        input,
        _,
        plugins
    |
    {

        /*
        let mut powerobject = view_modes_argument_passer::Projects
        {
            scene: &mut scene,
        };
        */

        let window_size = WindowSize::init(window);

        // Label error checking
        //issues::issue_checks::labels(&mut flameobjects);

        // obtain the plugin
        let egui_plugin = plugins[0]
        // downcast it to obtain the plugin
        .downcast_mut::<blue_engine_egui::EGUI>()
        .expect("Plugin not found");


        // ui function will provide the context
        egui_plugin.ui(|ctx|
        {
            let mut blue_engine_args = BlueEngineArgs
            {
                renderer,
                //window: window,
                objects,
                input,
                ctx,
            };

            enable_shortcuts = true;

            let mut game_editor_args = GameEditorArgs
            {
                filepaths: &mut filepaths,
                string_backups: &mut string_backups,
                emojis: &emojis,
                widget_functions: &mut widget_functions,
                project_config: &mut project_config,
                current_project_dir: &mut current_project_dir,
                file_explorer_contents: &mut file_explorer_contents,
                //editor_modes: &mut editor_modes,
                window_size: &window_size,
                viewmode: &mut viewmode,
                previous_viewmode: &mut previous_viewmode,
                mouse_functions: &mut mouse_functions,
                enable_shortcuts: &mut enable_shortcuts,
            };

            
            // Load project scene
            //if game_editor_args.editor_modes.projects.0 == true
            if let EditorMode::Project(ref mut sub_editor_mode) = editor_mode
            {

                if editor_modes::projects::main(&mut scene, &mut projects, sub_editor_mode, &mut game_editor_args, &mut blue_engine_args, window) == true
                {
                    editor_mode = EditorMode::Main(editor_mode_variables::main::Main::init());
                }
                /*
                editor_modes::projects::main(&mut scene, &mut projects, &mut filepaths, &mut string_backups, &emojis, &mut widget_functions, &mut project_config,
                    &mut current_project_dir, &mut editor_modes, &window_size, &mut blue_engine_args, window);
                */
                /*
                editor_modes::projects::main_scene(&mut powerobject, &mut projects, &mut filepaths, &mut string_backups, &emojis, &mut widget_functions, &mut project_config,
                    &mut current_project_dir, &mut editor_modes, &window_size, input, ctx, renderer, objects, window);
                */
            }
            // After passing the projects screen, load the main scene
            //else if editor_modes.main.0 == true
            else if let EditorMode::Main(ref mut sub_editor_mode) = editor_mode
            {
                //struct AlertWindow{alert_window: }
                if editor_modes::main::main::main(
                    &mut scene,
                    &mut projects,
                    &mut blueprint,
                    sub_editor_mode,
                    &editor_settings,
                    &mut game_editor_args,
                    &mut alert_window,
                    &mut blue_engine_args,
                    window)
                == true
                {
                    editor_mode = EditorMode::Project(editor_mode_variables::Project::init());
                }
                /*
                editor_modes::main::main::main(
                    &mut alert_window,
                    &mut scene,
                    &mut flameobject_blueprint,
                    &mut previous_viewmode,
                    &mut projects,
                    &mut filepaths,
                    &mut string_backups,
                    &emojis,
                    &mut blueprint_savefolderpath,
                    &mut enable_shortcuts,
                    &editor_settings,
                    &mut widget_functions,
                    &mut project_config,
                    &mut current_project_dir,
                    &mut editor_modes,
                    &window_size,
                    &mut mouse_functions,
                    &mut blue_engine_args,
                    window
                );
                */
            }
        }, window)
    }).unwrap();

}


// Single line edit for directories and contains addtional buttons such as file explorer
fn directory_singleline(filepath_singleline: &mut String, starting_dir: Option<&str>, file_picker_mode: radio_options::FilePickerMode, make_relative: bool, ui: &mut Ui, emojis: &Emojis) -> (Response, bool)
{
    use radio_options::FilePickerMode;
    // bool return is to determine if file dir has been selected
    let mut selected_file = false;

    let mut response: Option<Response> = None;
    ui.horizontal(|ui|
    {
        response = Some(ui.add(egui::TextEdit::singleline(filepath_singleline)));
        if ui.button(format!("{}", emojis.file_icons.folder)).clicked()
        {
            let starting_dir = match starting_dir
            {
                Some(value) => value.to_string(),
                None =>
                {
                    match dirs::home_dir()
                    {
                        Some(value) => value.display().to_string(),
                        None => "/".to_string(),
                    }
                }
            };
            let mut path_picked: Option<PathBuf> = None;

            match file_picker_mode
            {
                FilePickerMode::OpenFolder => path_picked = FileDialog::new().set_directory(&starting_dir).pick_folder(),
                FilePickerMode::OpenFile => path_picked = FileDialog::new().set_directory(&starting_dir).pick_file(),
                FilePickerMode::SaveFile(_) => path_picked = FileDialog::new().set_directory(&starting_dir).save_file(),
            }
            /*
            if is_file == true
            {
                path_picked = FileDialog::new().set_directory(&starting_dir).pick_file();
            }
            else
            {
                path_picked = FileDialog::new().set_directory(&starting_dir).pick_folder();
            }
            */
            match path_picked
            {
                //invert_pathtype(&flameobject_settings.texture.file_location, &projects);
                //Some(value) => *filepath_singleline = value.display().to_string(),
                Some(value) =>
                {
                    selected_file = true;
                    if make_relative == true
                    {
                        *filepath_singleline = invert_pathtype(&value.display().to_string(), &starting_dir);
                    }
                    else
                    {
                        *filepath_singleline = value.display().to_string();
                    }

                    // Add file extension if we are saving file
                    if let FilePickerMode::SaveFile(file_extension) = file_picker_mode
                    {
                        if filepath_singleline.contains(&format!("{}", file_extension)) == false
                        {
                            filepath_singleline.push_str(&format!("{}", file_extension));
                        }
                    }
                },
                None => {},
            }

        }
    });
    return (response.unwrap(), selected_file);
}

// Right click menu for creating new objects
fn right_click_menu(mouse_functions: &mut MouseFunctions, input: &blue_engine::InputHelper, ctx: &egui::Context) -> Option<ObjectType>
{
    let mut create_object: Option<ObjectType> = None;

    // shift + A: Right click menu
    if input.key_held(VirtualKeyCode::LShift) && input.key_pressed(VirtualKeyCode::A)
    {
        mouse_functions.is_right_clicked = true;
        mouse_functions.captured_coordinates = input.mouse().unwrap_or_default();
    }

    // Fucks off the right click menu
    if input.key_pressed(VirtualKeyCode::Escape)
    {
        mouse_functions.is_right_clicked = false;
    }
    if mouse_functions.is_right_clicked == true
    {
        //use blue_flame_common::radio_options::object_type::ObjectType;
        use blue_flame_common::radio_options::object_type::{light, shape};

        //egui::Area::new("right click").fixed_pos(egui::pos2(axis.x, axis.y)).show(ctx, |ui|
        egui::Area::new("right click").fixed_pos(egui::pos2(mouse_functions.captured_coordinates.0, mouse_functions.captured_coordinates.1)).show(ctx, |ui|
        {
            ui.visuals_mut().button_frame = false;
            egui::Frame::menu(&egui::Style::default()).show(ui, |ui|
            {
                // main menu
                for (object_type, label) in ObjectType::elements(None)
                {
                    if ui.button(format!("{}", label)).hovered() {mouse_functions.object_type_captured = Some(object_type)}
                }
                
                // sub menu
                if mouse_functions.object_type_captured != None
                {
                    const SUBMENU_DISTANCE_OFFSET: f32 = 60f32;
                    egui::Area::new("sub menu").fixed_pos(egui::pos2(mouse_functions.captured_coordinates.0 + SUBMENU_DISTANCE_OFFSET, mouse_functions.captured_coordinates.1)).show(ctx, |ui|
                    {
                        ui.visuals_mut().button_frame = false;
                        egui::Frame::menu(&egui::Style::default()).show(ui, |ui|
                        {
                            match mouse_functions.object_type_captured.unwrap()
                            {
                                ObjectType::Light(_) => {},
                                ObjectType::Shape(dimensions) => match dimensions
                                {
                                    shape::Dimension::D2(shape) =>
                                    {
                                        for (shape, label) in shape::Shape2D::elements()
                                        {
                                            if ui.button(format!("{}", label)).clicked()
                                            {
                                                mouse_functions.is_right_clicked = false;
                                                create_object = Some(ObjectType::Shape(shape::Dimension::D2(shape)));
                                            }
                                        }
                                    }
                                    shape::Dimension::D3(_) => {}
                                }
                                ObjectType::Empty => {}
                            }
                            
                        });
            
                    });
                }
            });

        });
    }
    return create_object;
}

mod shortcut_commands
{
    // Determines what keyboard shortcut user has pressed, i.e. shift+A will then call right_click_menu()
    fn shortcut_commands()
    {

    }
    use super::*;
    // Right click menu to add stuff such as object or light etc, pressed by shift + A


}

/*

// Commands such as grab, size object, rotation etc
fn shortcut_commands(scene: &mut Scene, flameobjects_selected_parent_idx: &mut u16, editor_modes: &mut EditorModes, mouse_functions: &mut MouseFunctions,
    project_dir: &str, window_size: &WindowSize,
    /*Game engine shit*/ input: &blue_engine::InputHelper, ctx: &egui::Context, ui: &mut Ui, renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
{
    use blue_flame_common::convert_graphic_2_math_coords;


    //println!("mouse_x: {:?}, mouse_y: ", input.mouse_diff());
    // selectable_label

    //println!("x: {}, y: {}", input.mouse().unwrap_or_default().0, input.mouse().unwrap_or_default().1);

    // Undo (ctrl + Z)
    if input.key_held(VirtualKeyCode::LControl) && input.key_pressed(VirtualKeyCode::Z)
    {

    }
    /*
    // Right click menu (shift + A)
    if input.key_held(VirtualKeyCode::LShift) && input.key_pressed(VirtualKeyCode::A)
    {
        mouse_functions.is_right_clicked = true;
        mouse_functions.captured_coordinates = input.mouse().unwrap_or_default();
    }
    if mouse_functions.is_right_clicked == true
    {
        //use blue_flame_common::radio_options::object_type::ObjectType;
        use blue_flame_common::radio_options::object_type::{light, shape};

        //egui::Area::new("right click").fixed_pos(egui::pos2(axis.x, axis.y)).show(ctx, |ui|
        egui::Area::new("right click").fixed_pos(egui::pos2(mouse_functions.captured_coordinates.0, mouse_functions.captured_coordinates.1)).show(ctx, |ui|
        {
            ui.visuals_mut().button_frame = false;
            egui::Frame::menu(&egui::Style::default()).show(ui, |ui|
            {
                // main menu
                for (object_type, label) in ObjectType::elements(None)
                {
                    if ui.button(format!("{}", label)).hovered() {mouse_functions.object_type_captured = Some(object_type)}
                }
                
                // sub menu
                if mouse_functions.object_type_captured != None
                {
                    egui::Area::new("sub menu").fixed_pos(egui::pos2(mouse_functions.captured_coordinates.0 + 60f32, mouse_functions.captured_coordinates.1)).show(ctx, |ui|
                    {
                        ui.visuals_mut().button_frame = false;
                        egui::Frame::menu(&egui::Style::default()).show(ui, |ui|
                        {
                            match mouse_functions.object_type_captured.unwrap()
                            {
                                ObjectType::Light(_) => {},
                                ObjectType::Shape(dimensions) => match dimensions
                                {
                                    shape::Dimension::D2(shapes) =>
                                    {
                                        for (shape, label) in shape::Shape2D::elements()
                                        {
                                            if ui.button(format!("{}", label)).clicked()
                                            {
                                                mouse_functions.is_right_clicked = false;

                                                let len = scene.flameobjects.len() as u16;
                                                mouse_functions.object_type_captured = Some(ObjectType::Shape(shape::Dimension::D2(shape)));
                                                scene.flameobjects.push(Flameobject::init(len, Some(mouse_functions.object_type_captured.unwrap())));
                                                Flameobject::change_choice(&mut scene.flameobjects, len);
                                                for (i, flameobject) in scene.flameobjects.iter().enumerate()
                                                {
                                                    if flameobject.selected == true
                                                    {
                                                        scene.flameobject_selected_parent_idx = i as u16;
                                                        blue_flame_common::object_actions::create_shape(&flameobject.settings, project_dir, renderer, objects, window);
                                                    }
                                                }
                                                
                                            }
                                        }
                                    }
                                    shape::Dimension::D3(_) => {}
                                }
                                ObjectType::Empty => {}
                            }
                            
                        });
            
                    });
                }
            });

        });
        ui.visuals_mut().button_frame = true;
    }
    if input.key_pressed_os(VirtualKeyCode::Escape) {mouse_functions.is_right_clicked = false}


    // Deselects every object when pressing alt + A
    //if ui.input(|i| i.key_pressed(egui::Key::A) && i.modifiers.alt)
    if input.key_held(VirtualKeyCode::LAlt) && input.key_pressed(VirtualKeyCode::A)
    {
        for flameobject in scene.flameobjects.iter_mut()
        {
            flameobject.selected = false;
        }
    }

    // Selects all objects when pressing A
    else if !input.key_held(VirtualKeyCode::LShift) && input.key_pressed(VirtualKeyCode::A)
    {
        for flameobject in scene.flameobjects.iter_mut()
        {
            flameobject.selected = true;
        }
    }

    // Do something with mouse objects i.e. grab, rotate, size based on key input
    //else if let mou
    else if input.key_pressed(VirtualKeyCode::G)
    {
        mouse_functions.object_mouse_movement = Some(ObjectMouseMovement::Grab);

        for flameobject in scene.flameobjects.iter()
        {
            if flameobject.selected == true
            {
                let (mouse_x, mouse_y) = convert_graphic_2_math_coords(input.mouse().unwrap_or_default(), window_size.return_tuple());
                mouse_functions.captured_coordinates = (mouse_x - flameobject.settings.position.x, mouse_y - flameobject.settings.position.y); // Being used as differences
                println!("mouse_x: {}", mouse_x);
                //println!("mouse_functions.captured_coordinates: {:?}\n", mouse_functions.captured_coordinates);
            };
        }
    }
    else if input.key_pressed(VirtualKeyCode::S)
    {
        mouse_functions.object_mouse_movement = Some(ObjectMouseMovement::Size);
    }
    else if input.key_pressed(VirtualKeyCode::R)
    {
        mouse_functions.object_mouse_movement = Some(ObjectMouseMovement::Rotation);
    }

    // Terminate any key preses
    else if input.key_pressed(VirtualKeyCode::Escape) {mouse_functions.object_mouse_movement = None}
    match &mouse_functions.object_mouse_movement
    {
        Some(action) =>
        {
            match action
            {
                ObjectMouseMovement::Grab =>
                {
                    for flameobject in scene.flameobjects.iter_mut()
                    {
                        if flameobject.selected == true
                        {

                        }
                    }
                }
                ObjectMouseMovement::Size =>
                {

                }
                ObjectMouseMovement::Rotation =>
                {

                }
            }
        }
        None => {}
    }
    */
}
*/
// Displays shit like rotation, size, position, label etc
/*
fn right_panel_flameobject_settings(flameobject_settings: &mut flameobject::Settings, flameobject_selected_parent_idx: u16, flameobject_id: u16,
    undo_redo: &mut undo_redo::UndoRedo, enable_shortcuts: &mut bool, string_backups: &mut StringBackups,
    current_project_dir: &str,
    projects: &Vec<Project>, editor_settings: &EditorSettings, widget_functions: &mut WidgetFunctions,
    /*Game engine shit*/ ui: &mut Ui, blue_engine_args: &mut BlueEngineArgs, window: &Window)
*/

// Saves texture and saves to undo_redo
fn change_texture(
    flameobject_settings: &flameobject::Settings,
    string_backups: &mut StringBackups,
    widget_functions: &mut WidgetFunctions,
    //game_editor_args: &mut GameEditorArgs,
    undo_redo: &mut undo_redo::UndoRedo,
    flameobject_id: u16,
    editor_settings: &EditorSettings,
)
{

    let mut flameobject_settings_copyover = flameobject_settings.clone();
    flameobject_settings_copyover.texture.file_location = string_backups.texture.clone();
    //println!("flameobject_selected_parent_idx: {}", flameobject_selected_parent_idx);
    undo_redo.save_action(undo_redo::Action::Update((flameobject_settings_copyover.clone(), flameobject_settings.clone(), flameobject_id)), editor_settings);

    widget_functions.flameobject_old = Some(flameobject_settings.clone());

    string_backups.texture = flameobject_settings.texture.file_location.clone();
}

fn right_panel_flameobject_settings(
    flameobject_settings: &mut flameobject::Settings,
    flameobject_selected_parent_idx: u16,
    flameobject_id: u16,
    projects: &Vec<Project>,
    undo_redo: &mut undo_redo::UndoRedo,
    editor_settings: &EditorSettings,
    game_editor_args: &mut GameEditorArgs,
    blue_engine_args: &mut BlueEngineArgs,
    ui: &mut Ui,
    window: &Window,
)
{
    //let mut item_clicked = false;
    /*
    fn create_newobject_labelchange(flameobject_settings: &mut flameobject::Settings, enable_shortcuts: &mut bool, label_backup: &mut String,
        current_project_dir: &str, renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
    {
        *enable_shortcuts = false;
        // Destroys hashmap
        blue_flame_common::object_actions::delete_shape(&label_backup, objects);
        
        // Creates new shape
        //object_management(flameobject, &mut projects, renderer, objects, window, ui);
        blue_flame_common::object_actions::create_shape(flameobject_settings, current_project_dir, renderer, objects, window);

        *label_backup = flameobject_settings.label.clone();
    }
    */
    // Object name
    let objectname_response = ui.add(egui::TextEdit::singleline(&mut flameobject_settings.label));
    //if blue_engine_args.input.mouse_pressed(0) || response.lost_focus()
    if objectname_response.lost_focus()
    {
        //println!("response.lost_focus(): {}", objectname_response.lost_focus());
        // If label has been modified after clicking off the field do something
        if flameobject_settings.label != game_editor_args.string_backups.label
        {
            //println!("response.lost_focus(): {}", response.lost_focus());
            //println!("flameobject_settings.label: {}, game_editor_args.string_backups.label: {}", flameobject_settings.label, game_editor_args.string_backups.label);
            // Save history to undo_redo()
            {
                let mut flameobject_copyover = flameobject_settings.clone();
                flameobject_copyover.label = game_editor_args.string_backups.label.clone();
                undo_redo.save_action(undo_redo::Action::Update((flameobject_copyover.clone(), flameobject_settings.clone(), flameobject_selected_parent_idx)), editor_settings);

                game_editor_args.widget_functions.flameobject_old = Some(flameobject_settings.clone());
            }

            *game_editor_args.enable_shortcuts = false;
            // Destroys hashmap
            blue_flame_common::object_actions::delete_shape(&game_editor_args.string_backups.label, blue_engine_args);
            
            // Creates new shape
            //object_management(flameobject, &mut projects, renderer, objects, window, ui);
            blue_flame_common::object_actions::create_shape(flameobject_settings, game_editor_args.current_project_dir, blue_engine_args, window);
    
            game_editor_args.string_backups.label = flameobject_settings.label.clone();
        }
        //has_focus_label = false;
    }


    // Blueprint label key
    ui.horizontal(|ui|
    {
        ui.label(format!("Blueprint label key: {}", match flameobject_settings.blueprint_key
        {
            Some(ref blueprint_key) => blueprint_key.0.clone(),
            None => "None".to_string(),
        }));
    
        match flameobject_settings.blueprint_key
        {
            Some(ref mut blueprint_key) => 
            {
                ui.checkbox(&mut blueprint_key.1, "Modify?");
            },
            None => {},
        }
    });

    
    ui.separator();
    // Locatin of texture
    ui.label("TextureMode");
    ui.label("Location of Texture");
    //let response = ui.add(egui::TextEdit::singleline(&mut flameobject_settings.texture.file_location));
    let response = directory_singleline(&mut flameobject_settings.texture.file_location,
        Some(game_editor_args.current_project_dir), radio_options::FilePickerMode::OpenFile, true, ui, game_editor_args.emojis);
    if response.0.changed() || response.1 == true
    {
        *game_editor_args.enable_shortcuts = false;
        blue_flame_common::object_actions::update_shape::texture(flameobject_settings, &Project::selected_dir(&projects), blue_engine_args);
    }
    if ui.button("Invert filepath type").clicked()
    {
        flameobject_settings.texture.file_location = invert_pathtype(&flameobject_settings.texture.file_location, &game_editor_args.current_project_dir);
    }

    // Save texture change to undo_redo after clicking off text field
    //if blue_engine_args.input.mouse_pressed(0) || response.0.lost_focus()
    if response.0.lost_focus() || response.1 == true
    {
        //undo_redo.save_action(undo_redo::Action::Update((flameobject_settings.clone(), flameobject_selected_parent_idx)));
        // If label has been modified after clicking off the field do something
        // Save history to undo_redo()
        if flameobject_settings.texture.file_location != game_editor_args.string_backups.texture
        {
            change_texture(flameobject_settings, game_editor_args.string_backups, game_editor_args.widget_functions, undo_redo, flameobject_id, editor_settings)
        }
        /*
        if flameobject_settings.texture.file_location != string_backups.texture
        {
            let mut flameobject_copyover = flameobject_settings.clone();
            flameobject_copyover.texture.file_location = string_backups.texture.clone();
            undo_redo.save_action(undo_redo::Action::Update((flameobject_copyover, flameobject_settings.clone(), flameobject_id)), editor_settings);
        }
        */
    }

    // Radio buttons for texturemodes
    {
        use blue_flame_common::radio_options::Texture;
        //let elements = Texture::elements();

        for element in Texture::elements()
        {
            if ui.radio_value(&mut flameobject_settings.texture.mode, element, Texture::label(&element)).changed()
            {
                blue_flame_common::object_actions::update_shape::texture(flameobject_settings, &Project::selected_dir(&projects), blue_engine_args);
            }
        }
    }


    /*
    for i in 0..flameobject.1.texture.mode.len()
    {
        if ui.radio(flameobject.1.texture.mode[i], blue_flame_common::mapper::texture::label(i)).clicked()
        {
            radio_options::change_choice(&mut flameobject.1.texture.mode, i as u8);
            blue_flame_common::object_actions::update_shape::texture(&flameobject, &Project::selected_dir(&projects), objects, renderer);
        }
    }
    */
    ui.separator();

    ui.label("Color");
    ui.horizontal(|ui|
    {
        let response = ui.color_edit_button_rgba_unmultiplied(&mut flameobject_settings.color);
        /*
        if response.clicked()
        {
            widget_functions.flameobject_old = Some(flameobject_settings.clone());
        }
        */
        if response.changed()
        {
            game_editor_args.widget_functions.has_changed = Some(WhatChanged::Color);
            blue_flame_common::object_actions::update_shape::color(flameobject_settings, blue_engine_args);
        }
        // Save changes to undo_redo
        else if blue_engine_args.input.mouse_released(0) && !response.changed()
        {
            if let Some(WhatChanged::Color) = game_editor_args.widget_functions.has_changed
            {
                println!("lost focus color");
                if let Option::Some(ref value) = game_editor_args.widget_functions.flameobject_old
                {
                    undo_redo.save_action(undo_redo::Action::Update((value.clone(), flameobject_settings.clone(), flameobject_id)), editor_settings);
                    game_editor_args.widget_functions.flameobject_old = Some(flameobject_settings.clone());
                }
                game_editor_args.widget_functions.has_changed = None;
            }

        }
    });
    ui.separator();

    ui.label("Position");
    ui.horizontal(|ui|
    {
        let mut save_2_undoredo = false;
        // Has user moved the shape or not
        let mut update_position = false;
        //let elements = flameobject.settings.position.elements();
        //widget_functions.flameobject_old = Some(flameobject_settings.clone());

        for (value, label) in flameobject_settings.position.elements()
        {
            ui.label(format!("{}:", label as char));

            // Use Response::changed or whatever to determine if the value has been changed
            let response = ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed));
            
            // Dragging and typing
            if response.changed()
            {
                game_editor_args.widget_functions.has_changed = Some(WhatChanged::Position);
                update_position = true;
            }

            // Saving to flameobjects_old
            // Typing
            /*
            if response.gained_focus()
            {
                //println!("response.gained_focus()");
                widget_functions.has_changed = Some(WhatChanged::Position);
            }
            */
            //if response.changed() && input.mouse_released(0) i.e. if it has lost focused/not being changed anymore the value you are done putting in the new value
            if /*Dragging*/ response.drag_released() && !response.gained_focus() || /*Typing*/ response.changed() && blue_engine_args.input.mouse_released(0)
            {
                save_2_undoredo = true;
                if let Some(WhatChanged::Position) = game_editor_args.widget_functions.has_changed
                {
                    //undo_redo.save_action(undo_redo::Action::Update((flameobject_settings, flameobject_selected_parent_idx)));
                    //println!("save position undoredo");
                    //save_2_undoredo = true;
                    game_editor_args.widget_functions.has_changed = None;
                }
                
            }
        }

        // Updates the shape's position if the user has changed its value
        if update_position == true
        {
            blue_flame_common::object_actions::update_shape::position(flameobject_settings, blue_engine_args);
        }
        // Save undo redo
        if save_2_undoredo == true
        {
            if let Option::Some(ref value) = game_editor_args.widget_functions.flameobject_old
            {
                undo_redo.save_action(undo_redo::Action::Update((value.clone(), flameobject_settings.clone(), flameobject_id)), editor_settings);
                game_editor_args.widget_functions.flameobject_old = Some(flameobject_settings.clone());
            }
            game_editor_args.widget_functions.has_changed = None;
        }
        

        
    });
    ui.separator();

    ui.label("Size");
    ui.horizontal(|ui|
    {
        let mut save_2_undoredo = false;
        // Has user moved the shape or not
        let mut update_position = false;
        // Has user moved the shape or not
        let mut update_size = false;
        
        for (value, label) in flameobject_settings.size.elements()
        {
            ui.label(format!("{}:", label as char));

            // Use Response::changed or whatever to determine if the value has been changed
            let response = ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed));
            if response.changed()
            {
                //println!("Changed!");
                //widget_functions.has_changed = Some(WhatChanged::Size);
                update_size = true;
            }
            if /*Dragging*/ response.drag_released() && !response.gained_focus() || /*Typing*/ response.changed() && blue_engine_args.input.mouse_released(0)
            {
                save_2_undoredo = true;
            }
            
        }
        // Updates the shape's size if the user has changed its value
        if update_size == true
        {
            //println!("update_position: {update_position}");
            blue_flame_common::object_actions::update_shape::size(flameobject_settings, blue_engine_args, window);
        }
        // Save undo redo
        if save_2_undoredo == true
        {
            if let Option::Some(ref value) = game_editor_args.widget_functions.flameobject_old
            {
                undo_redo.save_action(undo_redo::Action::Update((value.clone(), flameobject_settings.clone(), flameobject_id)), editor_settings);
                game_editor_args.widget_functions.flameobject_old = Some(flameobject_settings.clone());
            }
            game_editor_args.widget_functions.has_changed = None;
        }
        
    });
    ui.separator();

    ui.label("Rotation");
    ui.horizontal(|ui|
    {
        
        for (value, label) in flameobject_settings.rotation.elements()
        {
            ui.label(format!("{}:", label as char));

            // Use Response::changed or whatever to determine if the value has been changed
            if ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed)).changed()
            {
                /*
                blue_flame_common::object_actions::update_shape::rotation
                (

                )
                */
            }
            
        }
    });
}

fn tab_spaces(tab_spaces_times: u16) -> String
{
    let mut tab_spaces = String::new();

    for _ in 0..tab_spaces_times
    {
        tab_spaces.push(' ');
    }
    return tab_spaces;
}

fn new_object_window(flameobject_settings: &mut flameobject::Settings, projects: &mut [Project], emojis: &Emojis, window_size: &WindowSize,
    ui: &mut Ui,
    blue_engine_args: &mut BlueEngineArgs, window: &Window) -> Option<bool>
{

    // If the user presses either Create, Cancel or does not do anything
    let mut action_button: Option<bool> = None;
    
    egui::Window::new("New Object")
    .fixed_pos(egui::pos2(window_size.x/2f32, window_size.y/2f32))
    .fixed_size(egui::vec2(100f32, 200f32))
    //.open(&mut editor_modes.main.2)
    .show(blue_engine_args.ctx, |ui|
    {
        ui.label("Select object type:");

        for (value, label) in ObjectType::elements(Some(flameobject_settings.object_type))
        {
            ui.selectable_value(&mut flameobject_settings.object_type, value, label);
        }
        // Shortcuts for moving up and down
        let move_direction: i8 = move_direction_keys(KeyMovement::Vertical, blue_engine_args.input);
        if move_direction != 0
        {
            let object_type_len = ObjectType::elements(None).len();
            let mut current_index: usize = 0;
            for (value, _) in ObjectType::elements(Some(flameobject_settings.object_type))
            {
                if flameobject_settings.object_type == value
                {
                    if (current_index == 0 && move_direction == -1) || (current_index == (object_type_len - 1) && move_direction == 1) {break}
                    
                    flameobject_settings.object_type = ObjectType::elements(None)[(current_index as i32 + move_direction as i32) as usize].0;
                    break;
                }
                current_index += 1;
            }
        }
    
        // Shows object type radio buttons i.e. Square, Triangle, Line
        ui.horizontal(|ui|
        {
            object_management(flameobject_settings, projects, blue_engine_args, ui, window);
        });
    
        // Shortcuts for changing radio button options i.e. Square, Triangle, Line etc
        let move_direction: i8 = move_direction_keys(KeyMovement::Horizontal, blue_engine_args.input);
        
        if move_direction != 0
        {
            use blue_flame_common::radio_options::object_type::{light, shape};
    
            match flameobject_settings.object_type
            {
                ObjectType::Light(_) => {},
                ObjectType::Shape(ref mut dimension) => match dimension
                {
                    shape::Dimension::D2(ref mut current_shape) =>
                    {
                        // object_type_child is like the sub category, for example Shape is Square, Triangle, Line
                        let object_type_child_len: usize = shape::Shape2D::elements().len();
                        let mut current_index: usize = 0;
                        for (value, _) in shape::Shape2D::elements()
                        {
                            if *current_shape == value
                            {
                                if (current_index == 0 && move_direction == -1) || (current_index == (object_type_child_len - 1) && move_direction == 1)
                                {break}
                                
                                *current_shape = shape::Shape2D::elements()[(current_index as i32 + move_direction as i32) as usize].0;
                                break;
                            }
                            current_index += 1;
                        }
                    },
                    shape::Dimension::D3(_) => {},
                }
                ObjectType::Empty => {},
            };
    
        }

        // Create or Cancel buttons
        ui.horizontal(|ui|
        {
            if ui.button(format!("{} Cancel", emojis.cancel)).clicked()
            //|| ui.input(|i| i.key_pressed(egui::Key::Escape))
            || blue_engine_args.input.key_pressed(VirtualKeyCode::Escape)
            {
                action_button = Some(false);
            }
            if ui.button(format!("{} Create", emojis.addition.plus)).clicked()
            //|| ui.input(|i| i.key_pressed(egui::Key::Enter))
            || blue_engine_args.input.key_pressed(VirtualKeyCode::Return)
            {
                action_button = Some(true);
            }
        });
    });

    return action_button;

}

// Determines if any flameobject is selected and then returns true or false
fn any_flameobject_selected(flameobjects: &[Flameobject]) -> bool
{
    for flameobject in flameobjects.iter()
    {
        if flameobject.selected == true
        {
            return true;
        }
    }
    return false;
}

fn save_blueprint(flameobject_blueprint: &Option<flameobject::Settings>, folderpath: &str, current_project_dir: &str)
{
    db::blueprint::save(&flameobject_blueprint, folderpath, current_project_dir);
}