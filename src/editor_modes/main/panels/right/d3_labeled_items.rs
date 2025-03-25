use blue_engine::Window;
use blue_engine_utilities::egui::egui::{self, Ui};
use blue_flame_common::
{
    structures::
    {
        flameobject,
        structures::{BlueEngineArgs, GameEditorArgs, WhatChanged},
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
    ui.label("Position");
    ui.horizontal(|ui|
    {
        // Has user moved the shape or not
        let mut update_position = false;
        //let elements = flameobject.settings.position.elements();
        //widget_functions.flameobject_old = Some(flameobject_settings.clone());

        for (value, label) in flameobject_settings.position.elements()
        {
            ui.label(format!("{}:", label as char));

            // Use Response::changed or whatever to determine if the value has been changed
            let response = ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed));
            
            // Dragging and typing
            if response.changed()
            {
                game_editor_args.widget_functions.has_changed = Some(WhatChanged::Position);
                update_position = true;
            }

            // Saving to flameobjects_old
            // Typing
            /*
            if response.gained_focus()
            {
                //println!("response.gained_focus()");
                widget_functions.has_changed = Some(WhatChanged::Position);
            }
            */
            //if response.changed() && input.mouse_released(0) i.e. if it has lost focused/not being changed anymore the value you are done putting in the new value
            if /*Dragging*/ response.drag_stopped() && !response.gained_focus() || /*Typing*/ response.changed() && blue_engine_args.input.mouse_released(blue_engine::MouseButton::Left)
            {
                if let Some(WhatChanged::Position) = game_editor_args.widget_functions.has_changed
                {
                    //undo_redo.save_action(undo_redo::Action::Update((flameobject_settings, flameobject_selected_parent_idx)));
                    //println!("save position undoredo");
                    //save_2_undoredo = true;
                    game_editor_args.widget_functions.has_changed = None;
                }
                
            }
        }

        // Updates the shape's position if the user has changed its value
        if update_position == true
        {
            blue_flame_common::object_actions::update_shape::position(flameobject_settings, blue_engine_args);
        }
        

        
    });
    ui.separator();

    ui.label("Size");
    ui.horizontal(|ui|
    {
        // Has user moved the shape or not
        let mut update_size = false;
        
        for (value, label) in flameobject_settings.size.elements()
        {
            ui.label(format!("{}:", label as char));

            // Use Response::changed or whatever to determine if the value has been changed
            let response = ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed));
            if response.changed()
            {
                //println!("Changed!");
                //widget_functions.has_changed = Some(WhatChanged::Size);
                update_size = true;
            }
            //if /*Dragging*/ response.drag_released() && !response.gained_focus() || /*Typing*/ response.changed() && blue_engine_args.input.mouse_released(0)
            if /*Dragging*/ response.drag_stopped() && !response.gained_focus() || /*Typing*/ response.changed() && blue_engine_args.input.mouse_released(blue_engine::MouseButton::Left)
            {
                
            }
            
        }
        // Updates the shape's size if the user has changed its value
        if update_size == true
        {
            //println!("update_position: {update_position}");
            blue_flame_common::object_actions::update_shape::size(flameobject_settings, blue_engine_args, window);
        }
        
    });
    ui.separator();

    ui.label("Rotation");
    ui.horizontal(|ui|
    {
        
        for (value, label) in flameobject_settings.rotation.elements()
        {
            ui.label(format!("{}:", label as char));

            // Use Response::changed or whatever to determine if the value has been changed
            if ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed)).changed()
            {
                /*
                blue_flame_common::object_actions::update_shape::rotation
                (

                )
                */
            }
            
        }
    });
}