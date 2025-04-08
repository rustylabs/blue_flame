pub mod texture;
pub mod d3_labeled_items;
pub mod particle_system;
pub mod box_collider;
pub mod linked_code;

use blue_engine_utilities::egui::egui::{self, Ui};
use blue_engine::KeyCode;
use blue_engine::Window;
use blue_flame_common::structures::{emojis::EMOJIS, flameobject::{self, Flameobject}};
use blue_flame_common::FileExtensionNames;
use crate::{BlueEngineArgs, Blueprint, EditorSettings, GameEditorArgs, Project, Scene, ViewModes};
use rfd::FileDialog;
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
                if let Some(selected_id) = Flameobject::get_selected_id(&scene.flameobjects)
                {
                    fn recursive_call(flameobjects: &mut[Flameobject],
                        selected_id: u16, 
                        projects: &Vec<Project>,
                        blueprint: &mut Blueprint,
                        editor_settings: &EditorSettings,
                        game_editor_args: &mut GameEditorArgs,
                        blue_engine_args: &mut BlueEngineArgs,
                        ui: &mut Ui,
                        window: &Window,
                    )
                    {
                        for flameobject in flameobjects.iter_mut()
                        {
                            recursive_call(&mut flameobject.children,
                                selected_id,
                                projects,
                                blueprint,
                                editor_settings,
                                game_editor_args,
                                blue_engine_args,
                                ui,
                                window,
                            );
                            if flameobject.id == selected_id
                            {
                                right_panel_flameobject_settings(
                                    //&mut flameobject.settings,
                                    &mut flameobject.settings,
                                    editor_settings,
                                    game_editor_args,
                                    blue_engine_args,
                                    ui,
                                    window);

                                ui.separator();
                                // single line edit for blue print save location
                                //ui.add(egui::TextEdit::singleline(&mut blueprint.save_file_path));
                                ui.label("Save current shape as a blueprint");

                                // blue print save button
                                if ui.button(format!("{} Save current object as blueprint", EMOJIS.save)).clicked()
                                {
                                    if let Some(file ) = FileDialog::new().set_directory(&game_editor_args.current_project_dir)
                                    .add_filter(FileExtensionNames::name(FileExtensionNames::Blueprint), &[FileExtensionNames::extension_name(FileExtensionNames::Blueprint)])
                                    .set_file_name(FileExtensionNames::add_extension_name(FileExtensionNames::Blueprint, &flameobject.settings.label))
                                    .save_file()
                                    {
                                        blueprint.save_file_path = file.display().to_string();
                                        blueprint.flameobject_settings = Some(flameobject.settings.clone());
                                        crate::db::blueprint::save(&blueprint.flameobject_settings, &blueprint.save_file_path);

                                    }
                                }
                            }
                        }
                    }
                    recursive_call(&mut scene.flameobjects,
                        selected_id,
                        projects,
                        blueprint,
                        editor_settings,
                        game_editor_args,
                        blue_engine_args,
                        ui,
                        window,
                    );
                }

            }
            else if let ViewModes::Scenes = game_editor_args.viewmode
            {
                ui.label("Scene name:");
                ui.add(egui::TextEdit::singleline(&mut scene.label));
                ui.separator();
    
                ui.label("Save location:");
                //ui.add(egui::TextEdit::singleline(&mut game_editor_args.filepaths.current_scene));
                /*
                crate::directory_singleline(&mut game_editor_args.filepaths.current_scene,
                    Some(game_editor_args.current_project_dir), FilePickerMode::SaveFile(FileExtensionNames::extension_name(FileExtensionNames::Scene)), true, ui);
                */
                if ui.button(format!("{} Save scene", EMOJIS.save)).clicked()
                {
                    if let Some(file ) = FileDialog::new().set_directory(&game_editor_args.current_project_dir)
                    .add_filter(FileExtensionNames::name(FileExtensionNames::Scene), &[format!("{}", FileExtensionNames::extension_name(FileExtensionNames::Scene))])
                    .set_file_name(FileExtensionNames::add_extension_name(FileExtensionNames::Scene, &scene.label))
                    .save_file()
                    {
                        game_editor_args.filepaths.current_scene = file.to_str().unwrap().to_string();
                    }
                    blue_flame_common::db::scene::save(&scene, &game_editor_args.filepaths.current_scene);
                }
                if ui.button("Load scene").clicked()
                {
                    if let Some(file ) = FileDialog::new().set_directory(&game_editor_args.current_project_dir)
                    .add_filter(FileExtensionNames::name(FileExtensionNames::Scene), &[format!("{}", FileExtensionNames::extension_name(FileExtensionNames::Scene))])
                    .set_file_name(FileExtensionNames::add_extension_name(FileExtensionNames::Scene, &scene.label))
                    .pick_file()
                    {
                        game_editor_args.filepaths.current_scene = file.display().to_string();

                        load_scene_by_file(scene, game_editor_args.current_project_dir, game_editor_args.filepaths,
                            game_editor_args.project_config, blue_engine_args, window);
                    }


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
                    right_panel_flameobject_settings(flameobject_settings,
                        editor_settings, game_editor_args, blue_engine_args, ui, window);
                }
            }
    
            for _ in 0..2
            {
                ui.separator();
            }
    
            // Delete button
            ui.horizontal(|ui|
            {
                if let ViewModes::Objects = game_editor_args.viewmode
                {
                    if ui.button(format!("{} Delete objects", EMOJIS.trash)).clicked()
                    || blue_engine_args.input.key_pressed(KeyCode::KeyX) && *game_editor_args.enable_shortcuts == true
                    {
                        //scene.undo_redo.save_action(crate::undo_redo::Action::Delete(scene.flameobjects[scene.flameobject_selected_parent_idx as usize].copy()), &editor_settings);
                        fn recursive_call(flameobjects: &mut Vec<Flameobject>, blue_engine_args: &mut BlueEngineArgs)
                        {
                            let mut remove_indexes: Vec<usize> = Vec::new();
                            //let mut copy_over_undoredo: Vec<(flameobject::Flameobject, u16)> = Vec::new();
                            //let mut copy_over_undoredo: (u16, Vec<(flameobject::Flameobject, u16)>) = (0, Vec::new());
                            
                            // Deletes object from game engine and stores the index of vector to remove
                            for (i, flameobject) in flameobjects.iter_mut().enumerate()
                            {
                                recursive_call(&mut flameobject.children, blue_engine_args);

                                if flameobject.selected == true
                                {
                                    fn object_actions_del_recursive_call(flameobjects: &[Flameobject], blue_engine_args: &mut BlueEngineArgs)
                                    {
                                        for flameobject in flameobjects.iter()
                                        {
                                            object_actions_del_recursive_call(&flameobject.children, blue_engine_args);
                                            blue_flame_common::object_actions::delete_shape(&flameobject.settings.label, blue_engine_args);
                                        }
                                    }
                                    object_actions_del_recursive_call(&flameobject.children, blue_engine_args);
                                    blue_flame_common::object_actions::delete_shape(&flameobject.settings.label, blue_engine_args);
                                    remove_indexes.push(i);
                                }
                            }
                            // Removes any element in flameobjects from vector based on the remove_indexes vector
                            for remove_index in remove_indexes.iter().rev()
                            {
                                flameobjects.remove(*remove_index);
                            }
                        }
                        recursive_call(&mut scene.flameobjects, blue_engine_args);

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
    texture::main(game_editor_args, blue_engine_args, flameobject_settings, editor_settings, ui);


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