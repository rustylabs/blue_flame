use blue_engine_egui::{self, egui::{self, Ui, InputState, Context}};
use blue_engine::header::VirtualKeyCode;
use blue_engine::Window;
use blue_flame_common::emojis::Emojis;
use blue_flame_common::structures::{flameobject::Flameobject, flameobject::Settings};
use crate::{Scene, WindowSize, Project, FilePaths, StringBackups, WidgetFunctions, ProjectConfig, EditorModes, ViewModes, AlertWindow, BlueEngineArgs, EditorSettings,
    MouseFunctions,
};

pub fn main(alert_window: &mut [AlertWindow], blue_engine_args: &mut BlueEngineArgs, scene: &mut Scene, filepaths: &mut FilePaths, current_project_dir: &mut String)
{
    // One of the settings menu if opened
    /*
    egui::Window::new(AlertWindow::whats_enabled(&alert_window))
    .fixed_pos(egui::pos2(400f32, 50f32))
    .fixed_size(egui::vec2(100f32, 200f32))
    .open(&mut alert_window.0)
    .show(blue_engine_args.ctx, |ui|
    {
        ui.label("")
    });
    */

    // Menu bar
    egui::TopBottomPanel::top("Menu Bar").show(blue_engine_args.ctx, |ui|
    {
        //ui.set_enabled(!alert_window.0);

        egui::menu::bar(ui, |ui|
        {
            ui.menu_button("Menu", |ui|
            {
                for list in alert_window.iter_mut()
                {
                    // Individual elements after clicking on "Menu"
                    if ui.button(&list.label).clicked()
                    {
                        if list.label == "💾 Save"
                        {
                            blue_flame_common::db::scene::save(&scene, &filepaths.current_scene, &current_project_dir);
                            break;
                        }

                        //alert_window.0 = true;
                        list.state = true;
                    }
                    /*
                    else if alert_window.0 == false
                    {
                        list.state = false;
                    }
                    */
                }

            });
            ui.menu_button("About", |ui|
            {
                //if ui.bu
            });

        });
    });
}