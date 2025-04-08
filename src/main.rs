use blue_engine::{Engine, PowerPreference, WindowDescriptor, KeyCode, Window};
use blue_engine_utilities::egui::egui::{self, Response, Ui};
use blue_flame_common::
{
    EditorSettings,
    radio_options::{ViewModes, object_type::ObjectType},
    structures::
    {
        emojis::EMOJIS, file_explorer::FilePaths, flameobject::{self, Flameobject},
        project_config::ProjectConfig,
        scene::Scene,
        BlueEngineArgs,
        GameEditorArgs,
        MouseFunctions,
        Project,
        WidgetFunctions,
        WindowSize
    }
};

use editor_mode_variables::EditorMode;
use std::process::exit;

mod object_settings;
mod db;
mod practice;

mod editor_modes;




// Generic way to create either flameobject or blueprint
struct CreateNewFlameObject;
impl CreateNewFlameObject
{
    // Child needs to be pushed before calling this function
    pub fn child(flameobject: &mut Flameobject, blueprint: Option<&flameobject::Settings>, project_dir: &str, blue_engine_args: &mut BlueEngineArgs, window: &Window)
    {
        // If its a blueprint then copy its contents over to the newly created object
        if let Some(blueprint) = blueprint
        {
            let tmp_label_key = flameobject.settings.label_key.clone();
            flameobject.settings = blueprint.clone();
            flameobject.settings.label_key = tmp_label_key; // Sets it back to the original object's label_key to avoid double label_key issues
            flameobject.settings.blueprint_key = Some((blueprint.label_key.clone(), true));
        }

        blue_flame_common::object_actions::create_shape(&flameobject.settings, blue_engine_args, window);
    }

    pub fn flameobject(object_type_captured: Option<&ObjectType>, scene: &mut Scene, widget_functions: &mut WidgetFunctions, project_dir: &str,
        blue_engine_args: &mut BlueEngineArgs, window: &Window, blueprint: Option<&flameobject::Settings>,
    )
    {
        let len = scene.flameobjects.len();
        //let id = Flameobject::get_available_id(&mut scene.flameobjects);
        let id = Flameobject::get_available_id(&scene.flameobjects, None);
        //println!("id: {}", id);

        match blueprint
        {
            // Is blueprint
            Some(value) =>
            {
                let tmp_label_key;

                scene.flameobjects.push(Flameobject::init(id, None, false));
                tmp_label_key = scene.flameobjects[len].settings.label_key.clone(); // Gets the object label_key
                scene.flameobjects[len].settings = value.clone();
                scene.flameobjects[len].settings.label_key = tmp_label_key; // Sets it back to the original object's label_key to avoid double label_key issues
                scene.flameobjects[len].settings.blueprint_key = Some((String::from(format!("{}", value.label_key)), true));
                //scene.flameobjects[len as usize].settings.label_key = 
            }
            // If not importing blueprint just do a regular object creation
            None => scene.flameobjects.push(Flameobject::init(id, Some(*object_type_captured.unwrap()), false)),
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
        
        Flameobject::change_choice(&mut scene.flameobjects, id);
        /*
        scene.flameobject_selected_parent_idx = scene.flameobjects.len() as u16 - 1;
        scene.undo_redo.save_action(
            undo_redo::Action::Create(
                (
                    scene.flameobjects[scene.flameobject_selected_parent_idx as usize].settings.object_type,
                    scene.flameobjects[scene.flameobject_selected_parent_idx as usize].id)),
                    editor_settings);
        */
        blue_flame_common::object_actions::create_shape(&scene.flameobjects[len].settings, blue_engine_args, window);
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
        *flameobject_blueprint = Some(flameobject::Settings::init(0, Some(*object_type_captured), true));
        blue_flame_common::object_actions::create_shape(flameobject_blueprint.as_ref().unwrap(), blue_engine_args, window);
    }
}





pub struct AlertWindow
{
    label       : String,
    state       : bool,
}
impl AlertWindow
{
    fn init() -> [Self; 6]
    {
        [
            Self{label: "Open".to_string(), state: false},
            Self{label: "New".to_string(), state: false},
            Self{label: format!("{} Save", EMOJIS.save), state: false},
            Self{label: "Export settings".to_string(), state: false},
            Self{label: "Import settings".to_string(), state: false},
            Self{label: format!("{} Settings", EMOJIS.settings), state: false},
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
    practice: bool,
    resolution: bool,
}


// Converts from fullpath to relativepath and vice versa
//fn invert_pathtype(filepath: &str, projects: &Vec<Project>) -> String
/*
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
*/


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
        blue_flame_common::object_actions::create_shape(&flameobject_settings, blue_engine_args, window);
    }
    
}

// Used for either loading already existing project or a brand new project
fn load_project_scene(is_loaded: bool, scene: &mut Scene, projects: &mut [Project], game_editor_args: &mut GameEditorArgs, blue_engine_args: &mut BlueEngineArgs, window: &Window)
{
    // If we have already loaded up the project just load the stuff from memory rather than db
    if is_loaded == true
    {
        for flameobject in scene.flameobjects.iter()
        {
            blue_flame_common::object_actions::create_shape(&flameobject.settings, blue_engine_args, window);
        }
        return;
    }

    // Test this out later

    let projects_idx = projects.len() - 1;
    for (i, project) in projects.iter_mut().enumerate()
    {
        if i == projects_idx
        {
            project.status = true;
            *game_editor_args.current_project_dir = project.dir.clone();
        }
        else
        {
            project.status = false;
        }
    }
    db::all_projects_manager::save(projects, game_editor_args.filepaths);


    // Loads the project

    for project in projects.iter()
    {
        if project.status == true
        {
            //filepaths.scene.push(format!("{}", project.dir));

            // Creates blue_flame dir in project_dir
            match std::fs::create_dir(format!("{}/blue_flame", project.dir))
            {
                Ok(_) => println!("Config dir for project created succesfully in {}/blue_flame", project.dir),
                Err(e) => println!("Unable to create config dir for project due to: {e}"),
            }

            //filepaths.scene.push("blue_flame");
            //filepaths.create_project_config(); // Creates blue_flame dir in project dir

            // Changing editor mode
            /*
            game_editor_args.editor_modes.projects.0 = false;
            game_editor_args.editor_modes.projects.1 = false;
            game_editor_args.editor_modes.main.0 = true;
            */
            game_editor_args.filepaths.project_config = format!("{}/blue_flame/project.conf", project.dir);
            db::project_config::load(game_editor_args.project_config, game_editor_args.filepaths);

            //db::scenes::load(scenes, filepaths);
            blue_flame_common::db::scene::load(scene,
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

            /*
            for (i, flameobject) in scene.flameobjects.iter().enumerate()
            {
                if flameobject.selected == true {scene.flameobject_selected_parent_idx = i as u16}
            }
            */
        }
    }

    //widget_functions.flameobject_old = Some(scene.flameobjects[scene.flameobject_selected_parent_idx as usize].settings.clone());
    /*
    if scene.flameobjects.len() > 0
    {
        game_editor_args.widget_functions.flameobject_old = Some(scene.flameobjects[scene.flameobject_selected_parent_idx as usize].settings.clone());
    }
    else
    {
        game_editor_args.widget_functions.flameobject_old = None;
    }
    */
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
            pub file_explorer: FileExplorer,
        }
        impl Main
        {
            pub fn init() -> Self
            {
                Self
                {
                    file_explorer: FileExplorer::init(),
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
    flameobject_settings: Option<flameobject::Settings>,
    save_file_path: String,
}

//const FLAMEOBJECT_BLUEPRINT_LABEL: &'static str = "FLAMEOBJECT_BLUEPRINT";
fn main()
{

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
        flameobject_settings: None,
        save_file_path: String::new(),
    };

    let mut viewmode = ViewModes::Objects;
    //let mut previous_viewmode = editor_modes.main.1.clone();

    // i.e. Are we in projects or main scene mode?
    let mut editor_mode = EditorMode::Project(editor_mode_variables::Project::init());


    let editor_settings = EditorSettings::init();

    //let mut view_modes = object_settings::radio_options::init(&["Objects", "Scenes"]);
    //let mut view_modes = [true, false];


    
    let mut alert_window = AlertWindow::init();

    let mut engine = Engine::new_config(
        WindowDescriptor
        {
            width               : if DEBUG.resolution == true {1280} else {1920},
            height              : if DEBUG.resolution == true {720} else {1080},
            title               : "Blue Flame",
            decorations         : true,
            resizable           : true,
            power_preference    : PowerPreference::LowPower,
            backends            : blue_engine::Backends::GL,
            ..Default::default()
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
    db::all_projects_manager::load(&mut projects, &filepaths);
    //db::scenes::load(&mut scenes);

    


    // Start the egui context
    let gui_context = blue_engine_utilities::egui::EGUI::new();

    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.signals.add_signal("egui", Box::new(gui_context));

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
        camera,
        signals
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
        let egui_plugin = signals
            .get_signal::<blue_engine_utilities::egui::EGUI>("egui")
            .expect("Plugin not found")
            .expect("Plugin type mismatch");


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
                camera: camera.get_mut("main").unwrap(),
            };

            enable_shortcuts = true;

            let mut game_editor_args = GameEditorArgs
            {
                filepaths: &mut filepaths,
                widget_functions: &mut widget_functions,
                project_config: &mut project_config,
                current_project_dir: &mut current_project_dir,
                //editor_modes: &mut editor_modes,
                window_size: &window_size,
                viewmode: &mut viewmode,
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
            //else if let EditorMode::Main(ref mut sub_editor_mode) = editor_mode
            else if let EditorMode::Main(_) = editor_mode
            {
                //struct AlertWindow{alert_window: }
                if editor_modes::main::main(
                    &mut scene,
                    &mut projects,
                    &mut blueprint,
                    &editor_settings,
                    &mut game_editor_args,
                    &mut alert_window,
                    &mut blue_engine_args,
                    window)
                == true
                {
                    editor_mode = EditorMode::Project(editor_mode_variables::Project::init());
                }
            }
        }, window)
    }).unwrap();

}


/*
// Single line edit for directories and contains file explorer button
fn directory_singleline(filepath_singleline: &mut String, starting_dir: Option<&str>, file_picker_mode: radio_options::FilePickerMode, make_relative: bool, ui: &mut Ui) -> (Response, bool)
{
    use radio_options::FilePickerMode;
    // bool return is to determine if file dir has been selected
    let mut selected_file = false;

    let mut response: Option<Response> = None;
    ui.horizontal(|ui|
    {
        response = Some(ui.add(egui::TextEdit::singleline(filepath_singleline)));
        if ui.button(format!("{}", EMOJIS.file_icons.folder)).clicked()
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

            if let Some(value) = path_picked
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
            }

        }
    });
    return (response.unwrap(), selected_file);
}
*/

// Right click menu for creating new objects
fn right_click_menu(mouse_functions: &mut MouseFunctions, input: &blue_engine::InputHelper, ctx: &egui::Context) -> Option<ObjectType>
{
    let mut create_object: Option<ObjectType> = None;

    // shift + A: Right click menu
    if input.key_held(KeyCode::ShiftLeft) && input.key_pressed(KeyCode::KeyA)
    {
        mouse_functions.is_right_clicked = true;
        //mouse_functions.captured_coordinates = input.mouse_diff().unwrap_or_default();
    }

    // Fucks off the right click menu
    if input.key_pressed(KeyCode::Escape)
    {
        mouse_functions.is_right_clicked = false;
    }
    if mouse_functions.is_right_clicked == true
    {
        //use blue_flame_common::radio_options::object_type::ObjectType;
        use blue_flame_common::radio_options::object_type::{light, shape};
        use blue_engine_utilities::egui::egui;

        //egui::Area::new("right click").fixed_pos(egui::pos2(axis.x, axis.y)).show(ctx, |ui|
        //egui::Area::new("right click".into()).fixed_pos(egui::pos2(mouse_functions.captured_coordinates.0, mouse_functions.captured_coordinates.1)).show(ctx, |ui|
        egui::Area::new("right click".into()).fixed_pos(egui::pos2(960f32/2f32, 720f32/2f32)).show(ctx, |ui|
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
                    //egui::Area::new("sub menu".into()).fixed_pos(egui::pos2(mouse_functions.captured_coordinates.0 + SUBMENU_DISTANCE_OFFSET, mouse_functions.captured_coordinates.1)).show(ctx, |ui|
                    egui::Area::new("sub menu".into()).fixed_pos(egui::pos2(960f32/2f32 + SUBMENU_DISTANCE_OFFSET, 720f32/2f32)).show(ctx, |ui|
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



fn tab_spaces(tab_spaces_times: u16) -> String
{
    let mut tab_spaces = String::new();

    for _ in 0..tab_spaces_times
    {
        tab_spaces.push(' ');
    }
    return tab_spaces;
}