use blue_engine::{header::{Engine, /*ObjectSettings,*/ WindowDescriptor, PowerPreference}, primitive_shapes::{triangle, self, square}, Window, PhysicalSize};
use blue_engine_egui::{self, egui, EGUI};


use std::process::exit;


pub mod object_settings;
pub mod db;
mod practice;

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
            slider_speed        : 0.05f32,
        }
    }
}

pub struct Objects
{
    id          : u16,
    visible     : bool,
    selected    : bool,
    label       : (String, issues::Issues),
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
            label       : (format!("Object {id}"), issues::Issues::init())
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

pub struct ObjectSettings
{
    object_type         : Vec<object_settings::radio_options::Fields>,
    position            : [object_settings::three_d_lables::Fields; 3],
    size                : [object_settings::three_d_lables::Fields; 3],
    rotation            : [object_settings::three_d_lables::Fields; 3],
    texture             : object_settings::texture::Fields,
    color               : [f32; 4],
}
impl ObjectSettings
{
    fn init() -> Self
    {
        Self
        {
            object_type         : object_settings::radio_options::init(&["Square", "Triangle", "Line"]),
            position            : object_settings::three_d_lables::Fields::init(0f32),
            size                : object_settings::three_d_lables::Fields::init(30f32),
            rotation            : object_settings::three_d_lables::Fields::init(0f32),
            texture             : object_settings::texture::Fields::init(),
            color               : [1f32, 1f32, 1f32, 1f32],
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
// Stores all the projects you are working on
pub struct Projects
{
    name        : String,
    dir         : String,
    game_type   : Vec<object_settings::radio_options::Fields>,

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
            game_type   : object_settings::radio_options::init(&["2D", "3D"]),

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


pub enum EditorModes
{
    Projects((bool, &'static str)),
    Main,
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
}

struct Debug
{
    practice            : bool,
    resolution          : bool,
}

struct Db
{
    objects             : db::objects::Sql,
    scenes              : db::scenes::Sql,
    projects            : db::projects::Sql,
    //projects            : sql::projects::Sql,
}
impl Db
{
    fn init() -> Self
    {
        Self
        {
            objects             : db::objects::Sql::init(),
            scenes              : db::scenes::Sql::init(),
            projects            : db::projects::Sql::init(),
            //projects            : sql::projects::Sql::init(),
        }
    }
}


// Determines what "mode" we are in, for example projects we want to see or the main game editor?

fn main()
{

    // object's previous label just before it is modified
    let mut label_backup = String::new();

    let mut editor_modes =
    [
        (true, EditorModes::Projects((false, "Create objects"))), 
        (false, EditorModes::Main),
    ];


    let editor_settings = EditorSettings::init();

    let mut view_modes = object_settings::radio_options::init(&["Objects", "Scenes"]);

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

    let mut engine = Engine::new(
        WindowDescriptor
        {
            width               : if debug.resolution == true {1280} else {1920},
            height              : if debug.resolution == true {720} else {1080},
            title               : "Blue Flame",
            decorations         : true,
            resizable           : true,
            power_preference    : PowerPreference::LowPower,
        }).unwrap();



    // sql Variables
    let mut db = Db::init();

    // objects & scenes
    //let mut objects = vec![(Objects::init(0), ObjectSettings::init())];
    let mut objects: Vec<(Objects, ObjectSettings)> = Vec::new();
    let mut scenes: Vec<(Scenes, SceneSettings)> = Vec::new();
    let mut projects: Vec<Projects> = Vec::new();



    // Load all dbs into memory
    db.scenes.load(&mut scenes);
    db.objects.load(&mut objects);
    db.projects.load(&mut projects);


    // Start the egui context
    let gui_context = blue_engine_egui::EGUI::new(&engine.event_loop, &mut engine.renderer);

    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.plugins.push(Box::new(gui_context));

    //triangle("Object 1", blue_engine::header::ObjectSettings::default(), &mut engine.renderer, &mut engine.objects).unwrap();

    // init: draws and updates shapes
    for object in objects.iter()
    {
        for i in 0..object.1.object_type.len()
        {
            //let window = Window;
            //println!("object's name: {}\tobject's status: {}", object.1.object_type[i].name, object.1.object_type[i].status);
            if object_settings::object_actions::create_shape(object, i, &mut engine.renderer, &mut engine.objects, &mut engine.window) == true
            {
                break;
            }
        }
    }

    // Determines the current object's name and the puts the name in the backup_label
    for object in objects.iter()
    {
        if object.0.selected == true
        {
            label_backup = object.0.label.0.clone();
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
        issues::issue_checks::labels(&mut objects);

        // obtain the plugin
        let egui_plugin = plugins[0]
        // downcast it to obtain the plugin
        .downcast_mut::<blue_engine_egui::EGUI>()
        .expect("Plugin not found");


        // ui function will provide the context
        egui_plugin.ui(|ctx|
        {
            for editor_mode in editor_modes.iter_mut()
            {
                
                if let (true, EditorModes::Projects((val, _))) = editor_mode
                {
                    let mut _create_new_project: bool = *val;
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
                                db.projects.save(&projects);
                            }
                            if ui.button("➕ Create new project").clicked()
                            {
                                projects.push(Projects::init());
                                
                                let len = projects.len() - 1;
                                Projects::change_choice(&mut projects, len as u8);
                                
                                _create_new_project = true;
                            }
                        });

                        // Show all projects
                        for i in 0..projects.len()
                        {
                            if ui.selectable_label(projects[i].status, format!("{}: {}{}", projects[i].name, projects[i].dir, tab_spaces((window_size.x/4f32) as u16)))
                            .clicked()
                            {
                                Projects::change_choice(&mut projects, i as u8);
                            }
                        }



                        // Shows "New Project" scene
                        if _create_new_project == true
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
                                            if ui.radio(project.game_type[i].status, project.game_type[i].name).clicked()
                                            {
                                                object_settings::radio_options::change_choice(&mut project.game_type, i as u8);
                                            }
                                        }
                                    }
                                }
                                

                                // Shows extra buttons
                                ui.horizontal(|ui|
                                {
                                    if ui.button("➕ Create").clicked()
                                    {
                                        _create_new_project = false;
                                        let projects_len = (projects.len() - 1) as u8;
                                        Projects::change_choice(&mut projects, projects_len);

                                        db.projects.save(&projects);

                                        editor_mode.0 = false;
                                    }
                                    if ui.button("⛔ Cancel").clicked()
                                    {
                                        _create_new_project = false;
                                        projects.pop();
                                    }
                                });
                            });
                        }


                    });
                    *val = _create_new_project;
                }
                else if let (true, EditorModes::Main) = editor_mode
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
                                            db.objects.save(&objects);
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
                            fn current_scene(scenes: &[(Scenes, SceneSettings)]) -> String
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
                            for i in 0..view_modes.len()
                            {
                                if ui.selectable_label(view_modes[i].status, view_modes[i].name).clicked()
                                {
                                    object_settings::radio_options::change_choice(&mut view_modes, i as u8);
                                }
                            }
                            
                        });
              
                        ui.separator();

                        // Create new _ and save buttons
                        ui.horizontal(|ui|
                        {
                            for view_mode in view_modes.iter()
                            {
                                if view_mode.name == "Objects" && view_mode.status == true
                                {
                                    // Create new object
                                    if ui.button("➕ Create object").clicked()
                                    {
                                        let len = objects.len() as u16;
        
                                        objects.push((Objects::init(len), ObjectSettings::init()));
                                        Objects::change_choice(&mut objects, len);
        
                                        // Creates new object for the game engine
                                        for (i, object_type) in objects[len as usize].1.object_type.iter().enumerate()
                                        {
                                            if object_type.status == true
                                            {
                                                object_settings::object_actions::create_shape(&objects[len as usize], i, renderer, gameengine_objects, window);
                                            }
                                        }
                                    }
                                    if ui.button("💾 Save current scene").clicked()
                                    {
                                        db.objects.save(&objects);
                                    }
                                }
                                else if view_mode.name == "Scenes" && view_mode.status == true
                                {
                                    // Create new object
                                    if ui.button("➕ Create scene").clicked()
                                    {
                                        let len = scenes.len() as u16;
        
                                        scenes.push((Scenes::init(len), SceneSettings::default()));
                                        Scenes::change_choice(&mut scenes, len);
                                    }
                                    if ui.button("💾 Save scene settings").clicked()
                                    {
                                        db.scenes.save(&scenes);
                                    }
                                }
                            }
        
                        });

                        for view_mode in view_modes.iter()
                        {
                            if view_mode.name == "Scenes" && view_mode.status == true
                            {
                                if ui.button("Load scene").clicked()
                                {
        
                                }
                            }
                        }
                        
                        ui.separator();

                        // Displays all objects/scenes button
                        for view_mode in view_modes.iter()
                        {
                            if view_mode.name == "Objects" && view_mode.status == true
                            {
                                for i in 0..objects.len()
                                {
                                    ui.horizontal(|ui|
                                    {
                                        ui.collapsing(format!("id: {}", &objects[i].0.id), |ui|
                                        {
                                            ui.label("some stuff");
                                        });
                                        if ui.selectable_label(objects[i].0.selected, &objects[i].0.label.0).clicked()
                                        {
                                            Objects::change_choice(&mut objects, i as u16);
                                            label_backup = objects[i].0.label.0.clone();
                                            //println!("label_backup: {}", label_backup);
                                        }
                                        ui.checkbox(&mut objects[i].0.visible, "");
                                        if objects[i].0.visible == true
                                        {
                                            ui.label("👁");
                                        }
                
                                        // Checks if variable names are correct or not
                                        // Warnings
                                        if objects[i].0.label.1.warning == true
                                        {
                                            ui.label(issues::output_symbols().0);
                                        }
                                        // Errors
                                        if objects[i].0.label.1.error == true
                                        {
                                            ui.label(issues::output_symbols().1);
                                        }
                
                                    });
                                }
                            }
                            else if view_mode.name == "Scenes" && view_mode.status == true
                            {
                                for i in 0..scenes.len()
                                {
                                    ui.horizontal(|ui|
                                    {
                                        ui.label(format!("id: {}", &scenes[i].0.id));
                                        if ui.selectable_label(scenes[i].0.selected, &scenes[i].0.label).clicked()
                                        {
                                            Scenes::change_choice(&mut scenes, i as u16);
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
                        for view_mode in view_modes.iter()
                        {
                            if view_mode.name == "Objects" && view_mode.status == true
                            {
                                // Object name
                                for object in objects.iter_mut()
                                {
                                    if object.0.selected == true
                                    {
                                        ui.label(format!("Object name: {} {}",
                                            if object.0.label.1.warning == true {issues::output_symbols().0} else {""},
                                            if object.0.label.1.error == true {issues::output_symbols().1} else {""},
                                        ));
                                        if ui.add(egui::TextEdit::singleline(&mut object.0.label.0)).changed()
                                        {
                                            // Destroys hashmap
                                            object_settings::object_actions::destroy_hashmap(&label_backup, gameengine_objects);
                                            
                                            // Determines the current shape
                                            for (i, current_shape) in object.1.object_type.iter().enumerate()
                                            {
                                                if current_shape.status == true
                                                {
                                                    object_settings::object_actions::create_shape(object, i, renderer, gameengine_objects, window);
                                                    break;
                                                }
                                            }
                                            label_backup = object.0.label.0.clone();
                                            println!("label_backup {}", label_backup);
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
                                                if ui.radio(object.1.object_type[i].status, object.1.object_type[i].name).clicked()
                                                {
                                                    object_settings::radio_options::change_choice(&mut object.1.object_type, i as u8);
        
                                                    // Creates new object and/or changes object if the user clicks on some random choice button
                                                    object_settings::object_actions::create_shape(object, i, renderer, gameengine_objects, window);
                                                }
                                            }
                                        });
                                        ui.separator();
                
                                        // Locatin of texture
                                        ui.label("TextureMode");
                                        ui.label("Location of Texture");
                                        ui.add(egui::TextEdit::singleline(&mut object.1.texture.data));
                
                
                                        // Radio buttons for texturemodes
                                        for i in 0..object.1.texture.mode.len()
                                        {
                                            if ui.radio(object.1.texture.mode[i].status, object.1.texture.mode[i].name).clicked()
                                            {
                                                object_settings::radio_options::change_choice(&mut object.1.texture.mode, i as u8);
                                            }
                                        }
                                        ui.separator();
        
                                        ui.label("Color");
                                        ui.horizontal(|ui|
                                        {
                                            if ui.color_edit_button_rgba_unmultiplied(&mut object.1.color).changed()
                                            {
                                                object_settings::object_actions::update_shape::color(&object, gameengine_objects);
                                            }
                                        });
                                        ui.separator();
                
                                        ui.label("Position");
                                        ui.horizontal(|ui|
                                        {
                                            // Has user moved the shape or not
                                            let mut update_position = false;
                                            
                                            for position in object.1.position.iter_mut()
                                            {
                                                ui.label(format!("{}:", position.axis as char));
        
                                                // Use Response::changed or whatever to determine if the value has been changed
                                                if ui.add(egui::DragValue::new(&mut position.value).speed(editor_settings.slider_speed)).changed()
                                                {
                                                    //println!("Changed!");
                                                    update_position = true;
                                                }
                                                
                                            }
                                            // Updates the shape's position if the user has changed its value
                                            if update_position == true
                                            {
                                                //println!("update_position: {update_position}");
                                                object_settings::object_actions::update_shape::position(&object, gameengine_objects);
                                                /*
                                                gameengine_objects
                                                    .get_mut(&object.0.label.0)
                                                    .unwrap()
                                                    .position(object.1.position[0].value, object.1.position[1].value, object.1.position[2].value);
                                                */
                                            }
        
                                            
                                        });
                                        ui.separator();
        
                                        ui.label("Size");
                                        ui.horizontal(|ui|
                                        {
                                            // Has user moved the shape or not
                                            let mut update_size = false;
                                            
                                            for size in object.1.size.iter_mut()
                                            {
                                                ui.label(format!("{}:", size.axis as char));
        
                                                // Use Response::changed or whatever to determine if the value has been changed
                                                if ui.add(egui::DragValue::new(&mut size.value).speed(editor_settings.slider_speed)).changed()
                                                {
                                                    //println!("Changed!");
                                                    update_size = true;
                                                }
                                                
                                            }
                                            // Updates the shape's size if the user has changed its value
                                            if update_size == true
                                            {
                                                //println!("update_position: {update_position}");
                                                object_settings::object_actions::update_shape::size(&object, gameengine_objects, window);
                                                /*
                                                gameengine_objects
                                                    .get_mut(&object.0.label.0)
                                                    .unwrap()
                                                    .resize(object.1.size[0].value, object.1.size[1].value, object.1.size[2].value, window.inner_size());
                                                */
                                                
                                            }
                                            
                                        });
                                        ui.separator();
        
                                        ui.label("Rotation");
                                        ui.horizontal(|ui|
                                        {
                                            
                                            for rotation in object.1.rotation.iter_mut()
                                            {
                                                ui.label(format!("{}:", rotation.axis as char));
        
                                                // Use Response::changed or whatever to determine if the value has been changed
                                                if ui.add(egui::DragValue::new(&mut rotation.value).speed(editor_settings.slider_speed)).changed()
                                                {
                                                    object_settings::object_actions::update_shape::rotation(&object.0.label.0, rotation, gameengine_objects)
                                                }
                                                
                                            }
        
                                            
                                        });
                                    }
                                }
                            }
                            else if view_mode.name == "Scenes" && view_mode.status == true
                            {
                                for scene in scenes.iter_mut()
                                {
                                    if scene.0.selected == true
                                    {
                                        ui.label("Scene name:");
                                        ui.add(egui::TextEdit::singleline(&mut scene.0.label));
                                        ui.separator();
        
                                        ui.label("Save location:");
                                        ui.horizontal(|ui|
                                        {
                                            ui.label("dir_save: ");
                                            ui.add(egui::TextEdit::singleline(&mut scene.0.dir_save));
                                        });
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
                            for view_mode in view_modes.iter()
                            {
                                if view_mode.name == "Objects" && view_mode.status == true
                                {
                                    if ui.button("🗑 Delete object").clicked()
                                    {
                                        for i in 0..objects.len()
                                        {
                                            if objects[i].0.selected == true
                                            {
                                                object_settings::object_actions::destroy_hashmap(&objects[i].0.label.0, gameengine_objects);
                                                objects.remove(i);
                                                Objects::recalculate_id(&mut objects);
                                                break;
                                            }
                                        }
                                    }
                                }
                                else if view_mode.name == "Scenes" && view_mode.status == true
                                {
                                    if ui.button("🗑 Delete scene").clicked()
                                    {
                                        for i in 0..scenes.len()
                                        {
                                            if scenes[i].0.selected == true
                                            {
                                                scenes.remove(i);
                                                Scenes::recalculate_id(&mut scenes);
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        });
                        

                    }); // Right side
                }
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