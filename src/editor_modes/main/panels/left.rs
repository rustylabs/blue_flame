use std::{fs::{self, DirEntry}, path::PathBuf};

use blue_engine_egui::{self, egui::{self, Context, Response, Ui}};
use blue_engine::header::VirtualKeyCode;
use blue_engine::Window;
use blue_flame_common::{emojis::{self, Emojis}, filepath_handling::fullpath_to_relativepath, radio_options::FilePickerMode, structures::{FileExplorerContent, MouseFunctions}, undo_redo, EditorSettings};
use blue_flame_common::structures::{flameobject::Flameobject, flameobject::Settings};
use serde::de::value;
use crate::{editor_mode_variables, editor_modes::main::main::load_scene_by_file, BlueEngineArgs, Blueprint, FilePaths, GameEditorArgs, Project, ProjectConfig, Scene, StringBackups, ViewModes, WidgetFunctions, WindowSize, FILE_EXTENSION_NAMES
};
use crate::egui::Vec2;
/*
pub fn main(scene: &mut Scene, blueprint.flameobject: &mut Option<Settings>, previous_viewmode: &mut ViewModes,
    projects: &mut Vec<Project>, filepaths: &mut FilePaths, string_backups: &mut StringBackups, emojis: &Emojis, blueprint_savefolderpath: &mut String,
   widget_functions: &mut WidgetFunctions, project_config: &mut ProjectConfig, current_project_dir: &mut String, editor_modes: &mut EditorModes,
   window_size: &WindowSize,
   blue_engine_args: &mut BlueEngineArgs, window: &Window)
*/
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
                // Create new flameobject
                if ui.button(format!("{} Create object", game_editor_args.emojis.addition.plus)).clicked()
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
                if ui.button(format!("{} New scene", game_editor_args.emojis.addition.plus)).clicked()
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
                if ui.button(format!("{} Create object", game_editor_args.emojis.addition.plus)).clicked()
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
                // WHen user preses save for blueprint object, any regular object inherited from blueprint and its changes will be affected
                // and also saves blueprint to its current assigned dir
                // Top left hand side when in blueprint view mode
                if ui.button(format!("{} Save blueprint", game_editor_args.emojis.save)).clicked()
                || blue_engine_args.input.key_held(VirtualKeyCode::LControl) && blue_engine_args.input.key_pressed(VirtualKeyCode::S)
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
                            scene, game_editor_args.widget_functions, game_editor_args.string_backups,
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
            //ui.plus(egui::TextEdit::singleline(&mut blueprint.save_file_path));
            crate::directory_singleline(&mut blueprint.save_file_path, Some(game_editor_args.current_project_dir),
                FilePickerMode::OpenFile, true, ui, game_editor_args.emojis);
            if ui.button("Load blueprint").clicked()
            {
                crate::db::blueprint::load(&mut blueprint.flameobject, &blueprint.save_file_path, &game_editor_args.current_project_dir, true,
                    blue_engine_args, window);
            }
        }



        // File explorer seperator
        if game_editor_args.file_explorer_contents.0 == false
        {
            FileExplorerWidget::init(game_editor_args);
        }
        FileExplorerWidget::display(scene, blueprint, editor_settings, sub_editor_mode, game_editor_args, blue_engine_args, ui, window);
        //file_explorer_widget(ui, game_editor_args);

    });
}

struct FileExplorerWidget;
impl FileExplorerWidget
{
    fn refresh(file_explorer_contents: &mut Vec<FileExplorerContent>, paths_new: &mut Vec<FileExplorerContent>,
        reading_dir: &str, subdir_level: u16)
    {
        //println!("reading_dir: {reading_dir}");
        let paths_new_tmp = fs::read_dir(format!("{}", reading_dir)).unwrap();

        //let mut paths_new: Vec<FileExplorerContent> = Vec::new();

        for path_new_tmp in paths_new_tmp
        {
            paths_new.push(FileExplorerContent
            {
                subdir_level,
                is_collapsed: true,
                selected: false,
                actual_content: path_new_tmp.unwrap(),
                childrens_content: None,
            });
        }

        for file_explorer_content in file_explorer_contents.iter_mut()
        {
            for path_new in paths_new.iter_mut()
            {
                if path_new.actual_content.file_name() == file_explorer_content.actual_content.file_name()
                {
                    path_new.subdir_level = file_explorer_content.subdir_level;
                    path_new.is_collapsed = file_explorer_content.is_collapsed;
                    path_new.selected = file_explorer_content.selected;

                    // If we have expanded the folder to view sub folders/files then load up childrens_content via recursive calls
                    if path_new.is_collapsed == false
                    {
                        path_new.childrens_content = Some(Vec::new());
                        //path_new.childrens_content = Some(Vec::new());
                        Self::refresh(
                            file_explorer_content.childrens_content.as_mut().unwrap(),
                            path_new.childrens_content.as_mut().unwrap(),
                            file_explorer_content.actual_content.path().to_str().unwrap(),
                            subdir_level + 1
                        );
                    }

                }
            }
        }
    }
    // Retrives all the dirs at parent level for the first time and pushes it to variable
    fn init(game_editor_args: &mut GameEditorArgs)
    {
        let mut file_explorer_contents = &mut game_editor_args.file_explorer_contents;
        let current_project_dir: &str = &game_editor_args.current_project_dir;

        let paths = fs::read_dir(format!("{}", &current_project_dir)).unwrap();
        //game_editor_args.file_explorer_contents.contents_retrieved = Some(Vec::new());
        file_explorer_contents.1 = Some(Vec::new());

        for path in paths
        {
            if let Some(ref mut value) = file_explorer_contents.1
            {
                //Self::retrieve_and_push_dirs(path.unwrap().file_name().to_str().unwrap(), value, 0)
                value.push(FileExplorerContent
                {
                    subdir_level: 0,
                    is_collapsed: true,
                    selected: false,
                    actual_content: path.unwrap(),
                    childrens_content: None,
                });
            }
        }

        file_explorer_contents.0 = true;
    }
    fn display(scene: &mut Scene, blueprint: &mut Blueprint, editor_settings: &EditorSettings, sub_editor_mode: &mut editor_mode_variables::main::Main,
        game_editor_args: &mut GameEditorArgs, blue_engine_args: &mut BlueEngineArgs, ui: &mut Ui, window: &Window)
    {

        let current_project_dir: &str = &game_editor_args.current_project_dir;
        let emojis = game_editor_args.emojis;
        let file_explorer_contents = &mut game_editor_args.file_explorer_contents.1;
        let window_size = game_editor_args.window_size;
        let mouse_functions = &mut game_editor_args.mouse_functions;

        for _ in 0..2
        {
            ui.separator();
        }
        ui.label("File Explorer");

        // Temperary for debugging, will remove when auto refresh is actually implemented
        if ui.button("Refresh").clicked()
        {
            let mut paths_new: Vec<FileExplorerContent> = Vec::new();
            Self::refresh(
                file_explorer_contents.as_mut().unwrap(),
                &mut paths_new,
                current_project_dir,
                0,
            );

            *file_explorer_contents = Some(paths_new);
        }
        /*
        if ui.button("print all contents").clicked()
        {
            //content.actual_content.file_name().to_str().unwrap(),
            if let Some(contents) = file_explorer_contents
            {
                for content in contents.iter()
                {
                    println!("content: {}, is_file: {}", content.actual_content.file_name().to_str().unwrap(), content.actual_content.path().is_file());
                }
                for _ in 0..2
                {
                    println!("---------------------");
                }
            }
        }
        */
        //let mut idx_make_selected: Option<usize> = None; // Make everything false but the one thing that was selected
        // Used to determine which file will be selected
        struct ChangeSelection
        {
            file_clicked: bool, // Used to determine if we need to change what file is selected
            coordinates: Vec<u16>, // Stores the coordinates of which file was selected
        }
        let mut change_selection = ChangeSelection{file_clicked: false, coordinates: Vec::new()};




        //egui::ScrollArea::vertical().show(ui, |ui|
        egui::ScrollArea::vertical().show(ui, |ui|
        {
            let response = ui.interact(ui.available_rect_before_wrap(), egui::Id::new("right_click_detector"), egui::Sense::click());
            if response.secondary_clicked()
            {
                mouse_functions.captured_coordinates = blue_engine_args.input.mouse().unwrap_or_default();
                sub_editor_mode.file_explorer.show_rightclick_menu = true;
            }

            actually_display(scene, blueprint, emojis, file_explorer_contents,
                editor_settings, game_editor_args.filepaths,
                game_editor_args.project_config, &mut change_selection, current_project_dir, game_editor_args.string_backups, sub_editor_mode, mouse_functions,
                game_editor_args.widget_functions, 
                window_size, blue_engine_args, ui, window);


            

            // plusing seperaters
            for _ in 0..2
            {
                ui.separator();
            }

        });

        // Right click menu
        if sub_editor_mode.file_explorer.show_rightclick_menu == true
        {
            egui::Area::new("right click").fixed_pos(egui::pos2(mouse_functions.captured_coordinates.0, mouse_functions.captured_coordinates.1))
            .show(blue_engine_args.ctx, |ui|
            {
                ui.visuals_mut().button_frame = false;
                egui::Frame::menu(&egui::Style::default()).show(ui, |ui|
                {
                    if ui.button(format!("{} New folder", emojis.addition.plus)).clicked()
                    {
                        sub_editor_mode.file_explorer.show_newfolder_wind = true;
                        sub_editor_mode.file_explorer.show_rightclick_menu = false;
                    }
                    // Only show delete option if an item is selected
                    if find_selected_item(file_explorer_contents) == true
                    {
                        if ui.button(format!("{} Delete item", emojis.trash)).clicked()
                        {
                            sub_editor_mode.file_explorer.show_deleteitem_wind = true;
                            sub_editor_mode.file_explorer.show_rightclick_menu = false;
                        }
                    }
                    fn find_selected_item(file_explorer_contents: &mut Option<Vec<FileExplorerContent>>) -> bool
                    {
                        //fullpath_to_relativepath(&content.actual_content.path().display().to_string(), current_project_dir),
                        if let Some(contents) = file_explorer_contents
                        {
                            for content in contents.iter_mut()
                            {
                                if content.selected == true
                                {
                                    return true;
                                }
                                find_selected_item(&mut content.childrens_content);
                            }
                        }
                        return false;
                    }

                });

                // Disable right click menu
                if blue_engine_args.input.key_pressed(VirtualKeyCode::Escape)
                {
                    sub_editor_mode.file_explorer.show_rightclick_menu = false;
                }
            });
        }

        // Delete item window
        if sub_editor_mode.file_explorer.show_deleteitem_wind == true
        {
            // Shows window
            egui::Window::new("Delete item")
            .fixed_pos(egui::pos2(window_size.x/2f32, window_size.y/2f32))
            .pivot(egui::Align2::CENTER_CENTER)
            .default_size(egui::vec2(window_size.x/2f32, window_size.y/2f32))
            .resizable(true)
            //.open(&mut _create_new_project)
            .show(blue_engine_args.ctx, |ui|
            {
                ui.label("Are you sure you want to delete the selected item?");
                ui.horizontal(|ui|
                //ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui|
                {
                    // Don't delete item
                    if ui.button(format!("{} No", emojis.cancel)).clicked()
                    {
                        sub_editor_mode.file_explorer.show_deleteitem_wind = false;
                    }
                    // Delete item
                    if ui.button(format!("{} Yes", emojis.tick)).clicked()
                    {
                        let mut selected_dir = PathBuf::from(current_project_dir.clone());
                        find_selected_item(file_explorer_contents, &mut selected_dir);
                        fn find_selected_item(file_explorer_contents: &mut Option<Vec<FileExplorerContent>>, selected_dir: &mut PathBuf)
                        {
                            //fullpath_to_relativepath(&content.actual_content.path().display().to_string(), current_project_dir),
                            if let Some(contents) = file_explorer_contents
                            {
                                for content in contents.iter_mut()
                                {
                                    if content.selected == true
                                    {
                                        selected_dir.push(content.actual_content.path());
                                        if content.actual_content.path().is_dir() == true
                                        {
                                            match std::fs::remove_dir_all(selected_dir.display().to_string())
                                            {
                                                Ok(_) => {},
                                                Err(e) => println!("Could not delete dir due to: {e}"),
                                            }
                                        }
                                        else if content.actual_content.path().is_file() == true
                                        {
                                            match std::fs::remove_file(selected_dir.display().to_string())
                                            {
                                                Ok(_) => {},
                                                Err(e) => println!("Could not delete file due to: {e}"),
                                            }
                                        }
                                        //content.actual_content.file_name().to_str().unwrap()
                                        //println!("{}", content.actual_content.file_name().to_str().unwrap());
                                        //println!("{:?}", file_explorer_contents);
                                        break;
                                    }
                                    find_selected_item(&mut content.childrens_content, selected_dir);
                                }
                            }
                        }

                        sub_editor_mode.file_explorer.show_deleteitem_wind = false;
                    }
                });
            });
        }

        // New folder window
        if sub_editor_mode.file_explorer.show_newfolder_wind == true
        {
            // Shows window
            egui::Window::new("New Folder")
            .fixed_pos(egui::pos2(window_size.x/2f32, window_size.y/2f32))
            .pivot(egui::Align2::CENTER_CENTER)
            .default_size(egui::vec2(window_size.x/2f32, window_size.y/2f32))
            .resizable(true)
            //.open(&mut _create_new_project)
            .show(blue_engine_args.ctx, |ui|
            {
                ui.label("New folder:");
                ui.add(egui::TextEdit::singleline(&mut sub_editor_mode.file_explorer.new_folder_name));

                // Closes the window
                fn close_window(sub_editor_mode: &mut editor_mode_variables::main::Main)
                {
                    sub_editor_mode.file_explorer.new_folder_name = String::new();
                    sub_editor_mode.file_explorer.show_newfolder_wind = false;
                }

                ui.horizontal(|ui|
                {
                    // Don't create folder
                    if ui.button(format!("{} Cancel", emojis.cancel)).clicked()
                    {
                        close_window(sub_editor_mode);
                    }
                    // Creates folder
                    if ui.button(format!("{} Create", emojis.addition.plus)).clicked()
                    {
                        let mut create_new_dir = PathBuf::from(current_project_dir.clone());

                        find_selected_item(file_explorer_contents, &mut create_new_dir);

                        fn find_selected_item(file_explorer_contents: &mut Option<Vec<FileExplorerContent>>, create_new_dir: &mut PathBuf)
                        {
                            //fullpath_to_relativepath(&content.actual_content.path().display().to_string(), current_project_dir),
                            if let Some(contents) = file_explorer_contents
                            {
                                for content in contents.iter_mut()
                                {
                                    if content.selected == true && content.actual_content.path().is_dir() == true
                                    {
                                        create_new_dir.push(content.actual_content.path().display().to_string());
                                        break;
                                    }
                                    find_selected_item(&mut content.childrens_content, create_new_dir);
                                }
                            }
                        }
                        create_new_dir.push(sub_editor_mode.file_explorer.new_folder_name.clone());
                        println!("create_new_dir: {}", create_new_dir.display().to_string());

                        match std::fs::create_dir(create_new_dir.display().to_string())
                        {
                            Ok(_) => {},
                            Err(e) => println!("Unable to create new dir due to: {}", e),
                        }

                        close_window(sub_editor_mode);
                    }
                    // Disable new folder window and clears anything for new folder
                    if blue_engine_args.input.key_pressed(VirtualKeyCode::Escape)
                    {
                        close_window(sub_editor_mode);
                    }
                });





            });
        }

        // Reselect files
        if change_selection.file_clicked == true
        {
            let coordinate_element = change_selection.coordinates[0];
            //println!("file coordinates: {:?}", change_selection.coordinates);
            deselect_all_content(file_explorer_contents);
            if let Some(ref mut file_explorer_contents) = file_explorer_contents
            {
                select_file(&mut file_explorer_contents[coordinate_element as usize], &change_selection.coordinates);
            }
            //select_file(file_explorer_contents.as_ref().unwrap(), &change_selection.coordinates);
        }

        fn deselect_all_content(file_explorer_contents: &mut Option<Vec<FileExplorerContent>>,)
        {
            if let Some(contents) = file_explorer_contents
            {
                for content in contents.iter_mut()
                {
                    content.selected = false;
                    deselect_all_content(&mut content.childrens_content);
                }
            }
        }
        fn select_file(file_explorer_contents: &mut FileExplorerContent, coordinates: &[u16])
        {
            let current_subdir_level = file_explorer_contents.subdir_level as usize;
            // The subdir level will be used as the coordinate's array idx
            if current_subdir_level < coordinates.len() - 1
            {
                if let Some(ref mut file_explorer_contents) = file_explorer_contents.childrens_content
                {
                    select_file(&mut file_explorer_contents[coordinates[current_subdir_level+1] as usize], coordinates);
                }
            }
            // On last cordinate
            else
            {
                file_explorer_contents.selected = true;
            }

        }

        // Recursive calls
        fn actually_display(
            scene: &mut Scene,
            blueprint: &mut Blueprint,
            emojis: &Emojis,
            file_explorer_contents: &mut Option<Vec<FileExplorerContent>>,
            editor_settings: &EditorSettings,
            filepaths: &mut FilePaths,
            project_config: &mut ProjectConfig,
            change_selection: &mut ChangeSelection,
            current_project_dir: &str,
            string_backups: &mut StringBackups,
            sub_editor_mode: &mut editor_mode_variables::main::Main,
            mouse_functions: &mut MouseFunctions,
            widget_functions: &mut WidgetFunctions,
            window_size: &WindowSize,
            blue_engine_args: &mut BlueEngineArgs,
            ui: &mut Ui,
            window: &Window,
        )
        {
            if let Some(contents) = file_explorer_contents
            {
                //let emojis = game_editor_args.emojis;
                //let current_project_dir: &str = &game_editor_args.current_project_dir;

                let mut idx_make_selected: Option<usize> = None; // Make everything false but the one thing that was selected

                if change_selection.file_clicked == false
                {
                    change_selection.coordinates.push(0);
                }

                let change_selection_len = change_selection.coordinates.len() - 1;
                for (i, content) in contents.iter_mut().enumerate()
                {
                    if change_selection.file_clicked == false
                    {
                        change_selection.coordinates[change_selection_len] = i as u16;
                    }
                    

                    ui.horizontal(|ui|
                    {
                        // For subdirs, pad based on the subdir_level
                        for _ in 0..content.subdir_level
                        {
                            //ui.label("|");
                            // Times subdir_level by how many times?
                            for _ in 0..2
                            {
                                ui.label(" ");
                                //ui.label("_");
                            }
                        }
                        // Folder
                        if content.actual_content.path().is_dir()
                        {
                            let arrow = 
                            {
                                if content.is_collapsed == false
                                {
                                    emojis.arrows.down
                                }
                                else
                                {
                                    emojis.arrows.right
                                }
                            };
                            if ui.button(format!("{}", arrow)).clicked()
                            {
                                content.is_collapsed = !content.is_collapsed;
                                if let None = content.childrens_content
                                {
                                    content.childrens_content = Some(Vec::new());
                                    push_subdir(&content.actual_content.path().display().to_string(), &mut content.childrens_content, content.subdir_level);
                                }
                            }
                            let response = ui.selectable_label(content.selected, format!("{} {}",
                                emojis.file_icons.folder,
                                content.actual_content.file_name().to_str().unwrap(),
                                //fullpath_to_relativepath(&content.actual_content.path().display().to_string(), current_project_dir),
                            ));
                            if response.clicked() || response.secondary_clicked()
                            {
                                idx_make_selected = Some(i);
                                change_selection.file_clicked = true;
                            }
                            // Right clicked
                            if response.secondary_clicked()
                            {
                                sub_editor_mode.file_explorer.show_rightclick_menu = true;
                                mouse_functions.captured_coordinates = blue_engine_args.input.mouse().unwrap_or_default();
                            }
                            if response.double_clicked()
                            {
                                content.is_collapsed = !content.is_collapsed;
                                if let None = content.childrens_content
                                {
                                    content.childrens_content = Some(Vec::new());
                                    push_subdir(&content.actual_content.path().display().to_string(), &mut content.childrens_content, content.subdir_level);
                                }
                            }

                        }
                        // File
                        else if content.actual_content.path().is_file()
                        {
                            let mut is_doubleclicked = false;
                            let response = ui.selectable_label(content.selected, format!("{} {}",
                                emojis.file_icons.file,
                                content.actual_content.file_name().to_str().unwrap(),
                                //fullpath_to_relativepath(&content.actual_content.path().display().to_string(), current_project_dir),
                            ));
                            if response.clicked() || response.secondary_clicked()
                            {
                                idx_make_selected = Some(i);
                                change_selection.file_clicked = true;
                            }
                            if response.double_clicked()
                            {
                                is_doubleclicked = true;
                            }
    
                            // Open file if double clicked
                            if is_doubleclicked == true
                            {
                                let selected_file = content.actual_content.file_name().to_string_lossy().to_string();
                                //let selected_file = String::from("asd");

                                // Scene
                                if selected_file.ends_with(FILE_EXTENSION_NAMES.scene)
                                {
                                    filepaths.current_scene = selected_file;
                                    load_scene_by_file(scene, current_project_dir, filepaths, &mut string_backups.label, 
                                        project_config, blue_engine_args, window);
                                }
                                // Blueprint
                                else if selected_file.ends_with(FILE_EXTENSION_NAMES.blueprint)
                                //if selected_file.ends_with("FILE_EXTENSION_NAMES.blueprint")
                                {
                                    blueprint.save_file_path = selected_file;
    
                                    crate::db::blueprint::load(&mut blueprint.flameobject, &blueprint.save_file_path, current_project_dir,
                                        false, blue_engine_args, window);
    
                                    crate::CreateNewFlameObject::flameobject(None,
                                    scene, widget_functions, string_backups,
                                    current_project_dir, &editor_settings, blue_engine_args, window, blueprint.flameobject.as_ref())
                                }
                                // Texture
                                else
                                {
                                    // List of file extension names for images
                                    const EXT_NAMES: [&str; 2] =
                                    [
                                        ".png",
                                        ".jpg",
                                    ];
                                    for ext_name in EXT_NAMES.iter()
                                    {
                                        if selected_file.ends_with(ext_name)
                                        {
                                            for flameobject in scene.flameobjects.iter_mut()
                                            {
                                                //let flameobject_selected_parent_idx = scene.flameobject_selected_parent_idx;
                                                if flameobject.selected == true
                                                {
                                                    flameobject.settings.texture.file_location = crate::invert_pathtype(
                                                        &content.actual_content.path().display().to_string(), current_project_dir);

                                                    blue_flame_common::object_actions::update_shape::texture(&flameobject.settings,
                                                        current_project_dir,
                                                        blue_engine_args);
                                                    //println!("flameobject.settings.texture.file_location: {}", flameobject.settings.texture.file_location);
                                                    println!("flameobject_selected_parent_idx: {}", scene.flameobject_selected_parent_idx);
                                                    crate::change_texture(
                                                        &flameobject.settings,
                                                        string_backups,
                                                        widget_functions,
                                                        &mut scene.undo_redo,
                                                        flameobject.id,
                                                        editor_settings);
                                                }
                                            }
                                            
                                        }
                                    }
                                }
                            }
                        }
                    });

                    // Display subdirectories by calling itself
                    if content.is_collapsed == false
                    {
                        actually_display(scene, blueprint, emojis, &mut content.childrens_content,
                            editor_settings, filepaths,
                            project_config, change_selection, current_project_dir, string_backups, sub_editor_mode, mouse_functions,
                            widget_functions, window_size, blue_engine_args, ui, window);
                    }
                }
                if change_selection.file_clicked == false
                {
                    change_selection.coordinates.pop();
                }
            }
        }

        fn push_subdir(path: &str, file_explorer_contents: &mut Option<Vec<FileExplorerContent>>, subdir_level: u16)
        {
            let paths = fs::read_dir(format!("{}", path)).unwrap();

            for path in paths
            {
                if let Some(file_explorer_contents) = file_explorer_contents
                {
                    file_explorer_contents.push(FileExplorerContent
                    {
                        subdir_level: subdir_level + 1,
                        is_collapsed: true,
                        selected: false,
                        actual_content: path.unwrap(),
                        childrens_content: None,
                    });
                }

            }
        }


    }
}
