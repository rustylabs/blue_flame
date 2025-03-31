pub mod texture;
pub mod d3_labeled_items;
pub mod particle_system;
pub mod box_collider;
pub mod linked_code;

use blue_engine_utilities::egui::egui::{self, Ui};
use blue_engine::header::KeyCode;
use blue_engine::Window;
use blue_flame_common::structures::{emojis::EMOJIS, flameobject};
use crate::{BlueEngineArgs, Blueprint, EditorSettings, GameEditorArgs, Project, Scene, ViewModes, FILE_EXTENSION_NAMES};
use blue_flame_common::radio_options::FilePickerMode;
use crate::editor_modes::main::load_scene_by_file;
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

        egui::ScrollArea::vertical().show(ui, |ui|
        {
            if let ViewModes::Objects = game_editor_args.viewmode
            {
                if scene.flameobjects.len() > 0 && scene.get_selected_flameobject_idx(&scene.flameobjects) != None
                {
                    let idx = scene.get_selected_flameobject_idx(&scene.flameobjects).unwrap();
                    let flameobject = &mut scene.flameobjects[idx];
    
                    right_panel_flameobject_settings(
                    &mut flameobject.settings,
                    projects,
                    editor_settings,
                    game_editor_args,
                    blue_engine_args,
                    ui,
                    window);
    
                }
            }
            else if let ViewModes::Scenes = game_editor_args.viewmode
            {
                ui.label("Scene name:");
                ui.add(egui::TextEdit::singleline(&mut scene.label));
                ui.separator();
    
                ui.label("Save location:");
                //ui.add(egui::TextEdit::singleline(&mut game_editor_args.filepaths.current_scene));
                crate::directory_singleline(&mut game_editor_args.filepaths.current_scene,
                    Some(game_editor_args.current_project_dir), FilePickerMode::SaveFile(FILE_EXTENSION_NAMES.scene), true, ui);
                if ui.button("Invert filepath type").clicked()
                {
                    game_editor_args.filepaths.current_scene = crate::invert_pathtype(&game_editor_args.filepaths.current_scene, &game_editor_args.current_project_dir);
                }
                if ui.button("Load scene").clicked()
                {
                    load_scene_by_file(scene, game_editor_args.current_project_dir, game_editor_args.filepaths,
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
                if let Some(ref mut flameobject_settings) = blueprint.flameobject_settings
                {
                    right_panel_flameobject_settings(flameobject_settings, &projects,
                        editor_settings, game_editor_args, blue_engine_args, ui, window);
                }
            }
    
            for _ in 0..2
            {
                ui.separator();
            }
    
            // Blue print save related stuff
            if let ViewModes::Objects = game_editor_args.viewmode
            {
                // If ONLY one object is selected then show save blueprint button
                if let Some(selected_flameobject_idx) = scene.get_selected_flameobject_idx(&scene.flameobjects)
                {
                    // single line edit for blue print save location
                    //ui.add(egui::TextEdit::singleline(&mut blueprint.save_file_path));
                    ui.label("Save current shape as a blueprint");
                    crate::directory_singleline(&mut blueprint.save_file_path, Some(game_editor_args.current_project_dir),
                    FilePickerMode::SaveFile(FILE_EXTENSION_NAMES.blueprint), true, ui);

                    // blue print save button
                    if ui.button(format!("{} Save current object as blueprint", EMOJIS.save)).clicked()
                    {
                        blueprint.flameobject_settings = Some(scene.flameobjects[selected_flameobject_idx].settings.clone());
                        crate::db::blueprint::save(&blueprint.flameobject_settings, &blueprint.save_file_path, &game_editor_args.current_project_dir);
                    }
                }
            }
    
            // Delete button
            ui.horizontal(|ui|
            {
                if let ViewModes::Objects = game_editor_args.viewmode
                {
                    if ui.button(format!("{} Delete object", EMOJIS.trash)).clicked()
                    || blue_engine_args.input.key_pressed(KeyCode::KeyX) && *game_editor_args.enable_shortcuts == true
                    {
                        //scene.undo_redo.save_action(crate::undo_redo::Action::Delete(scene.flameobjects[scene.flameobject_selected_parent_idx as usize].copy()), &editor_settings);
    
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
                    }
                }
                else if let ViewModes::Scenes = game_editor_args.viewmode
                {
                    if ui.button(format!("{} Delete scene", EMOJIS.trash)).clicked()
                    {
                    }
                }
            });
        });
        
        

    });
}

fn right_panel_flameobject_settings(
    flameobject_settings: &mut flameobject::Settings,
    projects: &Vec<Project>,
    editor_settings: &EditorSettings,
    game_editor_args: &mut GameEditorArgs,
    blue_engine_args: &mut BlueEngineArgs,
    ui: &mut Ui,
    window: &Window,
)
{
    

    // Object name
    ui.label(format!("label_key: {}", flameobject_settings.label_key));
    
    ui.add(egui::TextEdit::singleline(&mut flameobject_settings.label));


    // Blueprint label key
    ui.horizontal(|ui|
    {
        ui.label(format!("Blueprint label key: {}", match flameobject_settings.blueprint_key
        {
            Some(ref blueprint_key) => blueprint_key.0.clone(),
            None => "None".to_string(),
        }));
    
        match flameobject_settings.blueprint_key
        {
            Some(ref mut blueprint_key) => 
            {
                ui.checkbox(&mut blueprint_key.1, "Modify?");
            },
            None => {},
        }
    });

    
    ui.separator();
    texture::main(game_editor_args, blue_engine_args, flameobject_settings, editor_settings, projects, ui);


    new_sub_section("Color", ui);
    ui.horizontal(|ui|
    {
        if ui.color_edit_button_rgba_unmultiplied(&mut flameobject_settings.color).changed()
        {
            blue_flame_common::object_actions::update_shape::color(flameobject_settings, blue_engine_args);
        }
    });

    ui.separator();
    d3_labeled_items::main(game_editor_args, blue_engine_args, flameobject_settings, editor_settings, ui, window);

    ui.separator();
    particle_system::main(game_editor_args, flameobject_settings, editor_settings, ui);


    ui.separator();
    box_collider::main(flameobject_settings, editor_settings, ui);

    ui.separator();
    linked_code::main(game_editor_args, flameobject_settings, ui);


}

fn new_sub_section(label: &str, ui: &mut Ui)
{
    ui.separator();
    ui.label(&format!("{}", label));
}