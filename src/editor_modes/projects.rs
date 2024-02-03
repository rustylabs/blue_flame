use std::borrow::BorrowMut;
use std::{process::Command, path::PathBuf};
use std::io::Write; 

use blue_engine_egui::{self, egui::{self, Ui, InputState, Context}};
use blue_engine::header::VirtualKeyCode;
use blue_engine::{Renderer, ObjectSettings, ObjectStorage, Window};
use blue_flame_common::db::scene;
use blue_flame_common::emojis::Emojis;
use crate::{Scene, WindowSize, Project, FilePaths, StringBackups, WidgetFunctions, ProjectConfig, EditorModes, ViewModes, BlueEngineArgs};

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

pub fn main(/*powerobject: &mut view_modes_argument_passer::Projects*/scene: &mut Scene, projects: &mut Vec<Project>, filepaths: &mut FilePaths, string_backups: &mut StringBackups, emojis: &Emojis,
    widget_functions: &mut WidgetFunctions, project_config: &mut ProjectConfig, current_project_dir: &mut String, editor_modes: &mut EditorModes,
    window_size: &WindowSize,
    blue_engine_args: &mut BlueEngineArgs, window: &Window /*renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window*/)
{

    //let scene = &mut powerobject.scene;


    egui::Window::new("Project")
    .collapsible(false)
    .fixed_pos(egui::pos2(0f32, 0f32))
    .fixed_size(egui::vec2(window_size.x, window_size.y))
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
            || (blue_engine_args.input.key_pressed(VirtualKeyCode::Return) || blue_engine_args.input.key_pressed(VirtualKeyCode::NumpadEnter))
            {
                // Load existing project
                crate::load_project_scene(false, scene, projects, filepaths, string_backups, widget_functions,
                    project_config, current_project_dir, editor_modes,
                    blue_engine_args, window);
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

            crate::tab_spaces((window_size.x/4f32) as u16))).clicked()
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
            .show(blue_engine_args.ctx, |ui|
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
                            // cargo new and copy stuff over
                            for project in projects.iter_mut()
                            {
                                if project.status == true
                                {
                                    // Runs "cargo new" and adds extra filepaths to appropriate variables
                                    project.dir.push_str(&format!("/{}", editor_modes.projects.2.1));

                                    Command::new("sh")
                                    .arg("-c")
                                    .arg(format!("cargo new \"{}\" --bin", project.dir))
                                    //.arg("cargo new \"../testing\" --bin")
                                    .output()
                                    .unwrap();

                                    copy_files_over_new_project(project, editor_modes, filepaths);

                                    break;
                                }
                            }
                            // Load new project
                            crate::load_project_scene(false, scene, projects, filepaths, string_backups, widget_functions, project_config, current_project_dir, editor_modes,
                                blue_engine_args, window);
                            widget_functions.flameobject_old = Some(scene.flameobjects[scene.flameobject_selected_parent_idx as usize].settings.clone());
                        }
                    }
                });
            });
        }

        // Delete project
        if editor_modes.projects.3.0 == true
        {
            delete_project(projects, editor_modes, filepaths, emojis, window_size, blue_engine_args.ctx);
        }
    });
}

fn delete_project(projects: &mut Vec<Project>, editor_modes: &mut EditorModes, filepaths: &FilePaths, emojis: &Emojis, window_size: &WindowSize,
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
                        crate::db::projects::save(projects, filepaths);
                    }
                });
            });
            break;
        }
    }
}

fn copy_files_over_new_project(project: &Project, editor_modes: &EditorModes, filepaths: &FilePaths)
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
        main            : include_str!("../../copy_over/main.rs"),
        blue_flame      : include_bytes!("../../copy_over/blue_flame.rs"),
        cargo           : include_str!("../../copy_over/Cargo.toml"),
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
}

