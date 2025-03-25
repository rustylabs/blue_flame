use blue_engine_utilities::egui::egui::{self, Ui};
use blue_flame_common::
{
    radio_options::FilePickerMode,
    EditorSettings,

    structures::
    {
        emojis::EMOJIS,
        flameobject::
        {
            self, shape_2d_3d_specific_settings::shape_2d_settings::
            {AnimatedSprites, Shape2DSettings}, Shape2D3DSepcificSettings::D2, Texture
        },
        structures::{GameEditorArgs, BlueEngineArgs, Project},
    },
};


use crate::directory_singleline;



pub fn main(
    game_editor_args: &mut GameEditorArgs,
    blue_engine_args: &mut BlueEngineArgs,
    flameobject_settings: &mut flameobject::Settings,
    editor_settings: &EditorSettings,
    projects: &Vec<Project>,
    ui: &mut Ui
)
{
    egui::CollapsingHeader::new("TextureMode").default_open(false).show(ui, |ui|
    {
        ui.horizontal(|ui|
        {
            ui.label("Location of Texture");
        
            // Only show "New sprite for animation" button if we are in 2D mode
            if let D2(Shape2DSettings{ref mut animated_sprites}) = flameobject_settings.shape_2d_3d_specific_settings
            {
                if ui.button(format!("{} New sprite for animation", EMOJIS.addition.plus)).clicked()
                {
                    // If no animated sprites (i.e. multiple sprites/textures) create it then
                    if let None = animated_sprites
                    {
                        *animated_sprites = Some(AnimatedSprites::init());
                        //flameobject_settings.shape_2d_3d_specific_settings = D2(Shape2DSettings{animated_sprites: Some(vec![Texture::init()])});
                    }
                    // Push another value into it
                    else if let Some(animated_sprites) = animated_sprites
                    {
                        animated_sprites.sprites.push(Texture::init());
                    }
                }
            }
        });
            
        //let response = ui.add(egui::TextEdit::singleline(&mut flameobject_settings.texture.file_location));
        // Shows primary sprite/texture
        
        //let mut response = None;
        ui.horizontal(|ui|
        {
            let response = directory_singleline(&mut flameobject_settings.texture.file_location,
                Some(game_editor_args.current_project_dir), FilePickerMode::OpenFile, true, ui);
            
            // Clear texture field
            if ui.button(format!("{}", EMOJIS.trash)).clicked()
            {
                flameobject_settings.texture.file_location = String::new();
            }
        
            if response.0.changed() || response.1 == true
            {
                *game_editor_args.enable_shortcuts = false;
                blue_flame_common::object_actions::update_shape::texture(flameobject_settings, &Project::selected_dir(&projects), blue_engine_args);
            }
        });
        
        // For sprites/animations
        if let D2(Shape2DSettings{ref mut animated_sprites}) = flameobject_settings.shape_2d_3d_specific_settings
        {
            let mut remove_idx: Option<usize> = None;
            if let Some(animated_sprites) = animated_sprites
            {
                ui.separator();
                ui.label("Animation control");
        
                // Show rest of animated sprites/textures
                for (i, sprite) in animated_sprites.sprites.iter_mut().enumerate()
                {
                    egui::CollapsingHeader::new("Sprite Animations")
                    .default_open(true)
                    .show(ui, |ui|
                    {
                        ui.horizontal(|ui|
                        {
                            directory_singleline(&mut sprite.file_location,
                                Some(game_editor_args.current_project_dir), FilePickerMode::OpenFile, true, ui);
            
                            // Delete the animated sprite
                            if ui.button(format!("{}", EMOJIS.trash)).clicked()
                            {
                                remove_idx = Some(i);
                            }
                        });
                    });

                }
        
                if let Some(idx) = remove_idx
                {
                    animated_sprites.sprites.remove(idx);
                }
        
                // Shows animation speed the user wishes to set it at
                ui.horizontal(|ui|
                {
                    ui.label("Animation speed: ");
                    ui.add(egui::DragValue::new(&mut animated_sprites.animation_speed).speed(editor_settings.slider_speed));
                });
            }
        
            // Decides whether to close the seperate animated sprite area or if length is 0
            {
                let mut make_animated_sprites_none = false;
                if let Some(animated_sprites) = animated_sprites
                {
                    if animated_sprites.sprites.len() < 1
                    {
                        make_animated_sprites_none = true;
                    }
                }
        
                if make_animated_sprites_none == true
                {
                    *animated_sprites = None;
                }
            }
        }
        
        if ui.button("Invert filepath type").clicked()
        {
            /*
            for texture_file_location in flameobject_settings.texture.file_location.iter_mut()
            {
                *texture_file_location = invert_pathtype(texture_file_location, &game_editor_args.current_project_dir);
            }
            */
        }
        
        // Radio buttons for texturemodes
        {
            use blue_flame_common::radio_options::Texture;
            //let elements = Texture::elements();
        
            for element in Texture::elements()
            {
                if ui.radio_value(&mut flameobject_settings.texture.mode, element, Texture::label(&element)).changed()
                {
                    blue_flame_common::object_actions::update_shape::texture(flameobject_settings, &Project::selected_dir(&projects), blue_engine_args);
                }
            }
        }
    });
    

}


