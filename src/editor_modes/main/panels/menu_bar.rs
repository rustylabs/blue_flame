use std::f32::consts::E;

use blue_engine_utilities::egui::{egui, egui::{Ui, InputState, Context}};
use blue_engine::{header::KeyCode, Camera};
use blue_engine::Window;
use blue_flame_common::{emojis::EMOJIS, structures::GameEditorArgs};
use blue_flame_common::structures::{flameobject::Flameobject, flameobject::Settings};
use crate::{Scene, WindowSize, Project, FilePaths, StringBackups, WidgetFunctions, ProjectConfig, ViewModes, AlertWindow, BlueEngineArgs, EditorSettings,
    MouseFunctions,
};

pub fn main(alert_window: &mut [AlertWindow], blue_engine_args: &mut BlueEngineArgs, game_editor_args: &mut GameEditorArgs, scene: &mut Scene)
{
    let current_project_dir: &str = &game_editor_args.current_project_dir;
    let filepaths = &mut game_editor_args.filepaths;
    let project_config = &mut game_editor_args.project_config;
    let camera = &mut blue_engine_args.camera;
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

            ui.horizontal_centered(|ui|
            {
                zoom(project_config, camera, ui, blue_engine_args.input);
            });
            /*
            ui.centered_and_justified(|ui|
            {
                ui.button(format!("{}", EMOJIS.addition.minus));
                ui.button(format!("{}", EMOJIS.addition.plus));

            });
            */



        });
    });
}

fn zoom(project_config: &mut ProjectConfig, camera: &mut Camera, ui: &mut Ui, input: &blue_engine::InputHelper)
{

    const INCREMENT_ZOOM: u16 = 5;
    const INCREMENT_CAMERA: f32 = 1f32;
    /* For spacing
    ui.available_width()
    ui.space((ui.available_width() - 50.0) / 2)
    ui.reserve_[something I can't remember]()
    */
    fn change_camera_position(project_config: &mut ProjectConfig, camera: &mut Camera, increment: f32 /*-1 or 1*/)
    {
        //project_config.camera_position.zoom += -10;
        let mut camera_position = camera.position;

        camera_position.y -= INCREMENT_CAMERA * increment;

        camera.set_position([camera_position.x, camera_position.y, camera_position.z]);
    }
    // camera.set_position()

    

    // || blue_engine_args.input.key_held(VirtualKeyCode::LControl) && blue_engine_args.input.key_pressed(VirtualKeyCode::S)
    ui.label("Zoom: ");

    // Zoom out
    if ui.button(format!("{}", EMOJIS.addition.minus)).clicked()
    // || input.key_pressed(VirtualKeyCode::Scroll)
    {
        change_camera_position(project_config, camera, -1f32);
    }

    ui.add(egui::DragValue::new(&mut project_config.camera_position.zoom).speed(INCREMENT_ZOOM));

    // Zoom in
    if ui.button(format!("{}", EMOJIS.addition.plus)).clicked()
    {
        change_camera_position(project_config, camera, 1f32);
    }
}