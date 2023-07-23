use blue_engine::{header::{Engine, Renderer, ObjectStorage, /*ObjectSettings,*/ WindowDescriptor, PowerPreference}, Window};
use blue_engine_egui::{self, egui::{self, Ui}};
use blue_flame_common::{filepath_handling, structures::{flameobject::Flameobject, scene::Scene, project_config::ProjectConfig}};
use blue_flame_common::radio_options::ViewModes;
use std::{process::Command, io::Write};

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
    label       : &'static str,
    state       : bool,
}
impl AlertWindow
{
    fn init() -> [Self; 6]
    {
        [
            Self{label: "Open", state: false},
            Self{label: "New", state: false},
            Self{label: "💾 Save", state: false},
            Self{label: "Export settings", state: false},
            Self{label: "Import settings", state: false},
            Self{label: "⚙ Settings", state: false},
        ]
    }

    fn whats_enabled(alert_window: &[Self]) -> &'static str
    {
        for list in alert_window.iter()
        {
            if list.state == true
            {
                return list.label;
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



// Editor modes and options for creating new scene
struct EditorModes
{
    projects        :   (bool, bool /*"New Project" scene*/,
                        (bool /*2.0 Create new project with "cargo new" (checkbox)*/, String /*2.1 Label for <project_name>*/),
                        (bool /*3 Window for delete project*/, bool /*Delete entire project dir (checkbox)*/),
                        ),
    //main            : (bool, [bool;2]),
    main            : (bool, ViewModes),
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
fn object_management(flameobject: &mut Flameobject, projects: &mut [Project], renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window, ui: &mut Ui)
{
    use blue_flame_common::radio_options::object_type::{ObjectType, shape, light};

    let mut change_shape = false;

    match flameobject.settings.object_type
    {
        ObjectType::Empty => println!("todo!: Empty"),
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
                shape::Shape3D::Cube => println!("todo!: cube()"),
            }
        }
        ObjectType::Light(ref mut light) => match light
        {
            light::Light::Direction => println!("todo!: light()"),
        }
    }
    if change_shape == true
    {
        blue_flame_common::object_actions::create_shape(flameobject, &Project::selected_dir(&projects), renderer, objects, window);
    }
    
}

// Used for either loading already existing project or a brand new project
fn load_project_scene(is_new_project: bool, scene: &mut Scene, projects: &mut [Project],  filepaths: &mut FilePaths,
    project_config: &mut ProjectConfig, current_project_dir: &mut String, editor_modes: &mut EditorModes,
    /*Engine shit*/ renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window
)
{
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
            blue_flame_common::db::flameobjects::load(scene, &Project::selected_dir(&projects),
            &project_config.last_scene_filepath , true, renderer, objects, window);

            // If this is a new project we just created
            /*
            if is_new_project == true
            {
                loaded_scene.scene.dir_save = String::from(format!("{}",
                filepath_handling::fullpath_to_relativepath(&filepaths.scenes.display().to_string(), current_project_dir)));
            }
            */
        }
    }
}

fn main()
{

    let mut filepaths: FilePaths = FilePaths::init();

    // So that I don't have to keep finding out what is the current project dir
    let mut current_project_dir = String::new();

    // Creates lib dir
    //init_lib(&filepaths.library);

    // flameobject's previous label just before it is modified
    let mut label_backup = String::new();

    // Show load screen or main game screen?
    let mut editor_modes = EditorModes
    {
        projects:
            (true, false /*Create new project*/,
            (true /*Create new project with "cargo new"*/, String::new() /*Dir name <project_name>*/),
            (false /*Window for delete project*/, true /*Delete entire project dir*/),
            ),
        main: (false, ViewModes::Objects),
    };

    


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
    
    let mut alert_window = (false, AlertWindow::init());

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
            label_backup = flameobject.label.clone();
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
        _,
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
                        {
                            // Load existing project
                            load_project_scene(false, &mut scene, &mut projects, &mut filepaths, &mut project_config, &mut current_project_dir, &mut editor_modes,
                                renderer, objects, &window);
                        }
                        if ui.button("➕ Create/import project").clicked()
                        {
                            projects.push(Project::init());
                            
                            let len = (projects.len() - 1) as u16;
                            //Project::change_choice(&mut projects, len as u8);
                            projects.change_choice(len);
                            
                            editor_modes.projects.1 = true;
                        }
                        if ui.button("🗑 Delete project").clicked()
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
                                if ui.button("⛔ Cancel").clicked()
                                {
                                    editor_modes.projects.1 = false;
                                    projects.pop();
                                }

                                if ui.button("➕ Create").clicked()
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
                                    load_project_scene(true, &mut scene, &mut projects, &mut filepaths, &mut project_config, &mut current_project_dir, &mut editor_modes,
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
                                        if ui.button("⛔ Cancel").clicked()
                                        {
                                            editor_modes.projects.3.0 = false;
                                        }
                                        if ui.button("✅ Yes").clicked()
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
                                if ui.button(list.label).clicked()
                                {
                                    if list.label == "💾 Save"
                                    {
                                        blue_flame_common::db::flameobjects::save(&scene, &filepaths.current_scene, &current_project_dir);
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

                        let elements = ViewModes::elements();
                        for element in elements
                        {
                            ui.selectable_value(&mut editor_modes.main.1, element, ViewModes::label(&element));
                        }

                        /*
                        for i in 0..editor_modes.main.1.len()
                        {
                            if ui.selectable_label(editor_modes.main.1[i],
                                match blue_flame_common::mapper::ViewModes::value(i)
                                {
                                    blue_flame_common::mapper::ViewModes::Objects(label)  => label,
                                    blue_flame_common::mapper::ViewModes::Scenes(label)   => label,
                                }).clicked()
                            {
                                radio_options::change_choice(&mut editor_modes.main.1, i as u8);
                            }
                        }
                        */
                        
                    });
            
                    ui.separator();

                    // Create new _ and save buttons
                    ui.horizontal(|ui|
                    {
                        if let ViewModes::Objects = editor_modes.main.1
                        {
                            // Create new flameobject
                            if ui.button("➕ Create object").clicked()
                            {
                                let len = scene.flameobjects.len() as u16;

                                scene.flameobjects.push(Flameobject::init(len));
                                Flameobject::change_choice(&mut scene.flameobjects, len);

                                // Creates new flameobject for the game engine
                                blue_flame_common::object_actions::create_shape(&scene.flameobjects[len as usize], &Project::selected_dir(&projects), renderer, objects, window);
                                /*
                                for object_type in scene.flameobjects[len as usize].settings.object_type.iter()
                                {
                                    if *object_type == true
                                    {
                                        blue_flame_common::object_actions::create_shape(&scene.flameobjects[len as usize], &Project::selected_dir(&projects), renderer, objects, window);
                                    }
                                }
                                */
                            }
                        }
                        else if let ViewModes::Scenes = editor_modes.main.1
                        {
                            // Create new flameobject
                            if ui.button("➕ New scene").clicked()
                            {
                                for flameobject in scene.flameobjects.iter_mut()
                                {
                                    blue_flame_common::object_actions::delete_shape(&flameobject.label, objects);
                                }

                                scene = Scene::init(0);
                                filepaths.current_scene = String::new();
                            }
                        }
                        if ui.button("💾 Save current scene").clicked()
                        {
                            if blue_flame_common::db::flameobjects::save(&scene, &filepaths.current_scene, &current_project_dir) == true
                            {
                                db::project_config::save(&mut project_config, &mut filepaths, &current_project_dir);
                            }
                        }
    
                    });

                    ui.separator();

                    // Displays all flameobjects/scenes button
                    if let ViewModes::Objects = editor_modes.main.1
                    {
                        for i in 0..scene.flameobjects.len()
                        {
                            ui.horizontal(|ui|
                            {
                                ui.collapsing(format!("id: {}", &scene.flameobjects[i].id), |ui|
                                {
                                    ui.label("some stuff");
                                });
                                if ui.selectable_label(scene.flameobjects[i].selected, &scene.flameobjects[i].label).clicked()
                                {
                                    Flameobject::change_choice(&mut scene.flameobjects, i as u16);
                                    label_backup = scene.flameobjects[i].label.clone();
                                    //println!("label_backup: {}", label_backup);
                                }
                                ui.checkbox(&mut scene.flameobjects[i].visible, "");
                                if scene.flameobjects[i].visible == true
                                {
                                    ui.label("👁");
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

                                    blue_flame_common::db::flameobjects::load(&mut loaded_scene.flameobjects, &Project::selected_dir(&projects), &loaded_scene.scene.filepath(), true, renderer, objects, window);
                                }
                            });
                        }
                        */
                    }

                }); // Left panel

                // Right side
                egui::SidePanel::right("Object Settings").show(ctx, |ui|
                {
                    ui.set_enabled(!alert_window.0);

                    ui.set_width(ui.available_width());

                    if let ViewModes::Objects = editor_modes.main.1
                    {
                        // Object name
                        for flameobject in scene.flameobjects.iter_mut()
                        {
                            if flameobject.selected == true
                            {
                                if ui.add(egui::TextEdit::singleline(&mut flameobject.label)).changed()
                                {
                                    // Destroys hashmap
                                    blue_flame_common::object_actions::delete_shape(&label_backup, objects);
                                    
                                    // Determines the current shape
                                    
                                    object_management(flameobject, &mut projects, renderer, objects, window, ui);

                                    label_backup = flameobject.label.clone();
                                    //println!("label_backup {}", label_backup);
                                }
                            }
                        }
                        // Object type (radio buttons e.g. Square, Triangle, Line)
                        for flameobject in scene.flameobjects.iter_mut()
                        {
                            if flameobject.selected == true
                            {
                                ui.label("Object type");
                                ui.horizontal(|ui|
                                {
                                    object_management(flameobject, &mut projects, renderer, objects, window, ui);



                                    /*
                                    for i in 0..flameobject.settings.object_type.len()
                                    {
                                        if ui.radio(flameobject.settings.object_type[i],
                                            match blue_flame_common::mapper::ObjectType::value(i)
                                            {
                                                blue_flame_common::mapper::ObjectType::Square(label)        => label,
                                                blue_flame_common::mapper::ObjectType::Triangle(label)      => label,
                                                blue_flame_common::mapper::ObjectType::Line(label)          => label,
                                            }).clicked()
                                        {
                                            radio_options::change_choice(&mut flameobject.settings.object_type, i as u8);

                                            // Creates new flameobject and/or changes flameobject if the user clicks on some random choice button
                                            blue_flame_common::object_actions::create_shape(flameobject, &Project::selected_dir(&projects), renderer, objects, window);
                                        }
                                    }
                                    */
                                });
                                ui.separator();
        
                                // Locatin of texture
                                ui.label("TextureMode");
                                ui.label("Location of Texture");
                                if ui.add(egui::TextEdit::singleline(&mut flameobject.settings.texture.file_location)).changed()
                                {
                                    blue_flame_common::object_actions::update_shape::texture(&flameobject, &Project::selected_dir(&projects), objects, renderer);
                                }
                                if ui.button("Invert filepath type").clicked()
                                {
                                    flameobject.settings.texture.file_location = invert_pathtype(&flameobject.settings.texture.file_location, &projects);
                                }
        
        
                                // Radio buttons for texturemodes
                                {
                                    use blue_flame_common::radio_options::Texture;
                                    //let elements = Texture::elements();

                                    for element in Texture::elements()
                                    {
                                        if ui.radio_value(&mut flameobject.settings.texture.mode, element, Texture::label(&element)).changed()
                                        {
                                            blue_flame_common::object_actions::update_shape::texture(&flameobject, &Project::selected_dir(&projects), objects, renderer);
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
                                    if ui.color_edit_button_rgba_unmultiplied(&mut flameobject.settings.color).changed()
                                    {
                                        blue_flame_common::object_actions::update_shape::color(&flameobject, objects);
                                    }
                                });
                                ui.separator();
        
                                ui.label("Position");
                                ui.horizontal(|ui|
                                {
                                    // Has user moved the shape or not
                                    let mut update_position = false;
                                    //let elements = flameobject.settings.position.elements();
                                    for (value, label) in flameobject.settings.position.elements()
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
                                        blue_flame_common::object_actions::update_shape::position(&flameobject, objects);
                                    }

                                    
                                });
                                ui.separator();

                                ui.label("Size");
                                ui.horizontal(|ui|
                                {
                                    // Has user moved the shape or not
                                    let mut update_size = false;
                                    
                                    for (value, label) in flameobject.settings.size.elements()
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
                                        blue_flame_common::object_actions::update_shape::size(&flameobject, objects, window);
                                    }
                                    
                                });
                                ui.separator();

                                ui.label("Rotation");
                                ui.horizontal(|ui|
                                {
                                    
                                    for (value, label) in flameobject.settings.rotation.elements()
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
                            if blue_flame_common::db::flameobjects::load(&mut scene, &current_project_dir, &filepaths.current_scene, true,
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
                    
                    for _ in 0..2
                    {
                        ui.separator();
                    }

                    // Delete button
                    ui.horizontal(|ui|
                    {
                        if let ViewModes::Objects = editor_modes.main.1
                        {
                            if ui.button("🗑 Delete object").clicked()
                            {
                                for i in 0..scene.flameobjects.len()
                                {
                                    if scene.flameobjects[i].selected == true
                                    {
                                        blue_flame_common::object_actions::delete_shape(&scene.flameobjects[i].label, objects);
                                        scene.flameobjects.remove(i);
                                        Flameobject::recalculate_id(&mut scene.flameobjects);
                                        break;
                                    }
                                }
                            }
                        }
                        else if let ViewModes::Scenes = editor_modes.main.1
                        {
                            if ui.button("🗑 Delete scene").clicked()
                            {
                                /*
                                loaded_scene.scene.remove(i);
                                Scene::recalculate_id(&mut scenes);
                                break;
                                for i in 0..scenes.len()
                                {
                                    if scenes[i].selected == true
                                    {

                                    }
                                }
                                */
                            }
                        }
                    });
                    

                }); // Right side
            }
            
        }, &window)
    }).unwrap();

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

mod radio_options
{
    pub fn change_choice(list: &mut[bool], choice_true: u8)
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            if i as u8 == choice_true
            {
                *item = true;
            }
            else
            {
                *item = false;
            }
        }
    }
}