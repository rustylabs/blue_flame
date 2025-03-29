use blue_engine::Window;
use crate::{Scene, Project, FilePaths, ProjectConfig, ViewModes, AlertWindow, BlueEngineArgs, EditorSettings, Blueprint, GameEditorArgs, editor_mode_variables};

use crate::editor_modes::main::panels;

/*
pub fn main(alert_window: &mut [AlertWindow], scene: &mut Scene, flameobject_blueprint: &mut Option<Settings>, previous_viewmode: &mut ViewModes,
    projects: &mut Vec<Project>, filepaths: &mut FilePaths, string_backups: &mut StringBackups, emojis: &Emojis, blueprint_savefolderpath: &mut String,
    enable_shortcuts: &mut bool,
    editor_settings: &EditorSettings,
    widget_functions: &mut WidgetFunctions, project_config: &mut ProjectConfig, current_project_dir: &mut String, editor_modes: &mut EditorModes,
    window_size: &WindowSize,
    mouse_functions: &mut MouseFunctions,
    blue_engine_args: &mut BlueEngineArgs, window: &Window
)
*/
pub fn main(scene: &mut Scene, projects: &mut Vec<Project>, blueprint: &mut Blueprint,
    editor_settings: &EditorSettings,
    game_editor_args: &mut GameEditorArgs, alert_window: &mut [AlertWindow], blue_engine_args: &mut BlueEngineArgs, window: &Window) -> bool // Return to change editor_mode
{
    let change_editor_mode = false;

    panels::menu_bar::main(alert_window, blue_engine_args, game_editor_args, scene);
    panels::left::main(scene, projects, blueprint, game_editor_args, editor_settings, blue_engine_args, window);
    panels::right::main::main(scene, projects, blueprint, editor_settings, game_editor_args, blue_engine_args, window);



    if let ViewModes::Objects = game_editor_args.viewmode
    {
        if *game_editor_args.enable_shortcuts == true
        {
            match crate::right_click_menu(game_editor_args.mouse_functions, blue_engine_args.input, blue_engine_args.ctx)
            {
                Some(object_type_captured) => crate::CreateNewFlameObject::flameobject(Some(&object_type_captured), scene,
                    &mut game_editor_args.widget_functions, &game_editor_args.current_project_dir, &editor_settings, blue_engine_args, window, None, 
                    None),
                None => {},
            }
        }
    }
    else if let ViewModes::Blueprints = game_editor_args.viewmode
    {
        if *game_editor_args.enable_shortcuts == true
        {
            match crate::right_click_menu(game_editor_args.mouse_functions, blue_engine_args.input, blue_engine_args.ctx)
            {
                Some(object_type_captured) => crate::CreateNewFlameObject::blueprint(&object_type_captured, &mut blueprint.flameobject_settings, &game_editor_args.current_project_dir, blue_engine_args, window),
                None => {},
            }
        }
    }


    return change_editor_mode;
}

// Used when choosing different scenes
pub fn load_scene_by_file(scene: &mut Scene, current_project_dir: &str, filepaths: &mut FilePaths,
    project_config: &mut ProjectConfig,
    blue_engine_args: &mut BlueEngineArgs, window: &Window)
{
    if blue_flame_common::db::scene::load(scene, current_project_dir, &filepaths.current_scene, true,
        blue_engine_args, window) == true
    {
        project_config.last_scene_filepath = filepaths.current_scene.clone();
        crate::db::project_config::save(project_config, filepaths, &current_project_dir);

    }
}