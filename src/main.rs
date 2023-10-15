use blue_engine::{header::{Engine, Renderer, ObjectStorage, /*ObjectSettings,*/ WindowDescriptor, PowerPreference}, Window, VirtualKeyCode, MouseButton};
use blue_engine_egui::{self, egui::{self, Ui, InputState}};
use blue_flame_common::{filepath_handling, structures::{flameobject::{Flameobject, self}, scene::Scene, project_config::ProjectConfig}, db::scene, emojis::Emojis};
use blue_flame_common::radio_options::{ViewModes, object_type::ObjectType, ObjectMouseMovement};
use std::{process::Command, io::Write, f32::consts::E};

use std::process::exit;


pub mod object_settings;
pub mod db;
mod practice;

// Directory related libraries
use std::path::{Path, PathBuf};
use dirs;


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
// Defines where all the file paths are
pub struct FilePaths
{
    projects        : PathBuf, // ~/.config/blue_flame/blue_flame_common
    project_config  : &'static str, // blue_flame/project.conf
    current_scene   : String,
    library         : PathBuf,
}
impl FilePaths
{
    fn init() -> Self
    {
        // Creating dirs
        // ~/.config.blue_flame
        let mut projects: PathBuf =  match dirs::home_dir()
        {
            Some(v)         => v,
            None                     => {println!("Unable to obtain home dir"); PathBuf::new()}
        };
        projects.push(".config");
        projects.push("blue_flame");

        println!("config_dir: {:?}", projects);
        match std::fs::create_dir(&projects)
        {
            Ok(_)       => println!("Config dir created succesfully in {}", projects.display()),
            Err(e)      => println!("Unable to create config dir due to {e}"),
        }

        let mut library: PathBuf =  match dirs::home_dir()
        {
            Some(v)         => v,
            None                     => {println!("Unable to obtain home dir"); PathBuf::new()}
        };
        
        library.push(".local/share/blue_flame/blue_flame_common");
        println!("library: {:?}", library);

        let project_config: &'static str = "blue_flame/project.conf";

        Self
        {
            projects,
            project_config,
            current_scene: String::new(),
            library,
        }
    }
    // Creates the folder for the project
    /*
    fn create_project_config(&self)
    {
        match std::fs::create_dir(format!("{}", self.scenes.display()))
        {
            Ok(_)       => println!("Config dir for project created succesfully in {}", self.scenes.display()),
            Err(e)      => println!("Unable to create config dir for project due to: {e}"),
        }
    }
    */
}

struct EditorSettings
{
    width               : f32,
    height              : f32,
    range               : f32,
    slider_speed        : f32,
}
impl EditorSettings
{
    fn init() -> Self
    {
        Self
        {
            width               : 250f32,
            height              : 900f32,
            range               : 900_000_000f32,
            slider_speed        : 0.01f32,
        }
    }
}

struct AlertWindow
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
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Project
{
    name        : String,
    dir         : String,
    game_type   : blue_flame_common::radio_options::GameTypeDimensions,
    status      : bool,
}
impl Project
{
    fn init() -> Self
    {
        Self
        {
            name        : String::new(),
            dir         : String::new(),
            game_type   : blue_flame_common::radio_options::GameTypeDimensions::D2,
            status      : false,
        }
    }
    pub fn change_choice(list: &mut [Self], choice_true: u8)
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
    pub fn selected_dir(list: &[Self]) -> String
    {
        let mut selected_dir = String::new();

        for item in list.iter()
        {
            if item.status == true
            {
                selected_dir.push_str(&format!("{}", item.dir));
                break;
            }
        }
        return selected_dir;
    }
}




struct EditorModes
{
    projects        :   (bool, bool /*"New Project" scene window*/,
                        (bool /*2.0 Create new project with "cargo new" (checkbox)*/, String /*2.1 Label for <project_name>*/),
                        (bool /*3 Window for delete project*/, bool /*Delete entire project dir (checkbox)*/),
                        ),
    //main            : (bool, [bool;2]),
    main            : (bool, ViewModes, bool /*Create new object window*/),
}
    // Declaring variables/structures
pub struct WindowSize
{
    x           : f32,
    y           : f32,
}
impl WindowSize
{
    fn init(window: &Window) -> Self
    {
        Self
        {
            x       : window.inner_size().width as f32,
            y       : window.inner_size().height as f32,
        }
    }
    fn return_tuple(&self) -> (f32, f32)
    {
        return (self.x, self.y);
    }
}

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
fn invert_pathtype(filepath: &str, projects: &Vec<Project>) -> String
{
    let mut newpath = String::new();
    //let mut filepath = String::from(format!("{filepath}"));
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
    return newpath.to_string();
}


// Has radio buttons and creates flameobject
fn object_management(flameobject_settings: &mut flameobject::Settings, projects: &mut [Project], renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window, ui: &mut Ui)
{
    use blue_flame_common::radio_options::object_type::{ObjectType, shape, light};

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
        blue_flame_common::object_actions::create_shape(&flameobject_settings, &Project::selected_dir(&projects), renderer, objects, window);
    }
    
}

// Used for either loading already existing project or a brand new project
fn load_project_scene(is_loaded: bool, scene: &mut Scene, projects: &mut [Project],  filepaths: &mut FilePaths,
    project_config: &mut ProjectConfig, current_project_dir: &mut String, editor_modes: &mut EditorModes, flameobjects_selected_parent_idx: &mut u16,
    /*Engine shit*/ renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window
)
{
    // If we have already loaded up the project just load the stuff from memory rather than db
    if is_loaded == true
    {
        for (i, flameobject) in scene.flameobjects.iter().enumerate()
        {
            if flameobject.selected == true
            {
                *flameobjects_selected_parent_idx = i as u16;
            }
            blue_flame_common::object_actions::create_shape(&flameobject.settings, &Project::selected_dir(&projects), renderer, objects, window);
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
            *current_project_dir = project.dir.clone();
        }
        else
        {
            project.status = false;
        }
    }

    db::projects::save(projects, filepaths);



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
            editor_modes.projects.0 = false;
            editor_modes.projects.1 = false;
            editor_modes.main.0 = true;

            db::project_config::load(project_config, filepaths, current_project_dir);

            //db::scenes::load(scenes, filepaths);
            blue_flame_common::db::scene::load(scene, &Project::selected_dir(&projects),
            &project_config.last_scene_filepath , true, renderer, objects, window);

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
                if flameobject.selected == true {*flameobjects_selected_parent_idx = i as u16}
            }
        }
    }
}

// Invoked via shift+A
struct MouseFunctions
{
    is_right_clicked        : bool, // Has it been right clicked
    object_type_captured    : Option<ObjectType>,
    captured_coordinates    : (f32, f32), // Captures the coordinates at any given time, can be even used with difference between object and mouse
    object_mouse_movement   : Option<ObjectMouseMovement>, // grab to move the object etc
}

//const FLAMEOBJECT_BLUEPRINT_LABEL: &'static str = "FLAMEOBJECT_BLUEPRINT";
fn main()
{

    let emojis = Emojis::init();
    let mut filepaths: FilePaths = FilePaths::init();

    // Which flameobject was selected first and stores its index
    let mut flameobjects_selected_parent_idx: u16 = 0;

    let mut blueprint_savefolderpath = String::new();

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
    let mut flameobject_blueprint: Option<blue_flame_common::structures::flameobject::Settings> = None;

    // Creates lib dir
    //init_lib(&filepaths.library);

    // flameobject's previous label just before it is modified
    let mut label_backup = String::new();

    // Show load screen or main game screen?
    let mut editor_modes = EditorModes
    {
        projects: (true, false /*Create new project*/,
            (true /*Create new project with "cargo new"*/, String::new() /*Dir name <project_name>*/),
            (false /*Window for delete project*/, true /*Delete entire project dir*/),
        ),
        main: (false, ViewModes::Objects,
            false /*Create new object window*/
        ),
    };

    let mut previous_viewmode = editor_modes.main.1.clone();



    let editor_settings = EditorSettings::init();

    //let mut view_modes = object_settings::radio_options::init(&["Objects", "Scenes"]);
    //let mut view_modes = [true, false];

    let debug = Debug
    {
        practice        : false,
        resolution      : true,
    };

    if debug.practice == true
    {
        println!("\n--------------Practice Mode!!!!!--------------\n");
        practice::main();
        println!("\n--------------End of practice!!!!!--------------\n");
        exit(0);
    }
    
    let mut alert_window = (false, AlertWindow::init(&emojis));

    let mut engine = Engine::new_config(
        WindowDescriptor
        {
            width               : if debug.resolution == true {1280} else {1920},
            height              : if debug.resolution == true {720} else {1080},
            title               : "Blue Flame",
            decorations         : true,
            resizable           : true,
            power_preference    : PowerPreference::LowPower,
            backends            : blue_engine::Backends::VULKAN,
        }).unwrap();


    // DB Variables

    // flameobjects & scenes
    let mut scene = Scene::init(0);
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

    // Determines the current flameobject's name and the puts the name in the backup_label
    for flameobject in scene.flameobjects.iter()
    {
        if flameobject.selected == true
        {
            label_backup = flameobject.settings.label.clone();
            //println!("label_backup: {}", label_backup);
            break;
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

        let window_size = WindowSize::init(&window);

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
            enable_shortcuts = true;
            // if true load project scene
            if editor_modes.projects.0 == true
            {
                // Shows all your projects and what you want to load upon startup
                egui::Window::new("Project")
                .collapsible(false)
                .fixed_pos(egui::pos2(0f32, 0f32))
                .fixed_size(egui::vec2(window_size.x, window_size.y))
                //.open(&mut open_projects)
                .show(ctx, |ui|
                {
                    use blue_flame_common::radio_options::GameTypeDimensions;

                    ui.set_width(ui.available_width());
                    ui.set_height(ui.available_height());

                    // Load or Create
                    ui.horizontal(|ui|
                    {
                        if ui.button("Load scene").clicked()
                        //|| ui.input(|i| i.key_pressed(egui::Key::Enter))
                        || (input.key_pressed(VirtualKeyCode::Return) || input.key_pressed(VirtualKeyCode::NumpadEnter))
                        {
                            // Load existing project
                            load_project_scene(false, &mut scene, &mut projects, &mut filepaths, &mut project_config, &mut current_project_dir, &mut editor_modes, &mut flameobjects_selected_parent_idx,
                                renderer, objects, &window);
                        }
                        if ui.button(format!("{} Create/import project", emojis.add)).clicked()
                        {
                            projects.push(Project::init());
                            
                            let len = (projects.len() - 1) as u16;
                            //Project::change_choice(&mut projects, len as u8);
                            projects.change_choice(len);
                            
                            editor_modes.projects.1 = true;
                        }
                        if ui.button(format!("{} Delete project", emojis.trash)).clicked()
                        {
                            editor_modes.projects.3.0 = true;
                        }

                    });

                    // Show all projects
                    for i in 0..projects.len()
                    {
                        // Gets position of what is true in the game_type:[true, false]
                        /*
                        let mut game_type_pos: usize = 0;
                        
                        for (j, game_type) in projects[i].game_type.iter().enumerate()
                        {
                            if *game_type == true
                            {
                                game_type_pos = j;
                            }
                        }
                        */

                        if ui.selectable_label(projects[i].status, format!("{}: {} {}",
                        projects[i].name,
                        projects[i].dir,
                        //GameTypeDimensions::elements(&projects[i].game_type),
                        //blue_flame_common::mapper::game_type(game_type_pos),

                        tab_spaces((window_size.x/4f32) as u16))).clicked()
                        {
                            //Project::change_choice(&mut projects, i as u8);
                            projects.change_choice(i as u16);
                        }
                    }



                    // Shows "New Project" scene
                    if editor_modes.projects.1 == true
                    {
                        egui::Window::new("New Project")
                        .fixed_pos(egui::pos2(window_size.x/2f32, window_size.y/2f32))
                        .pivot(egui::Align2::CENTER_CENTER)
                        .default_size(egui::vec2(window_size.x/2f32, window_size.y/2f32))
                        .resizable(true)
                        //.open(&mut _create_new_project)
                        .show(ctx, |ui|
                        {

                            let len = projects.len() - 1;


                            ui.label("Project name:");
                            ui.add(egui::TextEdit::singleline(&mut projects[len].name));

                            ui.separator();

                            ui.label("Project directory:");
                            ui.add(egui::TextEdit::singleline(&mut projects[len].dir));

                            ui.label("Game type:");

                            // 2D or 3D
                            for project in projects.iter_mut()
                            {
                                //use blue_flame_common::radio_options::GameTypeDimensions;
                                if project.status == true
                                {
                                    //let elements  = GameTypeDimensions::elements();
                                    ui.horizontal(|ui|
                                    {
                                        for (element, label) in GameTypeDimensions::elements()
                                        {
                                            ui.radio_value(&mut project.game_type, element, label);
                                        }
                                    });

                                    /*
                                    for i in 0..project.game_type.len()
                                    {
                                        if ui.radio(project.game_type[i], blue_flame_common::mapper::game_type(i)).clicked()
                                        {
                                            radio_options::change_choice(&mut project.game_type, i as u8);
                                        }
                                    }
                                    */
                                }
                            }
                            ui.checkbox(&mut editor_modes.projects.2.0, "Create new project with command: \"cargo new <project_name> --bin\"");
                            // Shows label to type out the name of <project_name>
                            if editor_modes.projects.2.0 == true
                            {
                                ui.add(egui::TextEdit::singleline(&mut editor_modes.projects.2.1));
                            }

                            // Shows extra buttons
                            ui.horizontal(|ui|
                            {
                                if ui.button(format!("{} Cancel", emojis.cancel)).clicked()
                                {
                                    editor_modes.projects.1 = false;
                                    projects.pop();
                                }

                                if ui.button(format!("{} Create", emojis.add)).clicked()
                                {
                                    // Sets the scene and not flameobject to be true
                                    editor_modes.main.1 = ViewModes::Scenes;

                                    // Determines if to run "cargo new"
                                    if editor_modes.projects.2.0 == true
                                    {
                                        // Runs "cargo new" and adds extra filepaths to appropriate variables
                                        for project in projects.iter_mut()
                                        {
                                            if project.status == true
                                            {

                                                project.dir.push_str(&format!("/{}", editor_modes.projects.2.1));

                                                Command::new("sh")
                                                .arg("-c")
                                                .arg(format!("cargo new \"{}\" --bin", project.dir))
                                                //.arg("cargo new \"../testing\" --bin")
                                                .output()
                                                .unwrap();
                                            }
                                        }
                                    }

                                    // From "copy_over", load into memory, alter some stuff and output it to the project's respective dirs
                                    for project in projects.iter()
                                    {
                                        if project.status == true
                                        {
                                            let dir_src = String::from(format!("{}/src", project.dir));

                                            struct CopyOver
                                            {
                                                main            : &'static str,
                                                blue_flame      : &'static [u8],
                                                cargo           : &'static str,
                                            }
                                            let copy_over = CopyOver
                                            {
                                                main            : include_str!("../copy_over/main.rs"),
                                                blue_flame      : include_bytes!("../copy_over/blue_flame.rs"),
                                                cargo           : include_str!("../copy_over/Cargo.toml"),
                                            };

                                            // main.rs
                                            let mut loaded_content = String::from(copy_over.main);
                                            loaded_content = loaded_content.replace("{project_name}", &project.name);
                                            //loaded_content = loaded_content.replace("{scene_path}", &project.dir);

                                            let mut output_file = std::fs::File::create(format!("{dir_src}/main.rs")).unwrap();
                                            output_file.write_all(loaded_content.as_bytes()).unwrap();

                                            // blue_flame.rs
                                            let loaded_content = copy_over.blue_flame.to_vec();
                                            let mut output_file = std::fs::File::create(format!("{dir_src}/blue_flame.rs")).unwrap();
                                            output_file.write_all(&loaded_content).unwrap();

                                            // Cargo.toml
                                            let mut loaded_content = String::from(copy_over.cargo);
                                            loaded_content = loaded_content.replace("{project_name}", &editor_modes.projects.2.1);
                                            loaded_content = loaded_content.replace("{library}", &filepaths.library.to_str().unwrap());
                                            let mut output_file = std::fs::File::create(format!("{dir_src}/../Cargo.toml")).unwrap();
                                            output_file.write_all(loaded_content.as_bytes()).unwrap();

                                            break;
                                        }
                                    }

                                    // Load new project
                                    load_project_scene(false, &mut scene, &mut projects, &mut filepaths, &mut project_config, &mut current_project_dir, &mut editor_modes, &mut flameobjects_selected_parent_idx,
                                        renderer, objects, &window);
                                }
                            });
                        });
                    }


                    // Delete project
                    if editor_modes.projects.3.0 == true
                    {
                        for (i, project) in projects.iter_mut().enumerate()
                        {
                            if project.status == true
                            {
                                let remove_project_dir = PathBuf::from(format!("{}", project.dir));

                                // Window prompt
                                egui::Window::new("Conformation!")
                                .fixed_pos(egui::pos2(window_size.x/2f32, window_size.y/2f32))
                                .pivot(egui::Align2::CENTER_CENTER)
                                .default_size(egui::vec2(window_size.x/2f32, window_size.y/2f32))
                                .resizable(true)
                                //.open(&mut _create_new_project)
                                .show(ctx, |ui|
                                {
                                    ui.label("Are you sure you want to delete");
                                    ui.checkbox(&mut editor_modes.projects.3.1, "Delete entire project dir");

                                    ui.horizontal(|ui|
                                    {
                                        if ui.button(format!("{} Cancel", emojis.cancel)).clicked()
                                        {
                                            editor_modes.projects.3.0 = false;
                                        }
                                        if ui.button(format!("{} Yes", emojis.tick)).clicked()
                                        {
                                            editor_modes.projects.3.0 = false;

                                            if editor_modes.projects.3.1 == true
                                            {
                                                match std::fs::remove_dir_all(remove_project_dir)
                                                {
                                                    Ok(_) => {},
                                                    Err(e) => println!("Can't remove project: {e}"),
                                                }
                                            }
                                            projects.remove(i);
                                            db::projects::save(&mut projects, &mut filepaths);
                                        }                           
                                    });

                                });
                                break;
                            }
                        }
                    }
                });
            }
            // After passing the projects screen
            else if editor_modes.main.0 == true
            {
                // One of the settings menu if opened
                egui::Window::new(AlertWindow::whats_enabled(&alert_window.1))
                .fixed_pos(egui::pos2(400f32, 50f32))
                .fixed_size(egui::vec2(100f32, 200f32))
                .open(&mut alert_window.0)
                .show(ctx, |ui|
                {
                    ui.label("")
                });


                // Menu bar
                egui::TopBottomPanel::top("Menu Bar").show(ctx, |ui|
                {
                    ui.set_enabled(!alert_window.0);

                    egui::menu::bar(ui, |ui|
                    {
                        ui.menu_button("Menu", |ui|
                        {
                            for list in alert_window.1.iter_mut()
                            {
                                // Individual elements after clicking on "Menu"
                                if ui.button(&list.label).clicked()
                                {
                                    if list.label == "💾 Save"
                                    {
                                        blue_flame_common::db::scene::save(&scene, &filepaths.current_scene, &current_project_dir);
                                        break;
                                    }

                                    alert_window.0 = true;
                                    list.state = true;
                                }
                                else if alert_window.0 == false
                                {
                                    list.state = false;
                                }
                            }

                        });
                        ui.menu_button("About", |ui|
                        {
                            //if ui.bu
                        });

                    });
                });

                // Left panel
                egui::SidePanel::left("Objects").show(ctx, |ui|
                {
                    /*
                    if ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Secondary))
                    {
                        println!("mouse click!");
                    }
                    */

                    // Shortcuts



                    ui.set_enabled(!alert_window.0);

                    ui.set_width(ui.available_width());
    
                    // Shows the current scene we are using
                    ui.horizontal(|ui|
                    {
                        ui.label(format!("Current scene: {}", &scene.label));
                    });

                    ui.separator();

                    // Tabs for other Objects or Scenes view
                    ui.horizontal(|ui|
                    {
                        ui.label("Current display:");

                        //let elements = ViewModes::elements();
                        for (element, label) in ViewModes::elements()
                        {
                            if ui.selectable_value(&mut editor_modes.main.1, element, label).changed()
                            {
                                // Switching between tabs
                                match editor_modes.main.1
                                {
                                    ViewModes::Objects => 
                                    {
                                        if previous_viewmode == ViewModes::Blueprints
                                        {
                                            if let Option::Some(ref value) = flameobject_blueprint
                                            {
                                                blue_flame_common::object_actions::delete_shape(&value.label, objects);
                                            }
                                            load_project_scene(true, &mut scene, &mut projects, &mut filepaths, &mut project_config, &mut current_project_dir, &mut editor_modes, &mut flameobjects_selected_parent_idx,
                                                renderer, objects, &window);
                                        }
                                        previous_viewmode = editor_modes.main.1.clone();
                                    }
                                    ViewModes::Scenes => 
                                    {
                                        if previous_viewmode == ViewModes::Blueprints
                                        {
                                            if let Option::Some(ref value) = flameobject_blueprint
                                            {
                                                blue_flame_common::object_actions::delete_shape(&value.label, objects);
                                            }
                                            load_project_scene(true, &mut scene, &mut projects, &mut filepaths, &mut project_config, &mut current_project_dir, &mut editor_modes, &mut flameobjects_selected_parent_idx,
                                                renderer, objects, &window);
                                        }
                                        previous_viewmode = editor_modes.main.1.clone();
                                    }
                                    ViewModes::Blueprints => 
                                    {
                                        // Remove all objects from scene then load or create a new object for blueprints variable
                                        for flameobject in scene.flameobjects.iter()
                                        {
                                            blue_flame_common::object_actions::delete_shape(&flameobject.settings.label, objects);
                                        }
                                        match flameobject_blueprint
                                        {
                                            Some(ref flameobject_settings) =>
                                            {
                                                blue_flame_common::object_actions::create_shape(flameobject_settings, &current_project_dir, renderer, objects, window)
                                            },
                                            None => {},
                                        };
                                        //blue_flame_common::object_actions::create_shape(flameobject, project_dir, renderer, objects, window)

                                        previous_viewmode = editor_modes.main.1.clone();
                                    }
                                }
                            }
                        }
                        
                    });
            
                    ui.separator();

                    // Create new _ and save buttons
                    ui.horizontal(|ui|
                    {
                        if let ViewModes::Objects = editor_modes.main.1
                        {
                            // Create new flameobject
                            if ui.button(format!("{} Create object", emojis.add)).clicked()
                            //|| ui.input(|i| i.key_pressed(egui::Key::A) && i.modifiers.shift))
                            //|| input.key_held(VirtualKeyCode::LShift) && input.key_pressed(VirtualKeyCode::A)
                            && editor_modes.main.2 == false
                            {
                                editor_modes.main.2 = true;

                                let len = scene.flameobjects.len() as u16;

                                scene.flameobjects.push(Flameobject::init(len, None));
                                Flameobject::change_choice(&mut scene.flameobjects, len);
                                
                            }

                            // Determines to display "create new object" window
                            if editor_modes.main.2 == true
                            {
                                let mut cancel_creation_object = false; // If user presses cancel then pop from flameobjects
                                for (i, flameobject) in scene.flameobjects.iter_mut().enumerate()
                                {
                                    if flameobject.selected == true
                                    {
                                        match new_object_window(&mut flameobject.settings, &mut projects, &emojis, &window_size, ctx, ui, input, renderer, objects, window)
                                        {
                                            Some(action) =>
                                            {
                                                match action
                                                {
                                                    // ⛔ Cancel
                                                    false => cancel_creation_object = true,
                                                    // ➕ Create
                                                    true =>
                                                    {
                                                        flameobjects_selected_parent_idx = i as u16;
                                                        blue_flame_common::object_actions::create_shape(&flameobject.settings, &Project::selected_dir(&projects), renderer, objects, window);
                                                        editor_modes.main.2 = false;
                                                        break;
                                                    }
                                                }
                                            },
                                            None => {}
                                        }
                                    }
                                }
                                // If user presses cancel then pop from flameobjects
                                if cancel_creation_object == true
                                {
                                    scene.flameobjects.pop();
                                    editor_modes.main.2 = false;
                                }
                            }
                            if ui.button(format!("{} Save current scene", emojis.save)).clicked()
                            || input.key_held(VirtualKeyCode::LControl) && input.key_pressed(VirtualKeyCode::S)
                            //|| input.key_pressed(VirtualKeyCode::LControl || VirtualKeyCode::S)
                            {
                                if blue_flame_common::db::scene::save(&scene, &filepaths.current_scene, &current_project_dir) == true
                                {
                                    db::project_config::save(&mut project_config, &mut filepaths, &current_project_dir);
                                }
                            }


                        }
                        else if let ViewModes::Scenes = editor_modes.main.1
                        {
                            // Create new flameobject
                            if ui.button(format!("{} New scene", emojis.add)).clicked()
                            {
                                for flameobject in scene.flameobjects.iter_mut()
                                {
                                    blue_flame_common::object_actions::delete_shape(&flameobject.settings.label, objects);
                                }

                                scene = Scene::init(0);
                                filepaths.current_scene = String::new();
                            }
                            if ui.button(format!("{} Save current scene", emojis.save)).clicked()
                            || input.key_held(VirtualKeyCode::LControl) && input.key_pressed(VirtualKeyCode::S)
                            //|| input.key_pressed(VirtualKeyCode::LControl || VirtualKeyCode::S)
                            {
                                if blue_flame_common::db::scene::save(&scene, &filepaths.current_scene, &current_project_dir) == true
                                {
                                    db::project_config::save(&mut project_config, &mut filepaths, &current_project_dir);
                                }
                            }
                        }
                        else if let ViewModes::Blueprints = editor_modes.main.1
                        {
                            if ui.button(format!("{} Create object", emojis.add)).clicked()
                            {
                                flameobject_blueprint = Some(blue_flame_common::structures::flameobject::Settings::init(0, None));
                                editor_modes.main.2 = true;
                            }

                            if editor_modes.main.2 == true
                            {
                                let mut cancel_creation_object = false; // If user presses cancel then pop from flameobjects
                                match new_object_window(flameobject_blueprint.as_mut().unwrap(), &mut projects, &emojis, &window_size,
                                ctx, ui, input, renderer, objects, window)
                                {
                                    Some(action) =>
                                    {
                                        match action
                                        {
                                            // ⛔ Cancel
                                            false => editor_modes.main.2 = false,
                                            // ➕ Create
                                            true =>
                                            {
                                                blue_flame_common::object_actions::create_shape(flameobject_blueprint.as_ref().unwrap(), &Project::selected_dir(&projects), renderer, objects, window);
                                                editor_modes.main.2 = false;
                                            }
                                        }
                                    },
                                    None => {}
                                }
                            }
                            // Top left hand side when in blueprint view mode
                            if ui.button(format!("{} Save blueprint", emojis.save)).clicked()
                            || input.key_held(VirtualKeyCode::LControl) && input.key_pressed(VirtualKeyCode::S)
                            {
                                save_blueprint(&flameobject_blueprint, &blueprint_savefolderpath, &current_project_dir);
                                match flameobject_blueprint
                                {
                                    // WHen user preses save for blueprint object, any regular object inherited from blueprint and its changes will be affected 
                                    Some(ref flameobject_blueprint) =>
                                    {
                                        for flameobject in scene.flameobjects.iter_mut()
                                        {
                                            match flameobject.settings.blueprint_key
                                            {
                                                Some(ref blueprint_label) =>
                                                {
                                                    if blueprint_label.0 == flameobject_blueprint.label && blueprint_label.1 == true
                                                    {
                                                        flameobject.settings.texture = flameobject_blueprint.texture.clone();
                                                        flameobject.settings.color = flameobject_blueprint.color.clone();
                                                        flameobject.settings.rotation = flameobject_blueprint.rotation.clone();
                                                        flameobject.settings.size = flameobject_blueprint.size.clone();
                                                    }
                                                }
                                                None => continue,
                                            }
                                        }
                                    },
                                    None => {},
                                }
                                //db::blueprints::save(flameobject_blueprint.as_ref().unwrap(), &filepaths.current_scene, &current_project_dir);
                            }
                        }

                    });

                    // Temporary solution, will remove it when file explorer can be integrated
                    // Only created for testing purposes
                    if let ViewModes::Objects = editor_modes.main.1
                    {
                        if ui.button(format!("{} Blueprint in main scene", emojis.load)).clicked()
                        {
                            match flameobject_blueprint
                            {
                                Some(ref value) => 
                                {
                                    let len = scene.flameobjects.len() as u16;
                                    scene.flameobjects.push(Flameobject::init(len, None));
                                    scene.flameobjects[len as usize].settings = value.clone();
                                    scene.flameobjects[len as usize].settings.blueprint_key = Some((String::from(format!("{}", value.label)), true));
                                    Flameobject::change_choice(&mut scene.flameobjects, len);

                                    flameobjects_selected_parent_idx = len;
                                    blue_flame_common::object_actions::create_shape(&scene.flameobjects[len as usize].settings,
                                        &Project::selected_dir(&projects), renderer, objects, window);
                                }
                                None => println!("None in flameobject_blueprint"),
                            }
                        }
                    }

                    ui.separator();

                    // Displays all flameobjects/scenes button
                    if let ViewModes::Objects = editor_modes.main.1
                    {
                        // Only change chance labels if a single select was performed on labels
                        let mut change_choice = false;

                        for (i, flameobject) in scene.flameobjects.iter_mut().enumerate()
                        {
                            ui.horizontal(|ui|
                            {
                                ui.collapsing(format!("id: {}", flameobject.id), |ui|
                                {
                                    ui.label("some stuff");
                                });
                                if ui.selectable_label(flameobject.selected, &flameobject.settings.label).clicked()
                                {
                                    // if we are not attempting to select multiple items
                                    //if !ui.input(|i| i.modifiers.shift_only())
                                    if !input.key_held(VirtualKeyCode::LShift)
                                    {
                                        //Flameobject::change_choice(&mut scene.flameobjects, i as u16);
                                        label_backup = flameobject.settings.label.clone();
                                        flameobjects_selected_parent_idx = i as u16;
                                        change_choice = true;
                                    }
                                    // Multiple select via shift click keys
                                    else
                                    {
                                        flameobject.selected = !flameobject.selected;
                                    }

                                    //println!("label_backup: {}", label_backup);
                                }
                                ui.checkbox(&mut flameobject.visible, "");
                                if flameobject.visible == true
                                {
                                    ui.label(format!("{}", emojis.eye));
                                }
        
                                // Checks if variable names are correct or not
                                // Warnings
                                /*
                                if flameobjects[i].0.label.1.warning == true
                                {
                                    ui.label(issues::output_symbols().0);
                                }
                                // Errors
                                if flameobjects[i].0.label.1.error == true
                                {
                                    ui.label(issues::output_symbols().1);
                                }
                                */
        
                            });
                        }
                        if change_choice == true {Flameobject::change_choice(&mut scene.flameobjects, flameobjects_selected_parent_idx as u16)}
                    }
                    else if let ViewModes::Scenes = editor_modes.main.1
                    {
                        ui.label(format!("id: {}", &scene.id));
                        /*
                        for i in 0..scenes.len()
                        {
                            ui.horizontal(|ui|
                            {
                                ui.label(format!("id: {}", &loaded_scene.scene.id));
                                if ui.selectable_label(loaded_scene.scene.selected, &loaded_scene.scene.label).clicked()
                                {
                                    //Scene::change_choice(&mut loaded_scene.scene, i);
                                    // load scene

                                    blue_flame_common::db::scene::load(&mut loaded_scene.flameobjects, &Project::selected_dir(&projects), &loaded_scene.scene.filepath(), true, renderer, objects, window);
                                }
                            });
                        }
                        */
                    }

                    else if let ViewModes::Blueprints = editor_modes.main.1
                    {
                        ui.label("Save location of blueprint:");
                        ui.add(egui::TextEdit::singleline(&mut blueprint_savefolderpath));
                        if ui.button("Load blueprint").clicked()
                        {
                            db::blueprint::load(&mut flameobject_blueprint, &blueprint_savefolderpath, &current_project_dir, renderer, objects, window);
                        }
                    }

                }); // Left side

                // Right side
                egui::SidePanel::right("Object Settings").show(ctx, |ui|
                {

                    ui.set_enabled(!alert_window.0);

                    ui.set_width(ui.available_width());

                    if let ViewModes::Objects = editor_modes.main.1
                    {

                        if scene.flameobjects.len() > 0
                        {
                            let flameobject = &mut scene.flameobjects[flameobjects_selected_parent_idx as usize];

                            right_panel_flameobject_settings(&mut flameobject.settings, &mut enable_shortcuts, &mut label_backup, &current_project_dir, &projects,
                                &editor_settings,
                                ui, renderer, objects, window);

                        }

                        /*
                        if enable_shortcuts == true {shortcut_commands(&mut scene.flameobjects, &mut flameobjects_selected_parent_idx, &mut editor_modes, &mut mouse_functions,
                            &current_project_dir, &window_size,
                            input, ctx, ui,
                            renderer, objects, window)}
                        */
                        if enable_shortcuts == true
                        {
                            match right_click_menu(&mut mouse_functions, input, ctx)
                            {
                                Some(object_type_captured) => CreateNewFlameObject::flameobject(&object_type_captured, &mut scene.flameobjects,
                                    &mut flameobjects_selected_parent_idx, &current_project_dir, renderer, objects, window),
                                None => {},
                            }
                        }
                    }

                    else if let ViewModes::Scenes = editor_modes.main.1
                    {
                        ui.label("Scene name:");
                        ui.add(egui::TextEdit::singleline(&mut scene.label));
                        ui.separator();

                        ui.label("Save location:");
                        ui.add(egui::TextEdit::singleline(&mut filepaths.current_scene));
                        if ui.button("Invert filepath type").clicked()
                        {
                            filepaths.current_scene = invert_pathtype(&filepaths.current_scene, &projects);
                        }
                        if ui.button("Load scene").clicked()
                        {
                            if blue_flame_common::db::scene::load(&mut scene, &current_project_dir, &filepaths.current_scene, true,
                                renderer, objects, window) == true
                            {
                                project_config.last_scene_filepath = filepaths.current_scene.clone();
                                db::project_config::save(&mut project_config, &mut filepaths, &current_project_dir);
                            }
                        }
                        ui.separator();
                        
                        ui.label("High Power Mode:");
                        ui.horizontal(|ui|
                        {
                            ui.checkbox(&mut scene.settings.high_power_mode, "high_power_mode");
                        });
                    }
                    
                    else if let ViewModes::Blueprints = editor_modes.main.1
                    {
                        match flameobject_blueprint
                        {
                            Some(ref mut flameobject_settings) => 
                            {
                                right_panel_flameobject_settings(flameobject_settings, &mut enable_shortcuts, &mut label_backup, &current_project_dir, &projects,
                                    &editor_settings,
                                    ui, renderer, objects, window);
                            }
                            None => {}
                        }

                        /*
                        if enable_shortcuts == true {shortcut_commands(&mut scene.flameobjects, &mut flameobjects_selected_parent_idx, &mut editor_modes, &mut mouse_functions,
                            &current_project_dir, &window_size,
                            input, ctx, ui,
                            renderer, objects, window)}
                        */
                        if enable_shortcuts == true
                        {
                            match right_click_menu(&mut mouse_functions, input, ctx)
                            {
                                Some(object_type_captured) => CreateNewFlameObject::blueprint(&object_type_captured, &mut flameobject_blueprint, &current_project_dir, renderer, objects, window),
                                None => {},
                            }
                        }
 
                    }

                    for _ in 0..2
                    {
                        ui.separator();
                    }

                    // Blue print save related stuff
                    if let ViewModes::Objects = editor_modes.main.1
                    {
                        // single line edit for blue print save location
                        ui.add(egui::TextEdit::singleline(&mut blueprint_savefolderpath));

                        // blue print save button
                        if ui.button(format!("{} Save blueprint", emojis.save)).clicked()
                        {
                            save_blueprint(&flameobject_blueprint, &blueprint_savefolderpath, &current_project_dir);
                        }
                    }

                    // Delete button
                    ui.horizontal(|ui|
                    {
                        if let ViewModes::Objects = editor_modes.main.1
                        {
                            if ui.button(format!("{} Delete object", emojis.trash)).clicked()
                            //|| ui.input(|i| i.key_pressed(egui::Key::X))
                            || input.key_pressed(VirtualKeyCode::X) && enable_shortcuts == true
                            {
                                let mut remove_indexes: Vec<usize> = Vec::new();
                                
                                // Deletes object from game engine and stores the index of vector to remove
                                for (i, flameobject) in scene.flameobjects.iter().enumerate()
                                {
                                    if flameobject.selected == true
                                    {
                                        blue_flame_common::object_actions::delete_shape(&flameobject.settings.label, objects);
                                        remove_indexes.push(i);
                                    }
                                }
                                // Removes any element in flameobjects from vector based on the remove_indexes vector
                                for remove_index in remove_indexes.iter().rev()
                                {
                                    scene.flameobjects.remove(*remove_index);
                                }
                                Flameobject::recalculate_id(&mut scene.flameobjects);
                                //flameobjects_selected_parent_idx = (scene.flameobjects.len() - 1) as u16;
                                flameobjects_selected_parent_idx = blue_flame_common::range_limiter(scene.flameobjects.len() as i32 - 1i32,
                                u16::MIN as i32, u16::MAX as i32) as u16
                            }
                        }
                        else if let ViewModes::Scenes = editor_modes.main.1
                        {
                            if ui.button(format!("{} Delete scene", emojis.trash)).clicked()
                            {
                            }
                        }
                    });
                    

                }); // Right side
            }
            
        }, &window)
    }).unwrap();

}


// Generic way to create either flameobject or blueprint
struct CreateNewFlameObject;
impl CreateNewFlameObject
{
    fn flameobject(object_type_captured: &ObjectType, flameobjects: &mut Vec<Flameobject>, flameobjects_selected_parent_idx: &mut u16, project_dir: &str,
        renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
    {
        let len = flameobjects.len() as u16;
        flameobjects.push(Flameobject::init(len, Some(*object_type_captured)));
        Flameobject::change_choice(flameobjects, len);
        for (i, flameobject) in flameobjects.iter().enumerate()
        {
            if flameobject.selected == true
            {
                *flameobjects_selected_parent_idx = i as u16;
                blue_flame_common::object_actions::create_shape(&flameobject.settings, project_dir, renderer, objects, window);
            }
        }
    }
    fn blueprint(object_type_captured: &ObjectType, flameobject_blueprint: &mut Option<flameobject::Settings>, project_dir: &str,
        renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
    {
        //flameobject_blueprint = Some(Flameobject::init(len, Some(*object_type_captured)));
        *flameobject_blueprint = Some(flameobject::Settings::init(0, Some(*object_type_captured)));
        blue_flame_common::object_actions::create_shape(flameobject_blueprint.as_ref().unwrap(), project_dir, renderer, objects, window);
    }
}

fn right_click_menu(mouse_functions: &mut MouseFunctions, input: &blue_engine::InputHelper, ctx: &egui::Context) -> Option<ObjectType>
{
    let mut create_object: Option<ObjectType> = None;
    // shift + A: Right click menu
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

// Commands such as grab, size object, rotation etc
fn shortcut_commands(flameobjects: &mut Vec<Flameobject>, flameobjects_selected_parent_idx: &mut u16, editor_modes: &mut EditorModes, mouse_functions: &mut MouseFunctions,
    project_dir: &str, window_size: &WindowSize,
    /*Game engine shit*/ input: &blue_engine::InputHelper, ctx: &egui::Context, ui: &mut Ui, renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
{
    use blue_flame_common::convert_graphic_2_math_coords;


    //println!("mouse_x: {:?}, mouse_y: ", input.mouse_diff());
    // selectable_label
    ui.interact(egui::Rect::EVERYTHING, egui::Id::new("Right click"), egui::Sense::hover()).context_menu(|ui|
    {
        ui.label("One");
        ui.label("Two");
    });

    //println!("x: {}, y: {}", input.mouse().unwrap_or_default().0, input.mouse().unwrap_or_default().1);

    // shift + A: Right click menu
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

                                                let len = flameobjects.len() as u16;
                                                mouse_functions.object_type_captured = Some(ObjectType::Shape(shape::Dimension::D2(shape)));
                                                flameobjects.push(Flameobject::init(len, Some(mouse_functions.object_type_captured.unwrap())));
                                                Flameobject::change_choice(flameobjects, len);
                                                for (i, flameobject) in flameobjects.iter().enumerate()
                                                {
                                                    if flameobject.selected == true
                                                    {
                                                        *flameobjects_selected_parent_idx = i as u16;
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
        for flameobject in flameobjects.iter_mut()
        {
            flameobject.selected = false;
        }
    }

    // Selects all objects when pressing A
    else if !input.key_held(VirtualKeyCode::LShift) && input.key_pressed(VirtualKeyCode::A)
    {
        for flameobject in flameobjects.iter_mut()
        {
            flameobject.selected = true;
        }
    }

    // Do something with mouse objects i.e. grab, rotate, size based on key input
    //else if let mou
    else if input.key_pressed(VirtualKeyCode::G)
    {
        mouse_functions.object_mouse_movement = Some(ObjectMouseMovement::Grab);

        for flameobject in flameobjects.iter()
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
                    for flameobject in flameobjects.iter_mut()
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
}
// Displays shit like rotation, size, position, label etc
fn right_panel_flameobject_settings(flameobject_settings: &mut flameobject::Settings, enable_shortcuts: &mut bool, label_backup: &mut String, current_project_dir: &str,
    projects: &Vec<Project>, editor_settings: &EditorSettings,
    /*Game engine shit*/ ui: &mut Ui, renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
{

    // Object name
    if ui.add(egui::TextEdit::singleline(&mut flameobject_settings.label)).changed()
    {
        *enable_shortcuts = false;
        // Destroys hashmap
        blue_flame_common::object_actions::delete_shape(&label_backup, objects);
        
        // Creates new shape
        //object_management(flameobject, &mut projects, renderer, objects, window, ui);
        blue_flame_common::object_actions::create_shape(flameobject_settings, current_project_dir, renderer, objects, window);

        *label_backup = flameobject_settings.label.clone();
        //println!("label_backup {}", label_backup);
    }

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
    if ui.add(egui::TextEdit::singleline(&mut flameobject_settings.texture.file_location)).changed()
    {
        *enable_shortcuts = false;
        blue_flame_common::object_actions::update_shape::texture(flameobject_settings, &Project::selected_dir(&projects), objects, renderer);
    }
    if ui.button("Invert filepath type").clicked()
    {
        flameobject_settings.texture.file_location = invert_pathtype(&flameobject_settings.texture.file_location, &projects);
    }


    // Radio buttons for texturemodes
    {
        use blue_flame_common::radio_options::Texture;
        //let elements = Texture::elements();

        for element in Texture::elements()
        {
            if ui.radio_value(&mut flameobject_settings.texture.mode, element, Texture::label(&element)).changed()
            {
                blue_flame_common::object_actions::update_shape::texture(flameobject_settings, &Project::selected_dir(&projects), objects, renderer);
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
        if ui.color_edit_button_rgba_unmultiplied(&mut flameobject_settings.color).changed()
        {
            blue_flame_common::object_actions::update_shape::color(flameobject_settings, objects);
        }
    });
    ui.separator();

    ui.label("Position");
    ui.horizontal(|ui|
    {
        // Has user moved the shape or not
        let mut update_position = false;
        //let elements = flameobject.settings.position.elements();
        for (value, label) in flameobject_settings.position.elements()
        {
            ui.label(format!("{}:", label as char));

            // Use Response::changed or whatever to determine if the value has been changed
            if ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed)).changed()
            {
                update_position = true;
            }
        }
        // Updates the shape's position if the user has changed its value
        if update_position == true
        {
            blue_flame_common::object_actions::update_shape::position(flameobject_settings, objects);
        }

        
    });
    ui.separator();

    ui.label("Size");
    ui.horizontal(|ui|
    {
        // Has user moved the shape or not
        let mut update_size = false;
        
        for (value, label) in flameobject_settings.size.elements()
        {
            ui.label(format!("{}:", label as char));

            // Use Response::changed or whatever to determine if the value has been changed
            if ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed)).changed()
            {
                //println!("Changed!");
                update_size = true;
            }
            
        }
        // Updates the shape's size if the user has changed its value
        if update_size == true
        {
            //println!("update_position: {update_position}");
            blue_flame_common::object_actions::update_shape::size(flameobject_settings, objects, window);
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
    ctx: &egui::Context, ui: &mut Ui, input: &blue_engine::InputHelper,
    renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window, ) -> Option<bool>
{

    // If the user presses either Create, Cancel or does not do anything
    let mut action_button: Option<bool> = None;
    
    egui::Window::new("New Object")
    .fixed_pos(egui::pos2(window_size.x/2f32, window_size.y/2f32))
    .fixed_size(egui::vec2(100f32, 200f32))
    //.open(&mut editor_modes.main.2)
    .show(ctx, |ui|
    {
        ui.label("Select object type:");

        for (value, label) in ObjectType::elements(Some(flameobject_settings.object_type))
        {
            ui.selectable_value(&mut flameobject_settings.object_type, value, label);
        }
        // Shortcuts for moving up and down
        let move_direction: i8 = move_direction_keys(KeyMovement::Vertical, input);
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
            object_management(flameobject_settings, projects, renderer, objects, window, ui);
        });
    
        // Shortcuts for changing radio button options i.e. Square, Triangle, Line etc
        let move_direction: i8 = move_direction_keys(KeyMovement::Horizontal, input);
        
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
            || input.key_pressed(VirtualKeyCode::Escape)
            {
                action_button = Some(false);
            }
            if ui.button(format!("{} Create", emojis.add)).clicked()
            //|| ui.input(|i| i.key_pressed(egui::Key::Enter))
            || input.key_pressed(VirtualKeyCode::Return)
            {
                action_button = Some(true);
            }
        });
    });

    return action_button;

}

fn save_blueprint(flameobject_blueprint: &Option<flameobject::Settings>, folderpath: &str, current_project_dir: &str)
{
    db::blueprint::save(&flameobject_blueprint, folderpath, current_project_dir);
}