use std::{fs, path::PathBuf};

use blue_engine_utilities::egui::{egui, egui::Ui};
use blue_engine::{Window, header::KeyCode};
use blue_flame_common::{EditorSettings, radio_options::FilePickerMode,
    structures::{flameobject::Flameobject, emojis::EMOJIS, structures::MouseFunctions}};
use crate::{editor_mode_variables, editor_modes::main::main::load_scene_by_file, BlueEngineArgs, Blueprint, FilePaths, GameEditorArgs, Project, ProjectConfig, Scene, ViewModes, WidgetFunctions, WindowSize, FILE_EXTENSION_NAMES
};

pub fn main(scene: &mut Scene, projects: &mut Vec<Project>, blueprint: &mut Blueprint, sub_editor_mode: &mut editor_mode_variables::main::Main, game_editor_args: &mut GameEditorArgs,
    editor_settings: &EditorSettings,
    blue_engine_args: &mut BlueEngineArgs, window: &Window)
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

                if ui.button(format!("{} Save current scene", EMOJIS.save)).clicked()
                || blue_engine_args.input.key_held(KeyCode::ControlLeft) && blue_engine_args.input.key_pressed(KeyCode::KeyS)
                //|| input.key_pressed(KeyCode::ControlLeft || KeyCode::KeyS)
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
                if ui.button(format!("{} New scene", EMOJIS.addition.plus)).clicked()
                {
                    for flameobject in scene.flameobjects.iter_mut()
                    {
                        blue_flame_common::object_actions::delete_shape(&flameobject.settings.label, blue_engine_args);
                    }

                    *scene = Scene::init(0);
                    game_editor_args.filepaths.current_scene = String::new();
                }

                if ui.button(format!("{} Save current scene", EMOJIS.save)).clicked()
                || blue_engine_args.input.key_held(KeyCode::ControlLeft) && blue_engine_args.input.key_pressed(KeyCode::KeyS)
                //|| input.key_pressed(KeyCode::ControlLeft || KeyCode::KeyS)
                {
                    if blue_flame_common::db::scene::save(scene, &game_editor_args.filepaths.current_scene, &game_editor_args.current_project_dir) == true
                    {
                        crate::db::project_config::save(game_editor_args.project_config, game_editor_args.filepaths, &game_editor_args.current_project_dir);
                    }
                }
            }
            else if let ViewModes::Blueprints = game_editor_args.viewmode
            {
                // WHen user preses save for blueprint object, any regular object inherited from blueprint and its changes will be affected
                // and also saves blueprint to its current assigned dir
                // Top left hand side when in blueprint view mode
                if ui.button(format!("{} Save blueprint", EMOJIS.save)).clicked()
                || blue_engine_args.input.key_held(KeyCode::ControlLeft) && blue_engine_args.input.key_pressed(KeyCode::KeyS)
                {
                    crate::db::blueprint::save(&blueprint.flameobject, &blueprint.save_file_path, &game_editor_args.current_project_dir);
                    match blueprint.flameobject
                    {
                        // Any regular object inherited from blueprint and its changes will be affected
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
            if ui.button(format!("{} Undo", EMOJIS.undo_redo.undo)).clicked()
            || blue_engine_args.input.key_held(KeyCode::ControlLeft) && blue_engine_args.input.key_pressed(KeyCode::KeyZ)
            {
                scene.undo_redo.undo(&mut scene.flameobjects, &mut game_editor_args.widget_functions, &mut scene.flameobject_selected_parent_idx,
                    game_editor_args.current_project_dir, blue_engine_args, window);
            }
            if ui.button(format!("{} Redo", EMOJIS.undo_redo.redo)).clicked()
            || blue_engine_args.input.key_held(KeyCode::ControlLeft) && blue_engine_args.input.key_pressed(KeyCode::KeyY)
            {
                scene.undo_redo.redo(&mut scene.flameobjects, &mut game_editor_args.widget_functions, &game_editor_args.current_project_dir, blue_engine_args, window);
            }
            if ui.button(format!("{} Clear buf", EMOJIS.trash)).clicked()
            {
                scene.undo_redo.clear_buffer();
            }
        });
        ui.separator();

        // Temporary solution, will remove it when file explorer can be integrated
        // Only created for testing purposes
        if let ViewModes::Objects = game_editor_args.viewmode
        {
            if ui.button(format!("{} Blueprint in main scene", EMOJIS.load)).clicked()
            {
                match blueprint.flameobject
                {
                    Some(ref value) =>
                    {
                        /*
                        let len = scene.flameobjects.len() as u16;
                        scene.flameobjects.push(Flameobject::init(len, None));
                        scene.flameobjects[len as usize].settings = value.clone();
                        scene.flameobjects[len as usize].settings.blueprint_key = Some((String::from(format!("{}", value.label)), true));
                        Flameobject::change_choice(&mut scene.flameobjects, len);

                        scene.flameobject_selected_parent_idx = len;
                        blue_flame_common::object_actions::create_shape(&scene.flameobjects[len as usize].settings,
                            &Project::selected_dir(&projects), blue_engine_args, window);
                        */
                        crate::CreateNewFlameObject::flameobject(None,
                            scene, game_editor_args.widget_functions,
                            &game_editor_args.current_project_dir, &editor_settings, blue_engine_args, window, Some(value))
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
                        if !blue_engine_args.input.key_held(KeyCode::ShiftLeft)
                        {
                            //Flameobject::change_choice(&mut scene.flameobjects, i as u16);
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
                        ui.label(format!("{}", EMOJIS.eye));
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
            //ui.plus(egui::TextEdit::singleline(&mut blueprint.save_file_path));
            crate::directory_singleline(&mut blueprint.save_file_path, Some(game_editor_args.current_project_dir),
                FilePickerMode::OpenFile, true, ui);
            if ui.button("Load blueprint").clicked()
            {
                crate::db::blueprint::load(&mut blueprint.flameobject, &blueprint.save_file_path, &game_editor_args.current_project_dir, true,
                    blue_engine_args, window);
            }
        }


    });
}