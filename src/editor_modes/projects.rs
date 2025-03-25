use std::borrow::BorrowMut;
use std::{process::Command, path::PathBuf};
use std::io::Write; 

use blue_engine_utilities::egui::{egui, egui:: Context};
use blue_engine::
{
    header::KeyCode,
    Window,
};
use blue_flame_common::
{
    structures::emojis::EMOJIS,
    radio_options::FilePickerMode,
};
use crate::{Scene, WindowSize, Project, FilePaths, ViewModes, BlueEngineArgs, GameEditorArgs, editor_mode_variables};
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

/*
pub fn main(game_editor_args: &mut GameEditorArgs, scene: &mut Scene /*projects: &mut Vec<Project>, filepaths: &mut FilePaths, string_backups: &mut StringBackups*/, emojis: &Emojis,
    widget_functions: &mut WidgetFunctions, project_config: &mut ProjectConfig, current_project_dir: &mut String, editor_modes: &mut EditorModes,
    window_size: &WindowSize,
    blue_engine_args: &mut BlueEngineArgs, window: &Window /*renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window*/)
*/
pub fn main(scene: &mut Scene, projects: &mut Vec<Project>, sub_editor_mode: &mut editor_mode_variables::Project,
    game_editor_args: &mut GameEditorArgs, blue_engine_args: &mut BlueEngineArgs, window: &Window) -> bool // Return to change editor_mode
{
    let mut change_editor_mode = false;
    /*
    let projects = &mut game_editor_args.projects;
    let filepaths = &mut game_editor_args.filepaths;
    let string_backups = &mut game_editor_args.string_backups;
    let window_size = &game_editor_args.window_size;
    let project_config = &mut game_editor_args.project_config;
    let widget_functions = &mut game_editor_args.widget_functions;
    let emojis = &EMOJIS;
    */
    //let scene = &mut powerobject.scene;
    egui::Window::new("Project")
    .collapsible(false)
    .fixed_pos(egui::pos2(0f32, 0f32))
    .fixed_size(egui::vec2(game_editor_args.window_size.x, game_editor_args.window_size.y))
    //.open(&mut open_projects)
    .show(blue_engine_args.ctx, |ui|
    {

        use blue_flame_common::radio_options::GameTypeDimensions;

        ui.set_width(ui.available_width());
        ui.set_height(ui.available_height());

        // Load or Create
        ui.horizontal(|ui|
        {
            if ui.button("Load scene").clicked()
            || (blue_engine_args.input.key_pressed(KeyCode::Enter) || blue_engine_args.input.key_pressed(KeyCode::NumpadEnter))
            {
                // Load existing project
                crate::load_project_scene(false, scene, projects, game_editor_args, blue_engine_args, window);
                change_editor_mode = true;
            }
            if ui.button(format!("{} Create/import project", EMOJIS.addition.plus)).clicked()
            {

                projects.push(Project::init());
                let len = (projects.len() - 1) as u16;
                //Project::change_choice(&mut projects, len as u8);
                projects.change_choice(len);
                
                //game_editor_args.editor_modes.projects.1 = true;
                sub_editor_mode.new_project_window = true;
            }
            if ui.button(format!("{} Delete project", EMOJIS.trash)).clicked()
            {
                sub_editor_mode.del_proj_win = true;
                //game_editor_args.editor_modes.projects.3.0 = true;
                
            }
        });

        // Show all projects
        {
            let mut change_choice_proj: Option<u16> = None;
            for (i, project) in projects.iter().enumerate()
            {
                if ui.selectable_label(project.status, format!("{}: {} {}",
                project.name,
                project.dir,
                //GameTypeDimensions::elements(&projects[i].game_type),
                //blue_flame_common::mapper::game_type(game_type_pos),
    
                crate::tab_spaces((game_editor_args.window_size.x/4f32) as u16))).clicked()
                {
                    //Project::change_choice(&mut projects, i as u8);
                    change_choice_proj = Some(i as u16);
                }
            }
            if let Some(i) = change_choice_proj
            {
                projects.change_choice(i);
            }
        }


        /*
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

            crate::tab_spaces((game_editor_args.window_size.x/4f32) as u16))).clicked()
            {
                //Project::change_choice(&mut projects, i as u8);
                projects.change_choice(i as u16);
            }
        }
        */
        // Shows "New Project" window after user presses "create/import project" button
        //if game_editor_args.editor_modes.projects.1 == true
        
        if sub_editor_mode.new_project_window == true
        {
            egui::Window::new("New Project")
            .fixed_pos(egui::pos2(game_editor_args.window_size.x/2f32, game_editor_args.window_size.y/2f32))
            .pivot(egui::Align2::CENTER_CENTER)
            .default_size(egui::vec2(game_editor_args.window_size.x/2f32, game_editor_args.window_size.y/2f32))
            .resizable(true)
            //.open(&mut _create_new_project)
            .show(blue_engine_args.ctx, |ui|
            {

                let len = projects.len() - 1;


                ui.label("Project name:");
                ui.add(egui::TextEdit::singleline(&mut projects[len].name));
                ui.separator();
                ui.label("Project directory:");

                // Sets new project (after pressing create button) to true
                //projects[len].status = true;
                match sub_editor_mode.selected_project_before_new
                {
                    Some(ref value) =>
                    {
                        let mut path = PathBuf::from(value);
                        path.pop();
                        crate::directory_singleline(&mut projects[len].dir, Some(&path.display().to_string()), FilePickerMode::OpenFolder, false, ui);
                    }
                    None =>
                    {
                        crate::directory_singleline(&mut projects[len].dir, None, FilePickerMode::OpenFolder, false, ui);
                    }
                }
                /*
                ui.horizontal(|ui|
                {
                    ui.add(egui::TextEdit::singleline(&mut projects[len].dir));
                    if ui.button(format!("{}", EMOJIS.file)).clicked()
                    {
                        let home_dir = match dirs::home_dir()
                        {
                            Some(value) => value.display().to_string(),
                            None => "/".to_string(),
                        };
                        let folder_path = FileDialog::new().set_directory(home_dir).pick_folder();

                        match folder_path
                        {
                            Some(value) => projects[len].dir = value.display().to_string(),
                            None => {},
                        }
                    }
                });
                */
                

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
                //ui.checkbox(&mut game_editor_args.editor_modes.projects.2.0, "Create new project with command: \"cargo new <project_name> --bin\"");
                ui.checkbox(&mut sub_editor_mode.create_new_project_with_cargo_new, "Create new project with command: \"cargo new <project_name> --bin\"");
                // Shows label to type out the name of <project_name>
                if sub_editor_mode.create_new_project_with_cargo_new == true
                {
                    //ui.add(egui::TextEdit::singleline(&mut game_editor_args.editor_modes.projects.2.1));
                    ui.add(egui::TextEdit::singleline(&mut sub_editor_mode.new_project_label));
                }

                // Shows Cancel and Create buttons
                ui.horizontal(|ui|
                {
                    if ui.button(format!("{} Cancel", EMOJIS.cancel)).clicked()
                    {
                        //editor_mode.projects.1 = false;
                        sub_editor_mode.new_project_window = false;
                        projects.pop();
                    }
                    // Create the project
                    if ui.button(format!("{} Create", EMOJIS.addition.plus)).clicked()
                    {
                        // Sets the scene and not flameobject to be true
                        *game_editor_args.viewmode = ViewModes::Scenes;

                        // Determines if to run "cargo new"
                        //if game_editor_args.editor_modes.projects.2.0 == true
                        if sub_editor_mode.create_new_project_with_cargo_new == true
                        {
                            // cargo new and copy stuff over
                            for project in projects.iter_mut()
                            {
                                if project.status == true
                                {
                                    
                                    // Runs "cargo new" and adds extra filepaths to appropriate variables
                                    //project.dir.push_str(&format!("/{}", game_editor_args.editor_modes.projects.2.1));
                                    project.dir.push_str(&format!("/{}", sub_editor_mode.new_project_label));

                                    Command::new("sh")
                                    .arg("-c")
                                    .arg(format!("cargo new \"{}\" --bin", project.dir))
                                    //.arg("cargo new \"../testing\" --bin")
                                    .output()
                                    .unwrap();

                                    //copy_files_over_new_project(project, game_editor_args.editor_modes, game_editor_args.filepaths);
                                    //copy_files_over_new_project(project, game_editor_args.editor_modes, game_editor_args.filepaths);
                                    copy_files_over_new_project(project, sub_editor_mode, game_editor_args.filepaths);

                                    break;
                                }
                            }
                            // Load new project
                            crate::load_project_scene(false, scene, projects, game_editor_args, blue_engine_args, window);
                            //game_editor_args.widget_functions.flameobject_old = Some(scene.flameobjects[scene.flameobject_selected_parent_idx as usize].settings.clone());
                            game_editor_args.widget_functions.flameobject_old = None;
                            change_editor_mode = true;
                        }
                        //*editor_mode = EditorMode::Main(crate::editor_mode_variables::main::Main::Main::init());
                    }
                });
            });
        }

        // Delete project
        //if game_editor_args.editor_modes.projects.3.0 == true
        if sub_editor_mode.del_proj_win == true
        {
            //delete_project(projects, game_editor_args.editor_modes, game_editor_args.filepaths, EMOJIS, game_editor_args.window_size, blue_engine_args.ctx);
            delete_project(projects, sub_editor_mode, game_editor_args.filepaths, game_editor_args.window_size, blue_engine_args.ctx);
        }
    });
    return change_editor_mode;
}
/*
fn delete_project(projects: &mut Vec<Project>, editor_modes: &mut EditorModes, filepaths: &FilePaths, emojis: &Emojis, window_size: &WindowSize,
    ctx: &Context)
*/
fn delete_project(projects: &mut Vec<Project>, sub_editor_mode: &mut editor_mode_variables::Project, filepaths: &FilePaths, window_size: &WindowSize,
    ctx: &Context)
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
                //ui.checkbox(&mut editor_modes.projects.3.1, "Delete entire project dir");
                ui.checkbox(&mut sub_editor_mode.del_entire_proj_checkbox, "Delete entire project dir");

                ui.horizontal(|ui|
                {
                    if ui.button(format!("{} Cancel", EMOJIS.cancel)).clicked()
                    {
                        //editor_modes.projects.3.0 = false;
                        sub_editor_mode.del_proj_win = false;
                    }
                    if ui.button(format!("{} Yes", EMOJIS.tick)).clicked()
                    {
                        //editor_modes.projects.3.0 = false;
                        sub_editor_mode.del_proj_win = false;

                        //if editor_modes.projects.3.1 == true
                        if sub_editor_mode.del_entire_proj_checkbox == true
                        {
                            match std::fs::remove_dir_all(remove_project_dir)
                            {
                                Ok(_) => {},
                                Err(e) => println!("Can't remove project: {e}"),
                            }
                        }
                        projects.remove(i);
                        crate::db::projects::save(projects, filepaths);
                    }
                });
            });
            break;
        }
    }
}

// This generates the files for the new project
fn copy_files_over_new_project(project: &Project, sub_editor_mode: &crate::editor_mode_variables::Project, filepaths: &FilePaths)
{
    use build_assert::build_assert;
    use const_str;
    use const_for::const_for;

    let dir_src = String::from(format!("{}/src", project.dir));

    struct CopyOver
    {
        main            : &'static str,
        blue_flame      : &'static [u8],
        cargo           : &'static str,
    }
    const COPY_OVER: CopyOver = CopyOver
    {
        main            : include_str!("../../copy_over/main"),
        blue_flame      : include_bytes!("../../copy_over/blue_flame"),
        cargo           : include_str!("../../copy_over/Cargo"),
    };

    struct ReplaceValue
    {
        project_name: &'static str,
        library: &'static str,
    }

    const REPLACE_VALUE: ReplaceValue = ReplaceValue{project_name: "<project_name>", library: "<library>"};

    // Perform safety checks to ensure REPLACE_VALUE in the copy over files exist otherwise compile fail
    const
    {
        struct CopyOverEnterStrip
        {
            main: &'static str,
            cargo: &'static str,
        }
        struct CopyOverTokens<'a>
        {
            main: &'a[&'static str],
            cargo: &'a[&'static str],
        }

        // Determine whether or not to pass compile check for each individual files
        struct CompilePass
        {
            main: bool,
            cargo: (bool/*project_name*/, bool /*library*/)
        }

        const COPY_OVER_ENTER_STRIP: CopyOverEnterStrip = CopyOverEnterStrip
        {
            main:
            {
                const TMP: &str = const_str::replace!(COPY_OVER.main, "\n", " ");
                const_str::replace!(TMP, ",", "")
            },
            cargo:
            {
                const TMP: &str = const_str::replace!(COPY_OVER.cargo, "\n", " ");
                const TMP1: &str = const_str::replace!(TMP, ",", "");
                const_str::replace!(TMP1, "}", "")
            },
        };
        const COPY_OVER_TOKENS: CopyOverTokens = CopyOverTokens
        {
            main: &const_str::split!(COPY_OVER_ENTER_STRIP.main, " "),
            cargo: &const_str::split!(COPY_OVER_ENTER_STRIP.cargo, " "),
        };

        let mut compile_pass: CompilePass = CompilePass{main: false, cargo: (false, false)};

        // main.rs
        const_for!(i in 0..COPY_OVER_TOKENS.main.len() =>
        {
            if const_str::equal!(COPY_OVER_TOKENS.main[i], REPLACE_VALUE.project_name)
            {
                compile_pass.main = true;
                break;
            }
        });
        build_assert!(compile_pass.main);


        // cargo.rs project_name
        const_for!(i in 0..COPY_OVER_TOKENS.cargo.len() =>
        {
            if const_str::equal!(COPY_OVER_TOKENS.cargo[i], REPLACE_VALUE.project_name)
            {
                compile_pass.cargo.0 = true;
                break;
            }
        });
        build_assert!(compile_pass.cargo.0);


        // cargo.rs library
        const_for!(i in 0..COPY_OVER_TOKENS.cargo.len() =>
        {
            if const_str::equal!(COPY_OVER_TOKENS.cargo[i], REPLACE_VALUE.library)
            {
                compile_pass.cargo.1 = true;
                break;
            }
        });
        build_assert!(compile_pass.cargo.1);
    }




    // main.rs
    let mut loaded_content = String::from(COPY_OVER.main);
    loaded_content = loaded_content.replace(REPLACE_VALUE.project_name, &project.name);
    //loaded_content = loaded_content.replace("{scene_path}", &project.dir);
    let mut output_file = std::fs::File::create(format!("{dir_src}/main.rs")).unwrap();
    output_file.write_all(loaded_content.as_bytes()).unwrap();

    // blue_flame.rs
    let loaded_content = COPY_OVER.blue_flame.to_vec();
    let mut output_file = std::fs::File::create(format!("{dir_src}/blue_flame.rs")).unwrap();
    output_file.write_all(&loaded_content).unwrap();

    // Cargo.toml
    let mut loaded_content = String::from(COPY_OVER.cargo);
    //loaded_content = loaded_content.replace("{project_name}", &editor_modes.projects.2.1);
    loaded_content = loaded_content.replace(REPLACE_VALUE.project_name, &sub_editor_mode.new_project_label);
    loaded_content = loaded_content.replace(REPLACE_VALUE.library, &filepaths.library.to_str().unwrap());
    let mut output_file = std::fs::File::create(format!("{dir_src}/../Cargo.toml")).unwrap();
    output_file.write_all(loaded_content.as_bytes()).unwrap();
}