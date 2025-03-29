use blue_engine::Window;
use blue_engine_utilities::egui::egui::{self, Ui};
use blue_flame_common::
{
    object_actions,
    radio_options::ViewModes,
    structures::
    {
        flameobject,
        structures::{BlueEngineArgs, GameEditorArgs},
    },
    EditorSettings
};



pub fn main(
    game_editor_args: &mut GameEditorArgs,
    blue_engine_args: &mut BlueEngineArgs,
    flameobject_settings: &mut flameobject::Settings,
    editor_settings: &EditorSettings,
    ui: &mut Ui,
    window: &Window,
)
{
    if let ViewModes::Objects = game_editor_args.viewmode
    {
        ui.label("Position");
        ui.horizontal(|ui|
        {
            let mut update_object = false;
            //let elements = flameobject.settings.position.elements();
            //widget_functions.flameobject_old = Some(flameobject_settings.clone());
    
            for (value, label) in flameobject_settings.position.elements()
            {
                ui.label(format!("{}:", label as char));
    
                // Use Response::changed or whatever to determine if the value has been changed
                if ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed)).changed()
                {
                    update_object = true;
                }
            }
    
            if update_object == true
            {
                object_actions::update_shape::position(flameobject_settings, blue_engine_args);
            }
        });
    }

    ui.separator();

    ui.label("Size");
    ui.horizontal(|ui|
    {
        let mut update_object = false;

        for (value, label) in flameobject_settings.size.elements()
        {
            ui.label(format!("{}:", label as char));

            // Use Response::changed or whatever to determine if the value has been changed
            if ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed)).changed()
            {
                update_object = true;
            }
        }

        if update_object == true
        {
            object_actions::update_shape::size(flameobject_settings, blue_engine_args, window);
        }
        
    });
    ui.separator();

    ui.label("Rotation");
    ui.horizontal(|ui|
    {
        
        let mut update_object = false;

        for (value, label) in flameobject_settings.rotation.elements()
        {
            ui.label(format!("{}:", label as char));

            // Use Response::changed or whatever to determine if the value has been changed
            if ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed)).changed()
            {
                update_object = true;
            }
        }

        if update_object == true
        {
            //object_actions::update_shape::rotation(flameobject_settings, blue_engine_args);
        }
    });
}