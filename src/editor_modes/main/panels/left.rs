use blue_engine_utilities::egui::egui;
use blue_engine::{Window, header::KeyCode};
use blue_flame_common::{object_actions, EditorSettings, radio_options::FilePickerMode, structures::{flameobject::Flameobject, emojis::EMOJIS}};
use rfd::FileDialog;
use crate::{invert_pathtype, BlueEngineArgs, Blueprint, CreateNewFlameObject, GameEditorArgs, Project, Scene, ViewModes, FILE_EXTENSION_NAMES};

pub fn main(scene: &mut Scene, projects: &mut Vec<Project>, blueprint: &mut Blueprint, game_editor_args: &mut GameEditorArgs,
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
                    if let ViewModes::Blueprints = game_editor_args.viewmode
                    {
                        // Remove all objects from scene then load or create a new object for blueprints variable
                        for flameobject in scene.flameobjects.iter()
                        {
                            object_actions::delete_shape(&flameobject.settings.label_key, blue_engine_args);
                        }

                        // If there is something inside blueprint variable, then load it into the scene
                        if let Some(ref flameobject_settings) = blueprint.flameobject_settings
                        {
                            object_actions::create_shape(flameobject_settings, &game_editor_args.current_project_dir, blue_engine_args, window)
                        }
                    }

                    // If any other tabs other than "Blueprints"
                    else
                    {
                        // Only check for if one of the flameobject's exists, if it doesn't then delete blueprint and load the entire project scene
                        if scene.flameobjects.len() > 0 && object_actions::if_object_exists(&scene.flameobjects[0].settings.label_key, blue_engine_args) == false
                        {
                            if let Option::Some(ref value) = blueprint.flameobject_settings
                            {
                                object_actions::delete_shape(&value.label, blue_engine_args);
                            }
                            crate::load_project_scene(true, scene, projects,  game_editor_args, blue_engine_args, window);
                        }
                    }
                }
            }
            
        });

        ui.separator();

        if let ViewModes::Blueprints = game_editor_args.viewmode
        {
            crate::directory_singleline(&mut blueprint.save_file_path, Some(game_editor_args.current_project_dir),
            FilePickerMode::SaveFile(FILE_EXTENSION_NAMES.blueprint), true, ui);
            

            ui.horizontal(|ui|
            {
                // WHen user preses save for blueprint object, any regular object inherited from blueprint and its changes will be affected
                // and also saves blueprint to its current assigned dir
                // Top left hand side when in blueprint view mode
                if ui.button(format!("{} Save blueprint", EMOJIS.save)).clicked()
                || blue_engine_args.input.key_held(KeyCode::ControlLeft) && blue_engine_args.input.key_pressed(KeyCode::KeyS)
                {
    
                    // If &blueprint.save_file_path is empty, prompt user to save the file before proceeding
                    if &blueprint.save_file_path == ""
                    {
                        if let Some(file ) = FileDialog::new().set_directory(&game_editor_args.current_project_dir).save_file()
                        {
                            blueprint.save_file_path = file.to_str().unwrap().to_string();
                            blueprint.save_file_path = invert_pathtype(&blueprint.save_file_path, &game_editor_args.current_project_dir);
    
                            if blueprint.save_file_path.contains(&format!("{}", FILE_EXTENSION_NAMES.blueprint)) == false
                            {
                                blueprint.save_file_path.push_str(&format!("{}", FILE_EXTENSION_NAMES.blueprint));
                            }
                        }
                    }
    
                    crate::db::blueprint::save(&blueprint.flameobject_settings, &blueprint.save_file_path, &game_editor_args.current_project_dir);
                    
                    // Any regular object inherited from blueprint and its changes will be affected
                    if let Some(ref blueprint_settings) = blueprint.flameobject_settings
                    {
                        // Goes through all flameobjects and the ones that have blueprint_key that matches the selected blueprint will have changes affected to them
                        for flameobject in scene.flameobjects.iter_mut()
                        {
                            // if flameobject is using a blueprint
                            if let Some(ref flameobject_blueprint) = flameobject.settings.blueprint_key
                            {
                                if flameobject_blueprint.0 == blueprint_settings.label_key && flameobject_blueprint.1 == true /*If flameobject's blueprint is subjected to change*/
                                {
                                    flameobject.settings.texture = blueprint_settings.texture.clone();
                                    flameobject.settings.color = blueprint_settings.color.clone();
                                    flameobject.settings.rotation = blueprint_settings.rotation.clone();
                                    flameobject.settings.size = blueprint_settings.size.clone();
                                }
                            }
                        }
                    }
                }

                if ui.button(format!("{} Load blueprint from file", EMOJIS.file_icons.file)).clicked()
                {
                    if let Some(file ) = FileDialog::new().set_directory(&game_editor_args.current_project_dir).pick_file()
                    {
                        crate::db::blueprint::load(&mut blueprint.flameobject_settings, file.to_str().unwrap(), &game_editor_args.current_project_dir, 
                        true, blue_engine_args, window);
    
                        blueprint.save_file_path = file.to_str().unwrap().to_string();
                        blueprint.save_file_path = invert_pathtype(&blueprint.save_file_path, &game_editor_args.current_project_dir);
                    }
                }

            });

        }

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
        });

        ui.separator();

        // Create object or blueprint as a child to the selected object
        // Button is tmp will be changed later
        ui.label("Child object");
        ui.horizontal(|ui|
        {
            if ui.button(format!("{} Create object", EMOJIS.addition.plus)).clicked()
            {
                let mut child_flameobject_selected_idx: Option<usize> = None;
                for (i, flameobject) in scene.flameobjects.iter_mut().enumerate()
                {
                    if flameobject.selected == true
                    {
                        child_flameobject_selected_idx = Some(i);
                        break;

                    }
                }

                if let Some(child_flameobject_selected_idx) = child_flameobject_selected_idx
                {
                    CreateNewFlameObject::flameobject(None,
                        scene, game_editor_args.widget_functions,
                        &game_editor_args.current_project_dir, &editor_settings, blue_engine_args, window, None,
                        Some(child_flameobject_selected_idx))
                }
            }
        });
        /*
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
        */
        // Open's blueprint file into main scene
        if let ViewModes::Objects = game_editor_args.viewmode
        {
            if ui.button(format!("{} Load blueprint into scene", EMOJIS.file_icons.file)).clicked()
            {
                if let Some(file ) = FileDialog::new().set_directory(&game_editor_args.current_project_dir).pick_file()
                {
                    crate::db::blueprint::load(&mut blueprint.flameobject_settings, file.to_str().unwrap(), &game_editor_args.current_project_dir, false, blue_engine_args, window);

                    blueprint.save_file_path = invert_pathtype(file.to_str().unwrap(), &game_editor_args.current_project_dir);


                    CreateNewFlameObject::flameobject(None,
                        scene, game_editor_args.widget_functions,
                        &game_editor_args.current_project_dir, &editor_settings, blue_engine_args, window, blueprint.flameobject_settings.as_ref(),
                        None)
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
                        if let Some(ref mut child_flameobject) = flameobject.child_flameobject
                        {
                            
                        }
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
            // This will clear the blueprint from scene and also make save location of blueprint empty
            if ui.button(format!("{} Clear blueprint from scene", EMOJIS.trash)).clicked()
            {
                if let Some(ref blueprint_settings) = blueprint.flameobject_settings
                {
                    object_actions::delete_shape(&blueprint_settings.label_key, blue_engine_args);
                }
                blueprint.save_file_path = "".to_string();
            }
        }


    });
}