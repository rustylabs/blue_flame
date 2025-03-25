use blue_engine_utilities::egui::egui::{self, Ui};
use blue_flame_common::
{
    structures::
    {
        emojis::EMOJIS,
        flameobject::{self, BoxCollider},
    },
    EditorSettings
};

pub fn main(
    flameobject_settings: &mut flameobject::Settings,
    editor_settings: &EditorSettings,
    ui: &mut Ui,
)
{
    ui.collapsing("Box Collider", |ui|
    {
        if ui.button(format!("{} Create Box Collider", EMOJIS.addition.plus)).clicked()
        {
            // If new create new particle system
            if let None = flameobject_settings.box_colliders
            {
                flameobject_settings.box_colliders = Some(vec![BoxCollider::init(flameobject_settings.size.clone(), flameobject_settings.rotation.clone())]);
            }
            // If already exists then push to vector
            else if let Some(ref mut box_colliders) = flameobject_settings.box_colliders
            {
                box_colliders.push(BoxCollider::init(flameobject_settings.size.clone(), flameobject_settings.rotation.clone()));
            }
        }
    
        if let Some(ref mut box_colliders) = flameobject_settings.box_colliders
        {
            let mut idx_remove: Option<usize> = None;
    
            for (i, box_collider) in box_colliders.iter_mut().enumerate()
            {
                egui::CollapsingHeader::new(format!("Box Collider id: {}", i))
                .default_open(true)
                .show(ui, |ui|
                {
                    ui.checkbox(&mut box_collider.enabled, "Box Collider Enabled");
    
                    ui.label("Offset");
                    ui.horizontal(|ui|
                    {
                        for (value, label) in box_collider.offset.elements()
                        {
                            // x, y, z
                            ui.label(format!("{}:", label as char));
                            if ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed)).changed()
                            {
                                
                            }
                        }
                    });
                    ui.separator();
                    ui.label("Size");
                    ui.horizontal(|ui|
                    {
                        for (value, label) in box_collider.size.elements()
                        {
                            // x, y, z
                            ui.label(format!("{}:", label as char));
                            if ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed)).changed()
                            {
                                
                            }
                        }
                    });
                    ui.separator();
                    ui.label("Rotation");
                    ui.horizontal(|ui|
                    {
                        for (value, label) in box_collider.rotation.elements()
                        {
                            // x, y, z
                            ui.label(format!("{}:", label as char));
                            if ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed)).changed()
                            {
                                
                            }
                        }
                    });
                    
                    ui.separator();
                    if ui.button(format!("{} Delete Box Collider", EMOJIS.trash)).clicked()
                    {
                        idx_remove = Some(i);
                    }
                });

            }
    
            // Delete the particle system vector element if submitted for removal
            if let Some(idx_remove) = idx_remove
            {
                box_colliders.remove(idx_remove);
            }
        }
    });


}