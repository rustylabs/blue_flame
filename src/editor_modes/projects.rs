use blue_engine_egui::{self, egui::{self, Ui, InputState, Context}};
use crate::WindowSize;

pub fn main_scene(window_size: &WindowSize, ctx: &Context)
{
    egui::Window::new("Project")
    .collapsible(false)
    .fixed_pos(egui::pos2(0f32, 0f32))
    .fixed_size(egui::vec2(window_size.x, window_size.y))
    //.open(&mut open_projects)
    .show(ctx, |ui|
    {
    });
}