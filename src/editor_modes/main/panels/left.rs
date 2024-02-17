use blue_engine_egui::{self, egui::{self}};
use blue_engine::header::VirtualKeyCode;
use blue_engine::Window;
use blue_flame_common::{emojis::Emojis, radio_options::FilePickerMode};
use blue_flame_common::structures::{flameobject::Flameobject, flameobject::Settings};
use crate::{Scene, WindowSize, Project, FilePaths, StringBackups, WidgetFunctions, ProjectConfig, ViewModes, BlueEngineArgs, GameEditorArgs, Blueprint,
    editor_mode_variables,
};
/*
pub fn main(scene: &mut Scene, blueprint.flameobject: &mut Option<Settings>, previous_viewmode: &mut ViewModes,
    projects: &mut Vec<Project>, filepaths: &mut FilePaths, string_backups: &mut StringBackups, emojis: &Emojis, blueprint_savefolderpath: &mut String,
   widget_functions: &mut WidgetFunctions, project_config: &mut ProjectConfig, current_project_dir: &mut String, editor_modes: &mut EditorModes,
   window_size: &WindowSize,
   blue_engine_args: &mut BlueEngineArgs, window: &Window)
*/
pub fn main(scene: &mut Scene, projects: &mut Vec<Project>, blueprint: &mut Blueprint, sub_editor_mode: &mut editor_mode_variables::Main, game_editor_args: &mut GameEditorArgs, blue_engine_args: &mut BlueEngineArgs, window: &Window)
{


    egui::SidePanel::left("Objects").show(blue_engine_args.ctx, |ui|
    {
        //ui.set_enabled(!alert_window.0);

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
                if ui.selectable_value(game_editor_args.viewmode, element, label).changed()
                {
                    // Switching between tabs
                    match game_editor_args.viewmode
                    {
                        ViewModes::Objects => 
                        {
                            if *game_editor_args.previous_viewmode == ViewModes::Blueprints
                            {
                                if let Option::Some(ref value) = blueprint.flameobject
                                {
                                    blue_flame_common::object_actions::delete_shape(&value.label, blue_engine_args);
                                }
                                crate::load_project_scene(true, scene, projects,  game_editor_args, blue_engine_args, window);
                                //widget_functions.flameobject_old = Some(scene.flameobjects[scene.flameobject_selected_parent_idx as usize].settings.clone());
                            }
                            *game_editor_args.previous_viewmode = game_editor_args.viewmode.clone();
                        }
                        ViewModes::Scenes => 
                        {
                            if *game_editor_args.previous_viewmode == ViewModes::Blueprints
                            {
                                if let Option::Some(ref value) = blueprint.flameobject
                                {
                                    blue_flame_common::object_actions::delete_shape(&value.label, blue_engine_args);
                                }
                                crate::load_project_scene(true, scene, projects, game_editor_args, blue_engine_args, window);
                                //widget_functions.flameobject_old = Some(scene.flameobjects[scene.flameobject_selected_parent_idx as usize].settings.clone());
                            }
                            *game_editor_args.previous_viewmode = game_editor_args.viewmode.clone();
                        }
                        ViewModes::Blueprints => 
                        {
                            // Remove all objects from scene then load or create a new object for blueprints variable
                            for flameobject in scene.flameobjects.iter()
                            {
                                blue_flame_common::object_actions::delete_shape(&flameobject.settings.label, blue_engine_args);
                            }
                            match blueprint.flameobject
                            {
                                Some(ref flameobject_settings) =>
                                {
                                    blue_flame_common::object_actions::create_shape(flameobject_settings, &game_editor_args.current_project_dir, blue_engine_args, window)
                                },
                                None => {},
                            };
                            //blue_flame_common::object_actions::create_shape(flameobject, project_dir, renderer, objects, window)

                            *game_editor_args.previous_viewmode = game_editor_args.viewmode.clone();
                        }
                    }
                }
            }
            
        });

        ui.separator();

        ui.horizontal(|ui|
        {
            if let ViewModes::Objects = game_editor_args.viewmode
            {
                // Create new flameobject
                if ui.button(format!("{} Create object", game_editor_args.emojis.add)).clicked()
                //|| ui.input(|i| i.key_pressed(egui::Key::A) && i.modifiers.shift))
                //|| input.key_held(VirtualKeyCode::LShift) && input.key_pressed(VirtualKeyCode::A)
                //&& sub_editor_mode.create_new_object_window == false
                && sub_editor_mode.create_new_object_window == false
                {
                    sub_editor_mode.create_new_object_window = true;

                    let len = scene.flameobjects.len() as u16;

                    scene.flameobjects.push(Flameobject::init(len, None));
                    Flameobject::change_choice(&mut scene.flameobjects, len);
                    
                }

                // Determines to display "create new object" window
                if sub_editor_mode.create_new_object_window == true
                {
                    let mut cancel_creation_object = false; // If user presses cancel then pop from flameobjects
                    for (i, flameobject) in scene.flameobjects.iter_mut().enumerate()
                    {
                        if flameobject.selected == true
                        {
                            match crate::new_object_window(&mut flameobject.settings, projects, &game_editor_args.emojis, &game_editor_args.window_size, ui, blue_engine_args, window)
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
                                            scene.flameobject_selected_parent_idx = i as u16;
                                            blue_flame_common::object_actions::create_shape(&flameobject.settings, &Project::selected_dir(projects), blue_engine_args, window);
                                            sub_editor_mode.create_new_object_window = false;
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
                        sub_editor_mode.create_new_object_window = false;
                    }
                }

                if ui.button(format!("{} Save current scene", game_editor_args.emojis.save)).clicked()
                || blue_engine_args.input.key_held(VirtualKeyCode::LControl) && blue_engine_args.input.key_pressed(VirtualKeyCode::S)
                //|| input.key_pressed(VirtualKeyCode::LControl || VirtualKeyCode::S)
                {
                    if blue_flame_common::db::scene::save(scene, &game_editor_args.filepaths.current_scene, &game_editor_args.current_project_dir) == true
                    {
                        crate::db::project_config::save(game_editor_args.project_config, game_editor_args.filepaths, &game_editor_args.current_project_dir);
                    }
                }

                ui.separator();
            }

            else if let ViewModes::Scenes = game_editor_args.viewmode
            {
                // Create new flameobject
                if ui.button(format!("{} New scene", game_editor_args.emojis.add)).clicked()
                {
                    for flameobject in scene.flameobjects.iter_mut()
                    {
                        blue_flame_common::object_actions::delete_shape(&flameobject.settings.label, blue_engine_args);
                    }

                    *scene = Scene::init(0);
                    game_editor_args.filepaths.current_scene = String::new();
                }

                if ui.button(format!("{} Save current scene", game_editor_args.emojis.save)).clicked()
                || blue_engine_args.input.key_held(VirtualKeyCode::LControl) && blue_engine_args.input.key_pressed(VirtualKeyCode::S)
                //|| input.key_pressed(VirtualKeyCode::LControl || VirtualKeyCode::S)
                {
                    if blue_flame_common::db::scene::save(scene, &game_editor_args.filepaths.current_scene, &game_editor_args.current_project_dir) == true
                    {
                        crate::db::project_config::save(game_editor_args.project_config, game_editor_args.filepaths, &game_editor_args.current_project_dir);
                    }
                }
            }
            else if let ViewModes::Blueprints = game_editor_args.viewmode
            {
                if ui.button(format!("{} Create object", game_editor_args.emojis.add)).clicked()
                {
                    blueprint.flameobject = Some(blue_flame_common::structures::flameobject::Settings::init(0, None));
                    sub_editor_mode.create_new_object_window = true;
                }
                if sub_editor_mode.create_new_object_window == true
                {
                    let mut cancel_creation_object = false; // If user presses cancel then pop from flameobjects
                    match crate::new_object_window(blueprint.flameobject.as_mut().unwrap(), projects, game_editor_args.emojis, game_editor_args.window_size,
                    ui, blue_engine_args, window)
                    {
                        Some(action) =>
                        {
                            match action
                            {
                                // ⛔ Cancel
                                false => sub_editor_mode.create_new_object_window = false,
                                // ➕ Create
                                true =>
                                {
                                    blue_flame_common::object_actions::create_shape(blueprint.flameobject.as_ref().unwrap(), &Project::selected_dir(projects), blue_engine_args, window);
                                    sub_editor_mode.create_new_object_window = false;
                                }
                            }
                        },
                        None => {}
                    }
                }
                // Top left hand side when in blueprint view mode
                if ui.button(format!("{} Save blueprint", game_editor_args.emojis.save)).clicked()
                || blue_engine_args.input.key_held(VirtualKeyCode::LControl) && blue_engine_args.input.key_pressed(VirtualKeyCode::S)
                {
                    //crate::save_blueprint(&blueprint.flameobject, &blueprint_savefolderpath, &current_project_dir);
                    match blueprint.flameobject
                    {
                        // WHen user preses save for blueprint object, any regular object inherited from blueprint and its changes will be affected 
                        Some(ref blueprint_flameobject) =>
                        {
                            for flameobject in scene.flameobjects.iter_mut()
                            {
                                match flameobject.settings.blueprint_key
                                {
                                    Some(ref blueprint_label) =>
                                    {
                                        if blueprint_label.0 == blueprint_flameobject.label && blueprint_label.1 == true
                                        {
                                            flameobject.settings.texture = blueprint_flameobject.texture.clone();
                                            flameobject.settings.color = blueprint_flameobject.color.clone();
                                            flameobject.settings.rotation = blueprint_flameobject.rotation.clone();
                                            flameobject.settings.size = blueprint_flameobject.size.clone();
                                        }
                                    }
                                    None => continue,
                                }
                            }
                        },
                        None => {},
                    }
                    //db::blueprints::save(blueprint.flameobject.as_ref().unwrap(), &filepaths.current_scene, &current_project_dir);
                }
            }
        });

        ui.separator();
        // UndoRedo
        ui.label("Undo Redo");

        ui.horizontal(|ui|
        {
            if ui.button(format!("{} Undo", game_editor_args.emojis.undo_redo.undo)).clicked()
            || blue_engine_args.input.key_held(VirtualKeyCode::LControl) && blue_engine_args.input.key_pressed(VirtualKeyCode::Z)
            {
                scene.undo_redo.undo(&mut scene.flameobjects, &mut game_editor_args.widget_functions, &mut scene.flameobject_selected_parent_idx,
                    game_editor_args.current_project_dir, blue_engine_args, window);
            }
            if ui.button(format!("{} Redo", game_editor_args.emojis.undo_redo.redo)).clicked()
            || blue_engine_args.input.key_held(VirtualKeyCode::LControl) && blue_engine_args.input.key_pressed(VirtualKeyCode::Y)
            {
                scene.undo_redo.redo(&mut scene.flameobjects, &mut game_editor_args.widget_functions, &game_editor_args.current_project_dir, blue_engine_args, window);
            }
            if ui.button(format!("{} Clear buf", game_editor_args.emojis.trash)).clicked()
            {
                scene.undo_redo.clear_buffer();
            }
        });
        ui.separator();

        // Temporary solution, will remove it when file explorer can be integrated
        // Only created for testing purposes
        if let ViewModes::Objects = game_editor_args.viewmode
        {
            if ui.button(format!("{} Blueprint in main scene", game_editor_args.emojis.load)).clicked()
            {
                match blueprint.flameobject
                {
                    Some(ref value) => 
                    {
                        let len = scene.flameobjects.len() as u16;
                        scene.flameobjects.push(Flameobject::init(len, None));
                        scene.flameobjects[len as usize].settings = value.clone();
                        scene.flameobjects[len as usize].settings.blueprint_key = Some((String::from(format!("{}", value.label)), true));
                        Flameobject::change_choice(&mut scene.flameobjects, len);

                        scene.flameobject_selected_parent_idx = len;
                        blue_flame_common::object_actions::create_shape(&scene.flameobjects[len as usize].settings,
                            &Project::selected_dir(&projects), blue_engine_args, window);
                    }
                    None => println!("None in blueprint.flameobject"),
                }
            }
        }

        ui.separator();

        // Displays all flameobjects/scenes button
        if let ViewModes::Objects = game_editor_args.viewmode
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
                        if !blue_engine_args.input.key_held(VirtualKeyCode::LShift)
                        {
                            //Flameobject::change_choice(&mut scene.flameobjects, i as u16);
                            game_editor_args.string_backups.label = flameobject.settings.label.clone();
                            scene.flameobject_selected_parent_idx = i as u16;
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
                        ui.label(format!("{}", game_editor_args.emojis.eye));
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
            if change_choice == true {Flameobject::change_choice(&mut scene.flameobjects, scene.flameobject_selected_parent_idx)}
        }
        else if let ViewModes::Scenes = game_editor_args.viewmode
        {
            ui.label(format!("id: {}", &scene.id));
        }

        else if let ViewModes::Blueprints = game_editor_args.viewmode
        {
            ui.label("Load blueprint into scene:");
            //ui.add(egui::TextEdit::singleline(&mut blueprint.save_file_path));
            crate::directory_singleline(&mut blueprint.save_file_path, Some(game_editor_args.current_project_dir),
                FilePickerMode::OpenFile, true, ui, game_editor_args.emojis);
            if ui.button("Load blueprint").clicked()
            {
                crate::db::blueprint::load(&mut blueprint.flameobject, &blueprint.save_file_path, &game_editor_args.current_project_dir, blue_engine_args, window);
            }
        }

    });
}