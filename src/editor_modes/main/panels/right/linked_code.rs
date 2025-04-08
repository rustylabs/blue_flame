use blue_engine_utilities::egui::egui::{self, Ui};
use rfd::FileDialog;
use blue_flame_common::
{
    FileExtensionNames,
    structures::
    {
        emojis::EMOJIS,
        flameobject,
        GameEditorArgs,
    },
};

pub fn main(
    game_editor_args: &mut GameEditorArgs,
    flameobject_settings: &mut flameobject::Settings,
    ui: &mut Ui,
)
{
    egui::CollapsingHeader::new(format!("{} Linked Code", EMOJIS.script)).default_open(false).show(ui, |ui|
    {
        /*
        directory_singleline(&mut flameobject_settings.linked_code,
            Some(game_editor_args.current_project_dir), FilePickerMode::OpenFile, true, ui);
        */

        if ui.button(format!("{}", EMOJIS.file_icons.file)).clicked()
        {
            if let Some(file ) = FileDialog::new().set_directory(&game_editor_args.current_project_dir)
            .add_filter("Rust", &["rs"])
            .pick_file()
            {
                flameobject_settings.linked_code = file.to_str().unwrap().to_string();
            }
        }

    });
}
