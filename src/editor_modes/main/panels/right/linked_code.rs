use blue_engine_utilities::egui::egui::{self, Ui};
use blue_flame_common::
{
    radio_options::FilePickerMode,
    structures::
    {
        emojis::EMOJIS,
        flameobject, structures::GameEditorArgs
    },
};

use crate::directory_singleline;

pub fn main(
    game_editor_args: &mut GameEditorArgs,
    flameobject_settings: &mut flameobject::Settings,
    ui: &mut Ui,
)
{
    egui::CollapsingHeader::new(format!("{} Linked Code", EMOJIS.script)).default_open(false).show(ui, |ui|
    {
        directory_singleline(&mut flameobject_settings.linked_code,
            Some(game_editor_args.current_project_dir), FilePickerMode::OpenFile, true, ui);
    });
}
