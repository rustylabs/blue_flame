use blue_engine_utilities::egui::egui::{self, Ui};
use blue_flame_common::
{
    radio_options::FilePickerMode,
    structures::
    {
        emojis::EMOJIS,
        flameobject::{self, ParticleSystem},
        structures::GameEditorArgs
    },
    EditorSettings
};

use crate::directory_singleline;

pub fn main(
    game_editor_args: &mut GameEditorArgs,
    flameobject_settings: &mut flameobject::Settings,
    editor_settings: &EditorSettings,
    ui: &mut Ui,
)
{
    ui.collapsing("Particle System", |ui|
    {
        if ui.button(format!("{} Create Particle System", EMOJIS.addition.plus)).clicked()
        {
            // If new create new particle system
            if let None = flameobject_settings.particle_systems
            {
                flameobject_settings.particle_systems = Some(vec![ParticleSystem::init()]);
            }
            // If already exists then push to vector
            else if let Some(ref mut particle_systems) = flameobject_settings.particle_systems
            {
                particle_systems.push(ParticleSystem::init());
            }
        }
    
        if let Some(ref mut particle_systems) = flameobject_settings.particle_systems
        {
            let mut idx_remove: Option<usize> = None;
    
            for (i, particle_system) in particle_systems.iter_mut().enumerate()
            {

                egui::CollapsingHeader::new(format!("Particle System id: {}", i))
                .default_open(true)
                .show(ui, |ui|
                {
                    ui.checkbox(&mut particle_system.enabled, "Particle System Enabled");
    
                    ui.label("Texture");
                    ui.label("Location of Texture");
                    directory_singleline(&mut flameobject_settings.texture.file_location,
                        Some(game_editor_args.current_project_dir), FilePickerMode::OpenFile, true, ui);
        
        
                    ui.label("Offset");
                    ui.horizontal(|ui|
                    {
                        for (value, label) in particle_system.offset.elements()
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
                        for (value, label) in particle_system.size.elements()
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
                        for (value, label) in particle_system.rotation.elements()
                        {
                            // x, y, z
                            ui.label(format!("{}:", label as char));
                            if ui.add(egui::DragValue::new(value).speed(editor_settings.slider_speed)).changed()
                            {
                                
                            }
                        }
                    });
        
                    ui.separator();
                    if ui.button(format!("{} Delete Particle System", EMOJIS.trash)).clicked()
                    {
                        idx_remove = Some(i);
                    }
                });


            }
    
    
            // Delete the particle system vector element if submitted for removal
            if let Some(idx_remove) = idx_remove
            {
                particle_systems.remove(idx_remove);
            }
        }
    });



}