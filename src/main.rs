use macroquad::prelude::*;
use std::process::exit;

pub mod object_settings;
pub mod sql;

mod practice;

struct Debug
{
    practice        : bool,
}

// Settings for the game editor
struct Settings
{
    slider_speed        : f32,
}
impl Settings
{
    // Creates defaults
    fn init() -> Self
    {
        Self
        {
            slider_speed        : 1f32,
        }
    }
}



mod issues
{

    pub struct Issues
    {
        pub warning         : bool,
        pub error           : bool,
    }
    impl Issues
    {
        pub fn init() -> Self
        {
            Self
            {
                warning     : false,
                error       : false,
            }
        }
    }

    pub fn output_symbols() -> (&'static str, &'static str)
    {
        ("⚠", "⛔")
    }

    pub mod issue_checks
    {
        pub fn labels(objects: &mut [(crate::Objects, crate::ObjectSettings)])
        {
            for i in 0..objects.len()
            {
                for j in 0..objects.len()
                {
                    if i != j
                    {
                        if objects[i].0.label.1.warning != true && objects[i].0.label.0 == objects[j].0.label.0
                        {
                            objects[i].0.label.1.warning = true;
                            break;
                        }
                        else
                        {
                            objects[i].0.label.1.warning = false;    
                        }
                    }
                }
            }
        }
    }
}




pub struct ObjectSettings
{
    object_type         : Vec<object_settings::radio_options::Fields>,
    position            : [object_settings::three_d_lables::Fields; 3],
    scale               : [object_settings::three_d_lables::Fields; 3],
    texture             : object_settings::texture::Fields,
}
impl ObjectSettings
{
    fn init() -> Self
    {
        Self
        {
            object_type         : object_settings::radio_options::init(&["Square", "Triangle", "Line"]),
            position            : object_settings::three_d_lables::Fields::init(0f32),
            scale               : object_settings::three_d_lables::Fields::init(1f32),
            texture             : object_settings::texture::Fields::init(),
        }
    }
}

pub struct Objects
{
    id          : u16,
    visible     : bool,
    selected    : bool,
    label       : (String, issues::Issues),
}
impl Objects
{
    fn init(id: u16) -> Self
    {
        Self
        {
            id,
            visible     : true,
            selected    : true,
            label       : (format!("object_{id}"), issues::Issues::init())
        }
    }
    fn change_choice(list: &mut [(Self, ObjectSettings)], choice_true: u16)
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            if i as u16 == choice_true
            {
                item.0.selected = true;
            }
            else
            {
                item.0.selected = false;
            }
        }
    }
    // When user deletes the objects, we need to re calculate ids
    fn recalculate_id(list: &mut  [(Self, ObjectSettings)])
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            item.0.id = i as u16;
        }
    }
    // Checks for warnings and errors for labels and assigns the Issues variables appropriately

}




#[macroquad::main("egui with macroquad")]
async fn main()
{
    let debug = Debug
    {
        practice       : false,
    };

    if debug.practice == true
    {
        println!("\n--------------Practice Mode!!!!!--------------\n");
        practice::main();
        println!("\n--------------End of practice!!!!!--------------\n");
        exit(0);
    }
    // Settings
    //const SLIDER_SPEED: f32     = 1f32;
    let settings = Settings::init();

    // object_settings Variables (no longer needed)
    //let mut object_settings = ObjectSettings::init();

    // sql Variables
    let mut sql = sql::Sql::init();

    // objects
    //let mut objects = vec![(Objects::init(0), ObjectSettings::init())];
    let mut objects = Vec::new();

    // Loads stuff from sql database file
    //sql.load(&mut object_type, &mut position, &mut scale, &mut texture);
    //sql.save(&mut object_type, &mut position, &mut scale, &mut texture);

    sql.load(&mut objects);

    loop
    {
        clear_background(WHITE);

        // Error checks

        // Label error checking
        issues::issue_checks::labels(&mut objects);
        


        // Left panel
        egui_macroquad::ui(|egui_ctx|
        {

            egui::SidePanel::left("Objects").show(egui_ctx, |ui|
            {
                ui.set_width(ui.available_width());

                ui.horizontal(|ui|
                {
                    // Create new object
                    if ui.button("➕ Create Object").clicked()
                    {
                        let len = objects.len() as u16;

                        objects.push((Objects::init(len), ObjectSettings::init()));
                        Objects::change_choice(&mut objects, len);
                    }
                    if ui.button("💾 Save").clicked()
                    {
                        sql.save(&objects);
                    }
                });


                ui.separator();

                // Displays all objects button
                for i in 0..objects.len()
                {
                    ui.horizontal(|ui|
                    {
                        ui.collapsing(format!("id: {}", &objects[i].0.id), |ui|
                        {
                            ui.label("some stuff");
                        });
                        if ui.selectable_label(objects[i].0.selected, &objects[i].0.label.0).clicked()
                        {
                            Objects::change_choice(&mut objects, i as u16);
                        }
                        ui.checkbox(&mut objects[i].0.visible, "");
                        if objects[i].0.visible == true
                        {
                            ui.label("👁");
                        }

                        // Checks if variable names are correct or not
                        // Warnings
                        if objects[i].0.label.1.warning == true
                        {
                            ui.label(issues::output_symbols().0);
                        }
                        // Errors
                        if objects[i].0.label.1.error == true
                        {
                            ui.label(issues::output_symbols().1);
                        }
                    });
                }
            });

            // Right panel
            egui::SidePanel::right("Object Settings")
                .show(egui_ctx, |ui|
                {
                    ui.set_width(ui.available_width());


                    for object in objects.iter_mut()
                    {

                        if object.0.selected == true
                        {
                            ui.label(format!("Object name: {} {}",
                                if object.0.label.1.warning == true {issues::output_symbols().0} else {""},
                                if object.0.label.1.error == true {issues::output_symbols().1} else {""},
                            ));
                            ui.add(egui::TextEdit::singleline(&mut object.0.label.0));
                        }
                    }

                    for object in objects.iter_mut()
                    {
                        if object.0.selected == true
                        {
                            ui.label("Object type");
                            ui.horizontal(|ui|
                            {
                                for i in 0..object.1.object_type.len()
                                {
                                    if ui.radio(object.1.object_type[i].status, object.1.object_type[i].name).clicked()
                                    {
                                        object_settings::radio_options::change_choice(&mut object.1.object_type, i as u8);
                                    }
                                }
                            });
                            ui.separator();

                            ui.label("TextureMode");
                            ui.label("Location of Texture");
                            ui.add(egui::TextEdit::singleline(&mut object.1.texture.data));


                            // Radio buttons for texturemodes
                            for i in 0..object.1.texture.mode.len()
                            {
                                if ui.radio(object.1.texture.mode[i].status, object.1.texture.mode[i].name).clicked()
                                {
                                    object_settings::radio_options::change_choice(&mut object.1.texture.mode, i as u8);
                                }
                            }
                            ui.separator();

                            ui.label("Position");
                            ui.horizontal(|ui|
                            {
                                for position in object.1.position.iter_mut()
                                {
                                    ui.label(format!("{}:", position.axis as char));
                                    ui.add(egui::DragValue::new(&mut position.value).speed(settings.slider_speed));
                                }
                            });
                            ui.separator();

                            //ui.label("Scale");
                            ui.horizontal(|ui|
                            {
                                for scale in object.1.scale.iter_mut()
                                {
                                    ui.label(format!("{}:", scale.axis as char));
                                    ui.add(egui::DragValue::new(&mut scale.value).speed(settings.slider_speed));
                                }
                            });


                        }
                    }

                    ui.separator();
                    ui.horizontal(|ui|
                    {
                        for i in 0..objects.len()
                        {
                            if objects[i].0.selected == true
                            {

                                if ui.button("🗑 Delete").clicked()
                                {
                                    for j in 0..objects.len()
                                    {
                                        if objects[j].0.selected == true
                                        {
                                            objects.remove(j);
                                            Objects::recalculate_id(&mut objects);
                                            break;
                                        }
                                    }
                                    break;
                                }
                            }

                        }

                    });

                    //println!("{:?}", response);
                });
        });

        // Draw things before egui

        egui_macroquad::draw();
        
        // Draw things after egui

        next_frame().await;
    }
}