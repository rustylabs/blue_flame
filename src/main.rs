use blue_engine::{header::{Engine, Renderer, ObjectStorage, /*ObjectSettings,*/ WindowDescriptor, PowerPreference}, Window};
use blue_engine_egui::{self, egui};
use std::{process::Command, io::Write};

use std::process::exit;


pub mod object_settings;
pub mod db;
mod practice;

// Directory related libraries
use std::path::PathBuf;
use dirs;



// Defines where all the file paths are
pub struct FilePaths
{
    projects        : PathBuf, // ~/.config/blue_flame
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
        
        library.push(".local/share/blue_flame/common");
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
/*
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Objects
{
    id          : u16,
    visible     : bool,
    selected    : bool,
    //label       : (String, issues::Issues),
    label       : String // "Object 0", "Object 1" etc
}
impl Objects
{
    fn init(id: u16) -> Self
    {
        Self
        {
            id,
            visible     : true,
            selected    : true,
            //label       : (format!("Object {id}"), issues::Issues::init()),
            label       : format!("Object {id}"),
        }
    }
    fn change_choice(list: &mut [(Self, ObjectSettings)], choice_true: u16)
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            if i as u16 == choice_true
            {
                item.0.selected = true;
            }
            else
            {
                item.0.selected = false;
            }
        }
    }
    // When user deletes the objects, we need to re calculate ids
    fn recalculate_id(list: &mut  [(Self, ObjectSettings)])
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            item.0.id = i as u16;
        }
    }
    // Checks for warnings and errors for labels and assigns the Issues variables appropriately

}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ObjectSettings
{
    object_type         : [bool; 3],
    //position            : [object_settings::three_d_lables::Fields; 3],
    position            : [f32; 3],
    size                : [f32; 3],
    rotation            : [f32; 3],
    texture             : object_settings::texture::Fields,
    //texture             : [String; 3],
    color               : [f32; 4],
}
impl ObjectSettings
{
    fn init() -> Self
    {
        //let position = [0f32; 3];
        const EMPTY: String = String::new();

        Self
        {
            object_type         : [true /*Square*/, false /*Triangle*/, false /*Line*/],
            position            : [0f32; 3],
            size                : [30f32; 3],
            rotation            : [0f32; 3],
            //texture             : [EMPTY; 3],
            texture             : object_settings::texture::Fields::init(),
            color               : [1f32; 4],
        }
    }
}
*/
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

/*
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Scenes
{
    id                  : u16,
    label               : String,
    dir_save            : String,
    selected            : bool,
}
impl Scenes
{
    fn init(id: u16) -> Self
    {
        Self
        {
            id,
            label               : format!("Scene {id}"),
            dir_save            : format!(""),
            selected            : true,
        }
    }
    // Returns full filepath of saved db of Scenes for &str args
    pub fn file_path(&self) -> String
    {
        return format!("{}/{}", self.dir_save, self.label);
    }

    fn change_choice(list: &mut [(Self, SceneSettings)], choice_true: u16)
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            if i as u16 == choice_true
            {
                item.0.selected = true;
            }
            else
            {
                item.0.selected = false;
            }
        }
    }
    // When user deletes the scenes, we need to re calculate ids
    fn recalculate_id(list: &mut  [(Self, SceneSettings)])
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            item.0.id = i as u16;
        }
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SceneSettings
{
    background_color        : u32,
    high_power_mode         : bool,
}
impl SceneSettings
{
    fn default() -> Self
    {
        Self
        {
            background_color        : 0x4d4d4d,         // Similar to Godot's background color for 2D
            high_power_mode         : true,
        }
    }
}
*/
// Stores all the projects you are working on
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Projects
{
    name        : String,
    dir         : String,
    game_type   : [bool; 2],

    status      : bool,
}
impl Projects
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
        pub fn labels(objects: &mut [(crate::Objects, crate::ObjectSettings)])
        {
            if objects.len() == 1
            {
                objects[0].0.label.1.error = false;
                return;
            }
            for i in 0..objects.len()
            {
                for j in 0..objects.len()
                {
                    if i != j
                    {
                        if objects[i].0.label.1.error != true && objects[i].0.label.0 == objects[j].0.label.0
                        {
                            objects[i].0.label.1.error = true;
                            break;
                        }
                        else
                        {
                            objects[i].0.label.1.error = false;    
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


fn create_project_config(path: &PathBuf)
{
    match std::fs::create_dir(format!("{}", path.display()))
    {
        Ok(_)       => println!("Config dir for project created succesfully in {}", path.display()),
        Err(e)      => println!("Unable to create config dir for project due to: {e}"),
    }
}

// Determines what "mode" we are in, for example projects we want to see or the main game editor?

fn main()
{

    // object's previous label just before it is modified
    let mut label_backup = String::new();

    // Show load screen or main game screen?
    let mut editor_modes = EditorModes
    {
        projects:
            (true, false /*Create new project*/,
            (true /*Create new project with "cargo new"*/, String::new() /*Dir name <project_name>*/),
            (false /*Window for delete project*/, false /*Delete entire project dir*/),
            ),
        main: (false, [true /*objects mode*/, false /*scenes mode*/]),
    };

    let mut file_paths: FilePaths = FilePaths::init();


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

    // objects & scenes
    //let mut objects: Vec<(Objects, ObjectSettings)> = Vec::new();
    let mut objects: Vec<(common::Objects, common::ObjectSettings)> = Vec::new();
    let mut scenes: Vec<(common::Scenes, common::SceneSettings)> = Vec::new();
    let mut projects: Vec<Projects> = Vec::new();




    // Load all dbs into memory
    db::projects::load(&mut projects, &file_paths);
    //db::scenes::load(&mut scenes);

    


    // Start the egui context
    let gui_context = blue_engine_egui::EGUI::new(&engine.event_loop, &mut engine.renderer);

    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.plugins.push(Box::new(gui_context));

    // Determines the current object's name and the puts the name in the backup_label
    for object in objects.iter()
    {
        if object.0.selected == true
        {
            label_backup = object.0.label.clone();
            //println!("label_backup: {}", label_backup);
            break;
        }
    }

    println!("----------Start of update_loop----------");
    engine.update_loop(move
    |
        renderer,
        window,
        gameengine_objects,
        _,
        _,
        plugins
    |
    {
        let window_size = WindowSize::init(&window);

        // Label error checking
        //issues::issue_checks::labels(&mut objects);

        // obtain the plugin
        let egui_plugin = plugins[0]
        // downcast it to obtain the plugin
        .downcast_mut::<blue_engine_egui::EGUI>()
        .expect("Plugin not found");




        fn add_scene(scenes: &mut Vec<(common::Scenes, common::SceneSettings)>)
        {
            let len = scenes.len() as u16;

            scenes.push((common::Scenes::init(len), common::SceneSettings::default()));
            common::Scenes::change_choice(scenes, len);
        }

        // ui function will provide the context
        egui_plugin.ui(|ctx|
        {

            // if true load project scene
            if editor_modes.projects.0 == true
            {


                fn load_project_scene(objects: &mut Vec<(common::Objects, common::ObjectSettings)>, scenes: &mut Vec<(common::Scenes, common::SceneSettings)>,
                projects: &mut [Projects], file_paths: &mut FilePaths, editor_modes: &mut EditorModes,
                    
                    /*Engine shit*/ renderer: &mut Renderer, gameengine_objects: &mut ObjectStorage, window: &Window
                )
                {
                    let projects_len = (projects.len() - 1) as u8;
                    Projects::change_choice(projects, projects_len);

                    db::projects::save(projects, file_paths);


                    for project in projects.iter()
                    {
                        if project.status == true
                        {
                            file_paths.scenes.push(format!("{}", project.dir));
                            file_paths.scenes.push("blue_flame");
                            file_paths.create_project_config(); // Creates blue_flame dir in project dir

                            // Changing editor mode
                            editor_modes.projects.0 = false;
                            editor_modes.projects.1 = false;
                            editor_modes.main.0 = true;

                            db::scenes::load(scenes, file_paths);

                            for scene in scenes.iter()
                            {
                                if scene.0.selected == true
                                {
                                    common::db::objects::load(objects, &scene.0.file_path(), true, renderer, gameengine_objects, window);
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
                                        scene.0.dir_save = String::from(format!("{}", file_paths.scenes.display()));
                                    }
                                }
                            }
                            break;
                        }
                    }
                }

                // Shows all your projects and what you want to load upon startup
                egui::Window::new("Projects")
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
                            load_project_scene(&mut objects, &mut scenes, &mut projects, &mut file_paths, &mut editor_modes, renderer, gameengine_objects, &window);
                        }
                        if ui.button("➕ Create/import project").clicked()
                        {
                            projects.push(Projects::init());
                            
                            let len = projects.len() - 1;
                            Projects::change_choice(&mut projects, len as u8);
                            
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
                        common::mapper::game_type(game_type_pos),

                        tab_spaces((window_size.x/4f32) as u16)))
                        .clicked()
                        {
                            Projects::change_choice(&mut projects, i as u8);
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
                                        if ui.radio(project.game_type[i], common::mapper::game_type(i)).clicked()
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
                                    // Sets the scene and not object to be true
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
                                            loaded_content = loaded_content.replace("{library}", &file_paths.library.to_str().unwrap());
                                            let mut output_file = std::fs::File::create(format!("{dir_src}/../Cargo.toml")).unwrap();
                                            output_file.write_all(loaded_content.as_bytes()).unwrap();

                                            break;
                                        }
                                    }


                                    load_project_scene(&mut objects, &mut scenes, &mut projects, &mut file_paths, &mut editor_modes, renderer, gameengine_objects, &window);
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
                                            db::projects::save(&mut projects, &mut file_paths);
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
                                                common::db::objects::save(&objects, &scene.0.file_path());
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
                        fn current_scene(scenes: &[(common::Scenes, common::SceneSettings)]) -> String
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
                            if ui.selectable_label(editor_modes.main.1[i], common::mapper::view_mode(i)).clicked()
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
                            if common::mapper::view_mode(i) == "Objects" && *view_mode == true
                            {
                                // Create new object
                                if ui.button("➕ Create object").clicked()
                                {
                                    let len = objects.len() as u16;
    
                                    objects.push((common::Objects::init(len), common::ObjectSettings::init()));
                                    common::Objects::change_choice(&mut objects, len);
    
                                    // Creates new object for the game engine
                                    for object_type in objects[len as usize].1.object_type.iter()
                                    {
                                        if *object_type == true
                                        {
                                            common::object_actions::create_shape(&objects[len as usize], renderer, gameengine_objects, window);
                                        }
                                    }
                                }
                                if ui.button("💾 Save current scene").clicked()
                                {
                                    for scene in scenes.iter()
                                    {
                                        if scene.0.selected == true
                                        {
                                            common::db::objects::save(&objects, &scene.0.file_path());
                                            break;
                                        }
                                    }
                                    
                                }
                            }
                            else if common::mapper::view_mode(i) == "Scenes" && *view_mode == true
                            {
                                // Create new object
                                if ui.button("➕ Create scene").clicked()
                                {
                                    add_scene(&mut scenes);
                                    common::db::objects::load(&mut objects, &scenes[i].0.file_path(), true, renderer, gameengine_objects, window);
                                }
                                if ui.button("💾 Save scene settings").clicked()
                                {
                                    db::scenes::save(&scenes, &file_paths);
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

                    // Displays all objects/scenes button
                    for (i, view_mode) in editor_modes.main.1.iter().enumerate()
                    {
                        if common::mapper::view_mode(i) == "Objects" && *view_mode == true
                        {
                            for i in 0..objects.len()
                            {
                                ui.horizontal(|ui|
                                {
                                    ui.collapsing(format!("id: {}", &objects[i].0.id), |ui|
                                    {
                                        ui.label("some stuff");
                                    });
                                    if ui.selectable_label(objects[i].0.selected, &objects[i].0.label).clicked()
                                    {
                                        common::Objects::change_choice(&mut objects, i as u16);
                                        label_backup = objects[i].0.label.clone();
                                        //println!("label_backup: {}", label_backup);
                                    }
                                    ui.checkbox(&mut objects[i].0.visible, "");
                                    if objects[i].0.visible == true
                                    {
                                        ui.label("👁");
                                    }
            
                                    // Checks if variable names are correct or not
                                    // Warnings
                                    /*
                                    if objects[i].0.label.1.warning == true
                                    {
                                        ui.label(issues::output_symbols().0);
                                    }
                                    // Errors
                                    if objects[i].0.label.1.error == true
                                    {
                                        ui.label(issues::output_symbols().1);
                                    }
                                    */
            
                                });
                            }
                        }
                        else if common::mapper::view_mode(i) == "Scenes" && *view_mode == true
                        {
                            for i in 0..scenes.len()
                            {
                                ui.horizontal(|ui|
                                {
                                    ui.label(format!("id: {}", &scenes[i].0.id));
                                    if ui.selectable_label(scenes[i].0.selected, &scenes[i].0.label).clicked()
                                    {
                                        common::Scenes::change_choice(&mut scenes, i as u16);
                                        // load scene

                                        common::db::objects::load(&mut objects, &scenes[i].0.file_path(), true, renderer, gameengine_objects, window);
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
                            for object in objects.iter_mut()
                            {
                                if object.0.selected == true
                                {
                                    if ui.add(egui::TextEdit::singleline(&mut object.0.label)).changed()
                                    {
                                        // Destroys hashmap
                                        common::object_actions::delete_shape(&label_backup, gameengine_objects);
                                        
                                        // Determines the current shape
                                        for current_shape in object.1.object_type.iter()
                                        {
                                            if *current_shape == true
                                            {
                                                common::object_actions::create_shape(object, renderer, gameengine_objects, window);
                                                break;
                                            }
                                        }
                                        label_backup = object.0.label.clone();
                                        //println!("label_backup {}", label_backup);
                                    }
                                }
                            }
                            // Object type
                            for object in objects.iter_mut()
                            {
                                if object.0.selected == true
                                {
                                    ui.label("Object type");
                                    ui.horizontal(|ui|
                                    {
                                        for i in 0..object.1.object_type.len()
                                        {
                                            if ui.radio(object.1.object_type[i], common::mapper::object_type(i)).clicked()
                                            {
                                                radio_options::change_choice(&mut object.1.object_type, i as u8);
    
                                                // Creates new object and/or changes object if the user clicks on some random choice button
                                                common::object_actions::create_shape(object, renderer, gameengine_objects, window);
                                            }
                                        }
                                    });
                                    ui.separator();
            
                                    // Locatin of texture
                                    ui.label("TextureMode");
                                    ui.label("Location of Texture");
                                    if ui.add(egui::TextEdit::singleline(&mut object.1.texture.file_location)).changed()
                                    {
                                        common::object_actions::update_shape::texture(&object, gameengine_objects, renderer);
                                    }
            
            
                                    // Radio buttons for texturemodes
                                    for i in 0..object.1.texture.mode.len()
                                    {
                                        if ui.radio(object.1.texture.mode[i], common::mapper::texture::label(i)).clicked()
                                        {
                                            radio_options::change_choice(&mut object.1.texture.mode, i as u8);
                                            common::object_actions::update_shape::texture(&object, gameengine_objects, renderer);
                                        }
                                    }
                                    ui.separator();
    
                                    ui.label("Color");
                                    ui.horizontal(|ui|
                                    {
                                        if ui.color_edit_button_rgba_unmultiplied(&mut object.1.color).changed()
                                        {
                                            common::object_actions::update_shape::color(&object, gameengine_objects);
                                        }
                                    });
                                    ui.separator();
            
                                    ui.label("Position");
                                    ui.horizontal(|ui|
                                    {
                                        // Has user moved the shape or not
                                        let mut update_position = false;
                                        
                                        for (i, position) in object.1.position.iter_mut().enumerate()
                                        {
                                            ui.label(format!("{}:", common::mapper::three_d_lables::label(i) as char));
    
                                            // Use Response::changed or whatever to determine if the value has been changed
                                            if ui.add(egui::DragValue::new(position).speed(editor_settings.slider_speed)).changed()
                                            {
                                                update_position = true;
                                            }
                                            
                                        }
                                        // Updates the shape's position if the user has changed its value
                                        if update_position == true
                                        {
                                            common::object_actions::update_shape::position(&object, gameengine_objects);
                                        }
    
                                        
                                    });
                                    ui.separator();
    
                                    ui.label("Size");
                                    ui.horizontal(|ui|
                                    {
                                        // Has user moved the shape or not
                                        let mut update_size = false;
                                        
                                        for (i, size) in object.1.size.iter_mut().enumerate()
                                        {
                                            ui.label(format!("{}:", common::mapper::three_d_lables::label(i) as char));
    
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
                                            common::object_actions::update_shape::size(&object, gameengine_objects, window);
                                        }
                                        
                                    });
                                    ui.separator();
    
                                    ui.label("Rotation");
                                    ui.horizontal(|ui|
                                    {
                                        
                                        for (i, rotation) in object.1.rotation.iter_mut().enumerate()
                                        {
                                            ui.label(format!("{}:", common::mapper::three_d_lables::label(i) as char));
    
                                            // Use Response::changed or whatever to determine if the value has been changed
                                            if ui.add(egui::DragValue::new(rotation).speed(editor_settings.slider_speed)).changed()
                                            {
                                                common::object_actions::update_shape::rotation
                                                (
                                                    &object.0.label,
                                                    common::mapper::three_d_lables::enumm(i),
                                                    *rotation,
                                                    gameengine_objects,
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
                                if ui.button("🗑 Delete object").clicked()
                                {
                                    for i in 0..objects.len()
                                    {
                                        if objects[i].0.selected == true
                                        {
                                            common::object_actions::delete_shape(&objects[i].0.label, gameengine_objects);
                                            objects.remove(i);
                                            common::Objects::recalculate_id(&mut objects);
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
                                            common::Scenes::recalculate_id(&mut scenes);
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

/*
// Maps numbers with names i.e. 0 => Square etc!
mod mapper
{
    // position means position of array/Vector
    // What shape, i.e. circle, triangle etc
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
    pub mod texture
    {
        pub fn text(position: usize) -> &'static str
        {
            let textures: &[&'static str] = &["Clamp", "Repeat", "Mirror Repeat"];
            return textures[position];
        }
        pub fn enumm(position: usize) -> blue_engine::TextureMode
        {
            let textures = &[blue_engine::TextureMode::Clamp, blue_engine::TextureMode::Repeat, blue_engine::TextureMode::MirrorRepeat];
            return textures[position];
        }
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
*/