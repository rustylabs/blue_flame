use blue_engine_utilities::egui::{egui, egui::{Ui, InputState, Context}};
use blue_engine::{header::KeyCode, Camera};
use blue_engine::Window;
use blue_flame_common::emojis::Emojis;
use blue_flame_common::structures::{flameobject::Flameobject, flameobject::Settings};
use crate::{AlertWindow, BlueEngineArgs, Blueprint, EditorSettings, FilePaths, GameEditorArgs, MouseFunctions, Project, ProjectConfig, Scene, StringBackups, ViewModes, WidgetFunctions, WindowSize, FILE_EXTENSION_NAMES,
};
use blue_flame_common::radio_options::FilePickerMode;
use crate::editor_modes::main::main::load_scene_by_file;
/*
pub fn main(scene: &mut Scene, flameobject_blueprint: &mut Option<Settings>,
    projects: &mut Vec<Project>, filepaths: &mut FilePaths, string_backups: &mut StringBackups, emojis: &Emojis, blueprint_savefolderpath: &mut String,
    enable_shortcuts: &mut bool,
    editor_settings: &EditorSettings,
    widget_functions: &mut WidgetFunctions, project_config: &mut ProjectConfig, current_project_dir: &mut String, editor_modes: &mut EditorModes,
    mouse_functions: &mut MouseFunctions,
    blue_engine_args: &mut BlueEngineArgs, window: &Window)
*/
pub fn main(scene: &mut Scene, projects: &mut Vec<Project>, blueprint: &mut Blueprint, editor_settings: &EditorSettings,
    game_editor_args: &mut GameEditorArgs, blue_engine_args: &mut BlueEngineArgs, window: &Window)
{
    egui::SidePanel::right("Object Settings").show(blue_engine_args.ctx, |ui|
    {
        ui.set_width(ui.available_width());
        
        if let ViewModes::Objects = game_editor_args.viewmode
        {
            if scene.flameobjects.len() > 0 && crate::any_flameobject_selected(&scene.flameobjects)
            {
                let flameobject = &mut scene.flameobjects[scene.flameobject_selected_parent_idx as usize];

                //debug_ctrl_z_pressed1(&mut debug_ctrl_z_pressed, &mut scene, &mut label_backup);

                // label changes after here!
                crate::right_panel_flameobject_settings(
                    &mut flameobject.settings,
                    scene.flameobject_selected_parent_idx,
                    flameobject.id,
                    projects,
                    &mut scene.undo_redo,
                    editor_settings,
                    game_editor_args,
                    blue_engine_args,
                    ui,
                    window)
                /*
                crate::right_panel_flameobject_settings(&mut flameobject.settings, scene.flameobject_selected_parent_idx, flameobject.id, &mut scene.undo_redo,
                    enable_shortcuts, string_backups, current_project_dir, projects, editor_settings,
                    widget_functions, ui, blue_engine_args, window);
                */
                
                //debug_ctrl_z_pressed1(&mut debug_ctrl_z_pressed, &mut scene, &mut label_backup);

            }
            /*
            if *game_editor_args.enable_shortcuts == true
            {
                match crate::right_click_menu(game_editor_args.mouse_functions, blue_engine_args.input, blue_engine_args.ctx)
                {
                    Some(object_type_captured) => crate::CreateNewFlameObject::flameobject(&object_type_captured, scene,
                        &mut game_editor_args.widget_functions, game_editor_args.string_backups, &game_editor_args.current_project_dir, &editor_settings, blue_engine_args, window),
                    None => {},
                }
            }
            */
        }
        else if let ViewModes::Scenes = game_editor_args.viewmode
        {
            ui.label("Scene name:");
            ui.add(egui::TextEdit::singleline(&mut scene.label));
            ui.separator();

            ui.label("Save location:");
            //ui.add(egui::TextEdit::singleline(&mut game_editor_args.filepaths.current_scene));
            crate::directory_singleline(&mut game_editor_args.filepaths.current_scene,
                Some(game_editor_args.current_project_dir), FilePickerMode::SaveFile(FILE_EXTENSION_NAMES.scene), true, ui, game_editor_args.emojis);
            if ui.button("Invert filepath type").clicked()
            {
                game_editor_args.filepaths.current_scene = crate::invert_pathtype(&game_editor_args.filepaths.current_scene, &game_editor_args.current_project_dir);
            }
            if ui.button("Load scene").clicked()
            {
                load_scene_by_file(scene, game_editor_args.current_project_dir, game_editor_args.filepaths, &mut game_editor_args.string_backups.label,
                    game_editor_args.project_config, blue_engine_args, window);
                /*
                if blue_flame_common::db::scene::load(scene, &game_editor_args.current_project_dir, &game_editor_args.filepaths.current_scene, true,
                    blue_engine_args, window) == true
                {
                    game_editor_args.project_config.last_scene_filepath = game_editor_args.filepaths.current_scene.clone();
                    crate::db::project_config::save(game_editor_args.project_config, game_editor_args.filepaths, &game_editor_args.current_project_dir);
                }
                */
            }
            ui.separator();
            
            ui.label("High Power Mode:");
            ui.horizontal(|ui|
            {
                ui.checkbox(&mut scene.settings.high_power_mode, "high_power_mode");
            });
        }

        else if let ViewModes::Blueprints = game_editor_args.viewmode
        {
            match blueprint.flameobject
            {
                Some(ref mut flameobject_settings) => 
                {
                    crate::right_panel_flameobject_settings(flameobject_settings, scene.flameobject_selected_parent_idx, 0, &projects, &mut scene.undo_redo,
                        editor_settings, game_editor_args, blue_engine_args, ui, window);
                }
                None => {}
            }

            /*
            if enable_shortcuts == true {shortcut_commands(&mut scene.flameobjects, &mut flameobjects_selected_parent_idx, &mut editor_modes, &mut mouse_functions,
                &current_project_dir, &window_size,
                input, ctx, ui,
                renderer, objects, window)}
            */
            if *game_editor_args.enable_shortcuts == true
            {
                match crate::right_click_menu(game_editor_args.mouse_functions, blue_engine_args.input, blue_engine_args.ctx)
                {
                    Some(object_type_captured) => crate::CreateNewFlameObject::blueprint(&object_type_captured, &mut blueprint.flameobject, &game_editor_args.current_project_dir, blue_engine_args, window),
                    None => {},
                }
            }

        }

        for _ in 0..2
        {
            ui.separator();
        }

        // Blue print save related stuff
        if let ViewModes::Objects = game_editor_args.viewmode
        {
            // single line edit for blue print save location
            //ui.add(egui::TextEdit::singleline(&mut blueprint.save_file_path));
            ui.label("Save current shape as a blueprint");
            crate::directory_singleline(&mut blueprint.save_file_path, Some(game_editor_args.current_project_dir),
            FilePickerMode::SaveFile(FILE_EXTENSION_NAMES.blueprint), true, ui, game_editor_args.emojis);

            // blue print save button
            if ui.button(format!("{} Save current object as blueprint", game_editor_args.emojis.save)).clicked()
            {
                if scene.flameobjects.len() > 0
                {
                    blueprint.flameobject = Some(scene.flameobjects[scene.flameobject_selected_parent_idx as usize].settings.clone());
                }
                //crate::save_blueprint(&blueprint.flameobject, &blueprint.save_file_path, &game_editor_args.current_project_dir);
                crate::db::blueprint::save(&blueprint.flameobject, &blueprint.save_file_path, &game_editor_args.current_project_dir);
            }
        }

        // Delete button
        ui.horizontal(|ui|
        {
            if let ViewModes::Objects = game_editor_args.viewmode
            {
                if ui.button(format!("{} Delete object", game_editor_args.emojis.trash)).clicked()
                || blue_engine_args.input.key_pressed(KeyCode::KeyX) && *game_editor_args.enable_shortcuts == true
                {
                    scene.undo_redo.save_action(crate::undo_redo::Action::Delete(scene.flameobjects[scene.flameobject_selected_parent_idx as usize].copy()), &editor_settings);

                    let mut remove_indexes: Vec<usize> = Vec::new();
                    //let mut copy_over_undoredo: Vec<(flameobject::Flameobject, u16)> = Vec::new();
                    //let mut copy_over_undoredo: (u16, Vec<(flameobject::Flameobject, u16)>) = (0, Vec::new());
                    
                    // Deletes object from game engine and stores the index of vector to remove
                    for (i, flameobject) in scene.flameobjects.iter().enumerate()
                    {
                        if flameobject.selected == true
                        {
                            blue_flame_common::object_actions::delete_shape(&flameobject.settings.label, blue_engine_args);
                            remove_indexes.push(i);
                        }
                    }
                    // Removes any element in flameobjects from vector based on the remove_indexes vector
                    for remove_index in remove_indexes.iter().rev()
                    {
                        scene.flameobjects.remove(*remove_index);
                    }
                    //copy_over_undoredo.0 = scene.flameobject_selected_parent_idx;
                    
                    //Flameobject::recalculate_id(&mut scene.flameobjects);
                    //flameobjects_selected_parent_idx = (scene.flameobjects.len() - 1) as u16;

                    if scene.flameobjects.len() > 0
                    {
                        scene.flameobject_selected_parent_idx = scene.flameobjects.len() as u16 - 1;
                    }
                    else
                    {
                        scene.flameobject_selected_parent_idx = 0;
                    }
                }
            }
            else if let ViewModes::Scenes = game_editor_args.viewmode
            {
                if ui.button(format!("{} Delete scene", game_editor_args.emojis.trash)).clicked()
                {
                }
            }
        });
    });
}