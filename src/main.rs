use blue_engine::{header::{Engine, Renderer, ObjectStorage, /*ObjectSettings,*/ WindowDescriptor, PowerPreference}, Window};
use blue_engine_egui::{self, egui};
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
}
// Defines where all the file paths are
pub struct FilePaths
{
    projects        : PathBuf, // ~/.config/blue_flame/blue_flame_common
    scenes          : PathBuf,
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

        Self
        {
            projects,
            scenes: PathBuf::new(),
            library,
        }
    }
    // Creates the folder for the project
    fn create_project_config(&self)
    {
        match std::fs::create_dir(format!("{}", self.scenes.display()))
        {
            Ok(_)       => println!("Config dir for project created succesfully in {}", self.scenes.display()),
            Err(e)      => println!("Unable to create config dir for project due to: {e}"),
        }
    }
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
    game_type   : [bool; 2],

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
            game_type   : [true, false], // 2D or 3D

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
}

// Editor modes and options for creating new scene
struct EditorModes
{
    projects        :   (bool, bool /*"New Project" scene*/,
                        (bool /*2.0 Create new project with "cargo new"*/, String /*2.1 Label for <project_name>*/),
                        (bool /*3 Window for delete project*/, bool /*Delete entire project dir*/),
                        ),
    main            : (bool, [bool;2]),
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

fn main()
{

    let mut filepaths: FilePaths = FilePaths::init();

    // Creates lib dir
    init_lib(&filepaths.library);

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
        main: (false, [true /*flameobjects mode*/, false /*scenes mode*/]),
    };

    


    let editor_settings = EditorSettings::init();

    //let mut view_modes = object_settings::radio_options::init(&["Objects", "Scenes"]);
    //let mut view_modes = [true, false];

    let debug = Debug
    {
        practice        : true,
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
    //let mut flameobjects: Vec<(Objects, ObjectSettings)> = Vec::new();
    let mut flameobjects: Vec<(blue_flame_common::Flameobject, blue_flame_common::FlameobjectSettings)> = Vec::new();
    let mut scenes: Vec<(blue_flame_common::Scene, blue_flame_common::SceneSettings)> = Vec::new();
    let mut projects: Vec<Project> = Vec::new();




    // Load all dbs into memory
    db::projects::load(&mut projects, &filepaths);
    //db::scenes::load(&mut scenes);

    


    // Start the egui context
    let gui_context = blue_engine_egui::EGUI::new(&engine.event_loop, &mut engine.renderer);

    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.plugins.push(Box::new(gui_context));

    // Determines the current flameobject's name and the puts the name in the backup_label
    for flameobject in flameobjects.iter()
    {
        if flameobject.0.selected == true
        {
            label_backup = flameobject.0.label.clone();
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




        fn add_scene(scenes: &mut Vec<(blue_flame_common::Scene, blue_flame_common::SceneSettings)>)
        {
            let len = scenes.len() as u16;

            scenes.push((blue_flame_common::Scene::init(len), blue_flame_common::SceneSettings::default()));
            blue_flame_common::Scene::change_choice(scenes, len);
        }

        // ui function will provide the context
        egui_plugin.ui(|ctx|
        {

            // if true load project scene
            if editor_modes.projects.0 == true
            {


                fn load_project_scene(flameobjects: &mut Vec<(blue_flame_common::Flameobject, blue_flame_common::FlameobjectSettings)>, scenes: &mut Vec<(blue_flame_common::Scene, blue_flame_common::SceneSettings)>,
                projects: &mut [Project], filepaths: &mut FilePaths, editor_modes: &mut EditorModes,
                    
                    /*Engine shit*/ renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window
                )
                {
                    let projects_len = (projects.len() - 1) as u8;
                    Project::change_choice(projects, projects_len);

                    db::projects::save(projects, filepaths);


                    for project in projects.iter()
                    {
                        if project.status == true
                        {
                            filepaths.scenes.push(format!("{}", project.dir));
                            filepaths.scenes.push("blue_flame");
                            filepaths.create_project_config(); // Creates blue_flame dir in project dir

                            // Changing editor mode
                            editor_modes.projects.0 = false;
                            editor_modes.projects.1 = false;
                            editor_modes.main.0 = true;

                            db::scenes::load(scenes, filepaths);

                            for scene in scenes.iter()
                            {
                                if scene.0.selected == true
                                {
                                    blue_flame_common::db::flameobjects::load(flameobjects, &Project::selected_dir(&projects), &scene.0.filepath(), true, renderer, objects, window);
                                    break;
                                }
                            }

                            if scenes.len() == 0
                            {
                                add_scene(scenes);
                                for scene in scenes.iter_mut()
                                {
                                    if scene.0.selected == true
                                    {
                                        scene.0.dir_save = String::from(format!("{}", filepaths.scenes.display()));
                                    }
                                }
                            }
                            break;
                        }
                    }
                }

                // Shows all your projects and what you want to load upon startup
                egui::Window::new("Project")
                .collapsible(false)
                .fixed_pos(egui::pos2(0f32, 0f32))
                .fixed_size(egui::vec2(window_size.x, window_size.y))
                //.open(&mut open_projects)
                .show(ctx, |ui|
                {
                    ui.set_width(ui.available_width());
                    ui.set_height(ui.available_height());

                    // Load or Create
                    ui.horizontal(|ui|
                    {
                        if ui.button("Load scene").clicked()
                        {
                            load_project_scene(&mut flameobjects, &mut scenes, &mut projects, &mut filepaths, &mut editor_modes, renderer, objects, &window);
                        }
                        if ui.button("➕ Create/import project").clicked()
                        {
                            projects.push(Project::init());
                            
                            let len = projects.len() - 1;
                            Project::change_choice(&mut projects, len as u8);
                            
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
                        let mut game_type_pos: usize = 0;
                        for (j, game_type) in projects[i].game_type.iter().enumerate()
                        {
                            if *game_type == true
                            {
                                game_type_pos = j;
                            }
                        }

                        if ui.selectable_label(projects[i].status, format!("{}: {} {}{}",
                        projects[i].name,
                        projects[i].dir,
                        blue_flame_common::mapper::game_type(game_type_pos),

                        tab_spaces((window_size.x/4f32) as u16))).clicked()
                        {
                            Project::change_choice(&mut projects, i as u8);
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
                                if project.status == true
                                {
                                    for i in 0..project.game_type.len()
                                    {
                                        if ui.radio(project.game_type[i], blue_flame_common::mapper::game_type(i)).clicked()
                                        {
                                            radio_options::change_choice(&mut project.game_type, i as u8);
                                        }
                                    }
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
                                    editor_modes.main.1[0] = false;
                                    editor_modes.main.1[1] = true;

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
                                            loaded_content = loaded_content.replace("{scene_path}", &project.dir);
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


                                    load_project_scene(&mut flameobjects, &mut scenes, &mut projects, &mut filepaths, &mut editor_modes, renderer, objects, &window);
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
                                        for scene in scenes.iter()
                                        {
                                            if scene.0.selected == true
                                            {
                                                blue_flame_common::db::flameobjects::save(&flameobjects, &scene.0.filepath());
                                                break;
                                            }
                                        }

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
                        ui.label(format!("Current scene: {}", current_scene(&scenes)));
                        fn current_scene(scenes: &[(blue_flame_common::Scene, blue_flame_common::SceneSettings)]) -> String
                        {
                            for scene in scenes.iter()
                            {
                                if scene.0.selected == true
                                {
                                    return scene.0.label.clone();
                                }
                            }
                            return String::from("");
                        }
                    });

                    ui.separator();

                    // Tabs for other Objects or Scenes view
                    ui.horizontal(|ui|
                    {
                        ui.label("Current display:");
                        for i in 0..editor_modes.main.1.len()
                        {
                            if ui.selectable_label(editor_modes.main.1[i], blue_flame_common::mapper::view_mode(i)).clicked()
                            {
                                radio_options::change_choice(&mut editor_modes.main.1, i as u8);
                            }
                        }
                        
                    });
            
                    ui.separator();

                    // Create new _ and save buttons
                    ui.horizontal(|ui|
                    {
                        for (i, view_mode) in editor_modes.main.1.iter().enumerate()
                        {
                            if blue_flame_common::mapper::view_mode(i) == "Objects" && *view_mode == true
                            {
                                // Create new flameobject
                                if ui.button("➕ Create flameobject").clicked()
                                {
                                    let len = flameobjects.len() as u16;
    
                                    flameobjects.push((blue_flame_common::Flameobject::init(len), blue_flame_common::FlameobjectSettings::init()));
                                    blue_flame_common::Flameobject::change_choice(&mut flameobjects, len);
    
                                    // Creates new flameobject for the game engine
                                    for object_type in flameobjects[len as usize].1.object_type.iter()
                                    {
                                        if *object_type == true
                                        {
                                            blue_flame_common::object_actions::create_shape(&flameobjects[len as usize], &Project::selected_dir(&projects), renderer, objects, window);
                                        }
                                    }
                                }
                                if ui.button("💾 Save current scene").clicked()
                                {
                                    for scene in scenes.iter()
                                    {
                                        if scene.0.selected == true
                                        {
                                            blue_flame_common::db::flameobjects::save(&flameobjects, &scene.0.filepath());
                                            break;
                                        }
                                    }
                                    
                                }
                            }
                            else if blue_flame_common::mapper::view_mode(i) == "Scenes" && *view_mode == true
                            {
                                // Create new flameobject
                                if ui.button("➕ Create scene").clicked()
                                {
                                    add_scene(&mut scenes);
                                    blue_flame_common::db::flameobjects::load(&mut flameobjects, &Project::selected_dir(&projects), &scenes[i].0.filepath(),true, renderer, objects, window);
                                }
                                if ui.button("💾 Save scene settings").clicked()
                                {
                                    db::scenes::save(&scenes, &filepaths);
                                }
                            }
                        }
    
                    });

                    for (i, view_mode) in editor_modes.main.1.iter().enumerate()
                    {
                        /*
                        if *view_mode == true
                        {
                            if ui.button("Load scene").clicked()
                            {
    
                            }
                        }
                        */
                    }
                    
                    ui.separator();

                    // Displays all flameobjects/scenes button
                    for (i, view_mode) in editor_modes.main.1.iter().enumerate()
                    {
                        if blue_flame_common::mapper::view_mode(i) == "Objects" && *view_mode == true
                        {
                            for i in 0..flameobjects.len()
                            {
                                ui.horizontal(|ui|
                                {
                                    ui.collapsing(format!("id: {}", &flameobjects[i].0.id), |ui|
                                    {
                                        ui.label("some stuff");
                                    });
                                    if ui.selectable_label(flameobjects[i].0.selected, &flameobjects[i].0.label).clicked()
                                    {
                                        blue_flame_common::Flameobject::change_choice(&mut flameobjects, i as u16);
                                        label_backup = flameobjects[i].0.label.clone();
                                        //println!("label_backup: {}", label_backup);
                                    }
                                    ui.checkbox(&mut flameobjects[i].0.visible, "");
                                    if flameobjects[i].0.visible == true
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
                        else if blue_flame_common::mapper::view_mode(i) == "Scenes" && *view_mode == true
                        {
                            for i in 0..scenes.len()
                            {
                                ui.horizontal(|ui|
                                {
                                    ui.label(format!("id: {}", &scenes[i].0.id));
                                    if ui.selectable_label(scenes[i].0.selected, &scenes[i].0.label).clicked()
                                    {
                                        blue_flame_common::Scene::change_choice(&mut scenes, i as u16);
                                        // load scene

                                        blue_flame_common::db::flameobjects::load(&mut flameobjects, &Project::selected_dir(&projects), &scenes[i].0.filepath(), true, renderer, objects, window);
                                    }
                                });
                            }
                        }
                    }

                }); // Left panel

                // Right side
                egui::SidePanel::right("Object Settings").show(ctx, |ui|
                {
                    ui.set_enabled(!alert_window.0);

                    ui.set_width(ui.available_width());
                    for (i, view_mode) in editor_modes.main.1.iter().enumerate()
                    {
                        if i == 0 /*Objects*/ && *view_mode == true
                        {
                            // Object name
                            for flameobject in flameobjects.iter_mut()
                            {
                                if flameobject.0.selected == true
                                {
                                    if ui.add(egui::TextEdit::singleline(&mut flameobject.0.label)).changed()
                                    {
                                        // Destroys hashmap
                                        blue_flame_common::object_actions::delete_shape(&label_backup, objects);
                                        
                                        // Determines the current shape
                                        for current_shape in flameobject.1.object_type.iter()
                                        {
                                            if *current_shape == true
                                            {
                                                blue_flame_common::object_actions::create_shape(flameobject, &Project::selected_dir(&projects), renderer, objects, window);
                                                break;
                                            }
                                        }
                                        label_backup = flameobject.0.label.clone();
                                        //println!("label_backup {}", label_backup);
                                    }
                                }
                            }
                            // Object type
                            for flameobject in flameobjects.iter_mut()
                            {
                                if flameobject.0.selected == true
                                {
                                    ui.label("Object type");
                                    ui.horizontal(|ui|
                                    {
                                        for i in 0..flameobject.1.object_type.len()
                                        {
                                            if ui.radio(flameobject.1.object_type[i], blue_flame_common::mapper::object_type(i)).clicked()
                                            {
                                                radio_options::change_choice(&mut flameobject.1.object_type, i as u8);
    
                                                // Creates new flameobject and/or changes flameobject if the user clicks on some random choice button
                                                blue_flame_common::object_actions::create_shape(flameobject, &Project::selected_dir(&projects), renderer, objects, window);
                                            }
                                        }
                                    });
                                    ui.separator();
            
                                    // Locatin of texture
                                    ui.label("TextureMode");
                                    ui.label("Location of Texture");
                                    if ui.add(egui::TextEdit::singleline(&mut flameobject.1.texture.file_location)).changed()
                                    {
                                        blue_flame_common::object_actions::update_shape::texture(&flameobject, &Project::selected_dir(&projects), objects, renderer);
                                    }
                                    if ui.button("Invert filepath type").clicked()
                                    {
                                        flameobject.1.texture.file_location = invert_pathtype(&flameobject.1.texture.file_location, &projects);
                                    }
            
            
                                    // Radio buttons for texturemodes
                                    for i in 0..flameobject.1.texture.mode.len()
                                    {
                                        if ui.radio(flameobject.1.texture.mode[i], blue_flame_common::mapper::texture::label(i)).clicked()
                                        {
                                            radio_options::change_choice(&mut flameobject.1.texture.mode, i as u8);
                                            blue_flame_common::object_actions::update_shape::texture(&flameobject, &Project::selected_dir(&projects), objects, renderer);
                                        }
                                    }
                                    ui.separator();
    
                                    ui.label("Color");
                                    ui.horizontal(|ui|
                                    {
                                        if ui.color_edit_button_rgba_unmultiplied(&mut flameobject.1.color).changed()
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
                                        
                                        for (i, position) in flameobject.1.position.iter_mut().enumerate()
                                        {
                                            ui.label(format!("{}:", blue_flame_common::mapper::three_d_lables::label(i) as char));
    
                                            // Use Response::changed or whatever to determine if the value has been changed
                                            if ui.add(egui::DragValue::new(position).speed(editor_settings.slider_speed)).changed()
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
                                        
                                        for (i, size) in flameobject.1.size.iter_mut().enumerate()
                                        {
                                            ui.label(format!("{}:", blue_flame_common::mapper::three_d_lables::label(i) as char));
    
                                            // Use Response::changed or whatever to determine if the value has been changed
                                            if ui.add(egui::DragValue::new(size).speed(editor_settings.slider_speed)).changed()
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
                                        
                                        for (i, rotation) in flameobject.1.rotation.iter_mut().enumerate()
                                        {
                                            ui.label(format!("{}:", blue_flame_common::mapper::three_d_lables::label(i) as char));
    
                                            // Use Response::changed or whatever to determine if the value has been changed
                                            if ui.add(egui::DragValue::new(rotation).speed(editor_settings.slider_speed)).changed()
                                            {
                                                blue_flame_common::object_actions::update_shape::rotation
                                                (
                                                    &flameobject.0.label,
                                                    blue_flame_common::mapper::three_d_lables::enumm(i),
                                                    *rotation,
                                                    objects,
                                                )
                                            }
                                            
                                        }
    
                                        
                                    });
                                }
                            }
                        }
                        else if i == 1 /*Scenes*/ && *view_mode == true
                        {
                            for scene in scenes.iter_mut()
                            {
                                if scene.0.selected == true
                                {
                                    ui.label("Scene name:");
                                    ui.add(egui::TextEdit::singleline(&mut scene.0.label));
                                    ui.separator();
    
                                    ui.label("Save location:");
                                    ui.add(egui::TextEdit::singleline(&mut scene.0.dir_save));
                                    if ui.button("Invert filepath type").clicked()
                                    {
                                        scene.0.dir_save = invert_pathtype(&scene.0.dir_save, &projects);
                                    }
                                    ui.separator();
                                    
                                    ui.label("High Power Mode:");
                                    ui.horizontal(|ui|
                                    {
                                        ui.checkbox(&mut scene.1.high_power_mode, "high_power_mode").clicked();
                                    });
                                }
                                
                            }
                        }
                    }
                    
                    for _ in 0..2
                    {
                        ui.separator();
                    }

                    // Delete button
                    ui.horizontal(|ui|
                    {
                        for (i, view_mode)in editor_modes.main.1.iter().enumerate()
                        {
                            if i == 0 /*Objects*/ && *view_mode == true
                            {
                                if ui.button("🗑 Delete flameobject").clicked()
                                {
                                    for i in 0..flameobjects.len()
                                    {
                                        if flameobjects[i].0.selected == true
                                        {
                                            blue_flame_common::object_actions::delete_shape(&flameobjects[i].0.label, objects);
                                            flameobjects.remove(i);
                                            blue_flame_common::Flameobject::recalculate_id(&mut flameobjects);
                                            break;
                                        }
                                    }
                                }
                            }
                            else if i == 1 /*Scenes*/ && *view_mode == true
                            {
                                if ui.button("🗑 Delete scene").clicked()
                                {
                                    for i in 0..scenes.len()
                                    {
                                        if scenes[i].0.selected == true
                                        {
                                            scenes.remove(i);
                                            blue_flame_common::Scene::recalculate_id(&mut scenes);
                                            break;
                                        }
                                    }
                                }
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