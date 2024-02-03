use blue_engine_egui::{self, egui::{self, Ui, InputState, Context}};
use blue_engine::header::VirtualKeyCode;
use blue_engine::Window;
use blue_flame_common::emojis::Emojis;
use blue_flame_common::structures::{flameobject::Flameobject, flameobject::Settings};
use crate::{Scene, WindowSize, Project, FilePaths, StringBackups, WidgetFunctions, ProjectConfig, EditorModes, ViewModes, AlertWindow, BlueEngineArgs, EditorSettings,
    MouseFunctions,
};

use crate::editor_modes::main::panels;


pub fn main(alert_window: &mut [AlertWindow], scene: &mut Scene, flameobject_blueprint: &mut Option<Settings>, previous_viewmode: &mut ViewModes,
    projects: &mut Vec<Project>, filepaths: &mut FilePaths, string_backups: &mut StringBackups, emojis: &Emojis, blueprint_savefolderpath: &mut String,
    enable_shortcuts: &mut bool,
    editor_settings: &EditorSettings,
    widget_functions: &mut WidgetFunctions, project_config: &mut ProjectConfig, current_project_dir: &mut String, editor_modes: &mut EditorModes,
    window_size: &WindowSize,
    mouse_functions: &mut MouseFunctions,
    blue_engine_args: &mut BlueEngineArgs, window: &Window
)
{
    panels::menu_bar::main(alert_window, blue_engine_args, scene, filepaths, current_project_dir);
    panels::left::main(scene, flameobject_blueprint, previous_viewmode, projects, filepaths, string_backups, emojis, blueprint_savefolderpath, widget_functions, project_config, current_project_dir, editor_modes, window_size, blue_engine_args, window);
    panels::right::main(scene, flameobject_blueprint, projects, filepaths, string_backups,
    emojis, blueprint_savefolderpath, enable_shortcuts, editor_settings, widget_functions, project_config, current_project_dir, editor_modes, mouse_functions, blue_engine_args, window);
}