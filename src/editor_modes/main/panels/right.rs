use blue_engine_egui::{self, egui::{self, Ui, InputState, Context}};
use blue_engine::header::VirtualKeyCode;
use blue_engine::Window;
use blue_flame_common::emojis::Emojis;
use blue_flame_common::structures::{flameobject::Flameobject, flameobject::Settings};
use crate::{Scene, WindowSize, Project, FilePaths, StringBackups, WidgetFunctions, ProjectConfig, EditorModes, ViewModes, AlertWindow, BlueEngineArgs, EditorSettings,
    MouseFunctions,
};

pub fn main(scene: &mut Scene, flameobject_blueprint: &mut Option<Settings>,
    projects: &mut Vec<Project>, filepaths: &mut FilePaths, string_backups: &mut StringBackups, emojis: &Emojis, blueprint_savefolderpath: &mut String,
    enable_shortcuts: &mut bool,
    editor_settings: &EditorSettings,
    widget_functions: &mut WidgetFunctions, project_config: &mut ProjectConfig, current_project_dir: &mut String, editor_modes: &mut EditorModes,
    mouse_functions: &mut MouseFunctions,
    blue_engine_args: &mut BlueEngineArgs, window: &Window)
{
    egui::SidePanel::right("Object Settings").show(blue_engine_args.ctx, |ui|
    {
        ui.set_width(ui.available_width());
        
        if let ViewModes::Objects = editor_modes.main.1
        {
            if scene.flameobjects.len() > 0 && crate::any_flameobject_selected(&scene.flameobjects)
            {
                let flameobject = &mut scene.flameobjects[scene.flameobject_selected_parent_idx as usize];

                //debug_ctrl_z_pressed1(&mut debug_ctrl_z_pressed, &mut scene, &mut label_backup);

                // label changes after here!
                crate::right_panel_flameobject_settings(&mut flameobject.settings, scene.flameobject_selected_parent_idx, flameobject.id, &mut scene.undo_redo,
                    enable_shortcuts, string_backups, current_project_dir, projects, editor_settings,
                    widget_functions, ui, blue_engine_args, window);
                
                //debug_ctrl_z_pressed1(&mut debug_ctrl_z_pressed, &mut scene, &mut label_backup);

            }
            if *enable_shortcuts == true
            {
                match crate::right_click_menu(mouse_functions, blue_engine_args.input, blue_engine_args.ctx)
                {
                    Some(object_type_captured) => crate::CreateNewFlameObject::flameobject(&object_type_captured, scene,
                        widget_functions, string_backups, &current_project_dir, &editor_settings, blue_engine_args, window),
                    None => {},
                }
            }
        }
        else if let ViewModes::Scenes = editor_modes.main.1
        {
            ui.label("Scene name:");
            ui.add(egui::TextEdit::singleline(&mut scene.label));
            ui.separator();

            ui.label("Save location:");
            ui.add(egui::TextEdit::singleline(&mut filepaths.current_scene));
            if ui.button("Invert filepath type").clicked()
            {
                filepaths.current_scene = crate::invert_pathtype(&filepaths.current_scene, &projects);
            }
            if ui.button("Load scene").clicked()
            {
                if blue_flame_common::db::scene::load(scene, &current_project_dir, &filepaths.current_scene, true,
                    blue_engine_args, window) == true
                {
                    project_config.last_scene_filepath = filepaths.current_scene.clone();
                    crate::db::project_config::save(project_config, filepaths, &current_project_dir);
                }
            }
            ui.separator();
            
            ui.label("High Power Mode:");
            ui.horizontal(|ui|
            {
                ui.checkbox(&mut scene.settings.high_power_mode, "high_power_mode");
            });
        }

        else if let ViewModes::Blueprints = editor_modes.main.1
        {
            match flameobject_blueprint
            {
                Some(ref mut flameobject_settings) => 
                {
                    crate::right_panel_flameobject_settings(flameobject_settings, scene.flameobject_selected_parent_idx, 0, &mut scene.undo_redo, enable_shortcuts, string_backups,
                        &current_project_dir, &projects, &editor_settings, widget_functions, ui, blue_engine_args, window);
                }
                None => {}
            }

            /*
            if enable_shortcuts == true {shortcut_commands(&mut scene.flameobjects, &mut flameobjects_selected_parent_idx, &mut editor_modes, &mut mouse_functions,
                &current_project_dir, &window_size,
                input, ctx, ui,
                renderer, objects, window)}
            */
            if *enable_shortcuts == true
            {
                match crate::right_click_menu(mouse_functions, blue_engine_args.input, blue_engine_args.ctx)
                {
                    Some(object_type_captured) => crate::CreateNewFlameObject::blueprint(&object_type_captured, flameobject_blueprint, &current_project_dir, blue_engine_args, window),
                    None => {},
                }
            }

        }

        for _ in 0..2
        {
            ui.separator();
        }

        // Blue print save related stuff
        if let ViewModes::Objects = editor_modes.main.1
        {
            // single line edit for blue print save location
            ui.add(egui::TextEdit::singleline(blueprint_savefolderpath));

            // blue print save button
            if ui.button(format!("{} Save blueprint", emojis.save)).clicked()
            {
                crate::save_blueprint(&flameobject_blueprint, &blueprint_savefolderpath, &current_project_dir);
            }
        }

        // Delete button
        ui.horizontal(|ui|
        {
            if let ViewModes::Objects = editor_modes.main.1
            {
                if ui.button(format!("{} Delete object", emojis.trash)).clicked()
                || blue_engine_args.input.key_pressed(VirtualKeyCode::X) && *enable_shortcuts == true
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
            else if let ViewModes::Scenes = editor_modes.main.1
            {
                if ui.button(format!("{} Delete scene", emojis.trash)).clicked()
                {
                }
            }
        });
    });
}