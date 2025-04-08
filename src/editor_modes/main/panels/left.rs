use blue_engine_utilities::egui::egui;
use blue_engine::{Window, KeyCode};
use blue_flame_common::{object_actions, structures::{emojis::EMOJIS, flameobject::{self, Flameobject}}, EditorSettings, FileExtensionNames};
use rfd::FileDialog;
use crate::{BlueEngineArgs, Blueprint, CreateNewFlameObject, GameEditorArgs, Project, Scene, ViewModes};

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
                            object_actions::create_shape(flameobject_settings, blue_engine_args, window)
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
            ui.horizontal(|ui|
            {
                // WHen user preses save for blueprint object, any regular object inherited from blueprint and its changes will be affected
                // and also saves blueprint to its current assigned dir
                // Top left hand side when in blueprint view mode
                if ui.button(format!("{} Save blueprint", EMOJIS.save)).clicked()
                || blue_engine_args.input.key_held(KeyCode::ControlLeft) && blue_engine_args.input.key_pressed(KeyCode::KeyS)
                {
                    if let Some(ref blueprint_settings) = blueprint.flameobject_settings
                    {
                        // If &blueprint.save_file_path is empty, prompt user to save the file before proceeding
                        if blueprint.save_file_path.is_empty()
                        {
                            if let Some(file ) = FileDialog::new().set_directory(&game_editor_args.current_project_dir)
                            .add_filter(FileExtensionNames::name(FileExtensionNames::Blueprint), &[format!("{}", FileExtensionNames::extension_name(FileExtensionNames::Blueprint))])
                            .set_file_name(FileExtensionNames::add_extension_name(FileExtensionNames::Blueprint, &blueprint_settings.label))
                            .save_file()
                            {
                                blueprint.save_file_path = file.to_str().unwrap().to_string();
                            }
                        }
        
                        crate::db::blueprint::save(&blueprint.flameobject_settings, &blueprint.save_file_path);
                        
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
                        crate::db::blueprint::load(&mut blueprint.flameobject_settings, file.to_str().unwrap(), 
                        true, blue_engine_args, window);
    
                        blueprint.save_file_path = file.to_str().unwrap().to_string();
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
                    if game_editor_args.filepaths.current_scene.is_empty()
                    {
                        if let Some(file ) = FileDialog::new().set_directory(&game_editor_args.current_project_dir)
                        .add_filter(FileExtensionNames::name(FileExtensionNames::Scene), &[FileExtensionNames::extension_name(FileExtensionNames::Scene)])
                        .set_file_name(FileExtensionNames::add_extension_name(FileExtensionNames::Scene, &scene.label))
                        .save_file()
                        {
                            game_editor_args.filepaths.current_scene = file.display().to_string();
                        }
                    }

                    if blue_flame_common::db::scene::save(scene, &game_editor_args.filepaths.current_scene) == true
                    {
                        crate::db::project_config::save(game_editor_args.project_config, game_editor_args.filepaths);
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
                    if blue_flame_common::db::scene::save(scene, &game_editor_args.filepaths.current_scene) == true
                    {
                        crate::db::project_config::save(game_editor_args.project_config, game_editor_args.filepaths);
                    }
                }
            }
        });

        ui.separator();

        // Create object or blueprint as a child to the selected object
        // Button is tmp will be changed later
        // Child buttons
        {
            let mut child_buttons = ||
            {
                ui.label("Child object");
                ui.horizontal(|ui|
                {
                    if ui.button(format!("{} Create child object", EMOJIS.addition.plus)).clicked()
                    {
                        create_child_object(&mut scene.flameobjects, None, game_editor_args.current_project_dir, blue_engine_args, window);
                    }
                    if ui.button(format!("{} Add child blueprint", EMOJIS.file_icons.file)).clicked()
                    {
                        if let Some(file ) = FileDialog::new().set_directory(&game_editor_args.current_project_dir)
                        .add_filter(FileExtensionNames::name(FileExtensionNames::Blueprint), &[format!("{}", FileExtensionNames::extension_name(FileExtensionNames::Blueprint))])
                        .pick_file()
                        {
                            crate::db::blueprint::load(&mut blueprint.flameobject_settings, file.to_str().unwrap(), false, blue_engine_args, window);
        
                            create_child_object(&mut scene.flameobjects, blueprint.flameobject_settings.as_ref(), game_editor_args.current_project_dir, blue_engine_args, window);
                        }
                    }
                });
            };

            match game_editor_args.viewmode
            {
                ViewModes::Objects => {child_buttons()},
                ViewModes::Blueprints => {child_buttons()},
                ViewModes::Scenes => {},
            }
        }

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
        ui.separator();
        if let ViewModes::Objects = game_editor_args.viewmode
        {
            if ui.button(format!("{} Load blueprint into scene", EMOJIS.file_icons.file)).clicked()
            {
                if let Some(file ) = FileDialog::new().set_directory(&game_editor_args.current_project_dir).pick_file()
                {
                    crate::db::blueprint::load(&mut blueprint.flameobject_settings, file.to_str().unwrap(), false, blue_engine_args, window);

                    CreateNewFlameObject::flameobject(None,
                        scene, game_editor_args.widget_functions,
                        &game_editor_args.current_project_dir, blue_engine_args, window, blueprint.flameobject_settings.as_ref())
                }
            }
        }

        ui.separator();

        // Displays all flameobjects/scenes button
        if let ViewModes::Objects = game_editor_args.viewmode
        {
            // Only change chance labels if a single select was performed on labels
            let mut change_choice_id: Option<u16> = None;
            display_all_flameobjects(&mut scene.flameobjects, &mut change_choice_id, blue_engine_args, ui);
            // Disaplays all the objects in a list
            fn display_all_flameobjects(flameobjects: &mut Vec<Flameobject>, change_choice_id: &mut Option<u16>, blue_engine_args: &mut BlueEngineArgs, ui: &mut egui::Ui)
            {
                for flameobject in flameobjects.iter_mut()
                {
                    ui.horizontal(|ui|
                    {
                        ui.collapsing(format!("id: {}", flameobject.id), |ui|
                        {
                            display_all_flameobjects(&mut flameobject.children, change_choice_id, blue_engine_args, ui);
                        });
                        if ui.selectable_label(flameobject.selected, &flameobject.settings.label).clicked()
                        {
                            // if we are not attempting to select multiple items
                            //if !ui.input(|i| i.modifiers.shift_only())
                            if !blue_engine_args.input.key_held(KeyCode::ShiftLeft)
                            {
                                *change_choice_id = Some(flameobject.id);
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

            }
            if let Some(change_choice_id) = change_choice_id
            {
                Flameobject::change_choice(&mut scene.flameobjects, change_choice_id);
            }

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

fn create_child_object(flameobjects: &mut Vec<Flameobject>, blueprint: Option<&flameobject::Settings>, current_project_dir: &str, blue_engine_args: &mut BlueEngineArgs, window: &Window)
{
    // blueprint_flameobject should be None if creating just a child object, if we want to create from a blueprint as a child then should be assigned a value

    fn recursive_call(flameobjects: &mut Vec<Flameobject>, blueprint: Option<&flameobject::Settings>, selected_id: u16, new_id: u16, current_project_dir: &str, blue_engine_args: &mut BlueEngineArgs, window: &Window)
    {
        for flameobject in flameobjects.iter_mut()
        {
            recursive_call(&mut flameobject.children, blueprint, selected_id, new_id, current_project_dir, blue_engine_args, window);
            if flameobject.id == selected_id
            {
                //CreateNewFlameObject::child(flameobject, blueprint, new_id, current_project_dir, blue_engine_args, window);
                let is_blueprint;
                if let Some(_) = blueprint {is_blueprint = true}
                else {is_blueprint = false}

                flameobject.children.push(Flameobject::init(new_id, None, is_blueprint));
                let idx = flameobject.children.len() - 1;

                CreateNewFlameObject::child(&mut flameobject.children[idx], blueprint, current_project_dir, blue_engine_args, window);
            }
        }
    }
    if let Some(selected_id) = Flameobject::get_selected_id(flameobjects)
    {
        let new_id = Flameobject::get_available_id(flameobjects, None);
        recursive_call(flameobjects, blueprint, selected_id, new_id, current_project_dir, blue_engine_args, window);
        Flameobject::change_choice(flameobjects, new_id);
    }
}