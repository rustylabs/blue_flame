use blue_engine::{gui::{self, Condition}, header::{Engine, WindowDescriptor}, primitive_shapes::{triangle, square}};

use std::process::exit;


pub mod object_settings;
pub mod sql;
mod practice;

struct EditorSettings
{
    width               : f32,
    height              : f32,
    range               : f32,
    slider_speed        : f32,
}
impl EditorSettings
{
    fn init() -> Self
    {
        Self
        {
            width               : 250f32,
            height              : 900f32,
            range               : 900_000_000f32,
            slider_speed        : 1f32,
        }
    }
}

// ObjectsAddtion - addtional information
pub struct Objects
{
    id          : u16, // Foreign key
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
            label       : (format!("Object {id}"), issues::Issues::init())
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
            if objects.len() == 1
            {
                objects[0].0.label.1.error = false;
                return;
            }
            for i in 0..objects.len()
            {
                for j in 0..objects.len()
                {
                    if i != j
                    {
                        if objects[i].0.label.1.error != true && objects[i].0.label.0 == objects[j].0.label.0
                        {
                            objects[i].0.label.1.error = true;
                            break;
                        }
                        else
                        {
                            objects[i].0.label.1.error = false;    
                        }
                    }
                }
            }
        }
    }
}

struct Debug
{
    practice            : bool,
    resolution          : bool,
}

fn main()
{
    let editor_settings = EditorSettings::init();

    let debug = Debug
    {
        practice        : false,
        resolution      : true,
    };

    if debug.practice == true
    {
        println!("\n--------------Practice Mode!!!!!--------------\n");
        practice::main();
        println!("\n--------------End of practice!!!!!--------------\n");
        exit(0);
    }
// Test


    let mut engine = Engine::new(
        WindowDescriptor
        {
            width           : if debug.resolution == true {1280} else {1920},
            height          : if debug.resolution == true {720} else {1080},
            title           : "Blue Flame",
            decorations     : true,
            resizable       : true,
        }).unwrap();
    
    let square = square(blue_engine::header::ObjectSettings::default(), &mut engine)
    .unwrap()
    .object_index;

    let mut color = [1f32, 1f32, 1f32, 1f32];



    // sql Variables
    let mut sql = sql::Sql::init();

    // objects
    //let mut objects = vec![(Objects::init(0), ObjectSettings::init())];
    let mut objects = Vec::new();


    
    sql.load(&mut objects);
    engine.update_loop(move |_, _, gameengine_objects, _, _, ui|
    {
        //let style = ui.style();
    
        // Left panel
        gui::Window::new("Left Panel")
            .resizable(true)
            .size([editor_settings.width, editor_settings.height], Condition::FirstUseEver)
            .position([10f32, 20f32], Condition::FirstUseEver)
            .build(&ui, ||
            {
                ui.text("some text");

                // Button
                if ui.button("➕ Create Object") == true
                {
                    let len = objects.len() as u16;

                    objects.push((Objects::init(len), ObjectSettings::init()));
                    Objects::change_choice(&mut objects, len);

                    // Create new object and store it into the game engine
                    /*
                    let test_shape = blue_engine::primitive_shapes::square(blue_engine::header::ObjectSettings::default(), &mut engine)
                    .unwrap()
                    .object_index;
                    */
                }
                if ui.button("💾 Save") == true
                {
                    sql.save(&objects);
                }

                // Displays all objects button
                for i in 0..objects.len()
                {
                    if ui.button(format!("id: {}, {}", &objects[i].0.id, &objects[i].0.label.0)) == true
                    {
                        Objects::change_choice(&mut objects, i as u16);
                    }
                }

                
            });

    
        // Right Panel
        gui::Window::new("Object Settings")
            .resizable(true)
            .size([editor_settings.width, editor_settings.height], Condition::FirstUseEver)
            .position([900f32, 20f32], Condition::FirstUseEver)
            .build(&ui, ||
            {
                // name of object
                for object in objects.iter_mut()
                {
                    if object.0.selected == true
                    {
                        ui.text("Object name: ");
                        ui.input_text(format!("id: {}", object.0.id), &mut object.0.label.0).build();
                    }
                    
                }

                
                for object in objects.iter_mut()
                {
                    // Type of object (e.g. square, triangle, line)
                    if object.0.selected == true
                    {
                        ui.text("object type");

                        for i in 0..object.1.object_type.len()
                        {
                            if ui.radio_button_bool(object.1.object_type[i].name, object.1.object_type[i].status) == true
                            {
                                object_settings::radio_options::change_choice(&mut object.1.object_type, i as u8);
                            }
                        }

                        // Location of texture
                        ui.text("TextureMode");
                        ui.text("Location of Texture");
                        ui.input_text("texture", &mut object.1.texture.data).build();

                        // Radio buttons for texturemodes
                        for i in 0..object.1.texture.mode.len()
                        {
                            if ui.radio_button_bool(object.1.texture.mode[i].name, object.1.texture.mode[i].status) == true
                            {
                                object_settings::radio_options::change_choice(&mut object.1.texture.mode, i as u8);
                            }
                        }

                        // Position
                        ui.text("Position");
                        for position in object.1.position.iter_mut()
                        {
                            //ui.text(format!("{}:", position.axis as char));
                            gui::Drag::new(format!("Pos {}:", position.axis as char))
                                .speed(editor_settings.slider_speed)
                                .range(editor_settings.range * -1f32, editor_settings.range)
                                .build(&ui, &mut position.value);
                        }

                        // Scale
                        ui.text("Scale");
                        for scale in object.1.scale.iter_mut()
                        {
                            //ui.text(format!("{}:", position.axis as char));
                            gui::Drag::new(format!("Sca {}:", scale.axis as char))
                                .speed(editor_settings.slider_speed)
                                .range(editor_settings.range * -1f32, editor_settings.range)
                                .build(&ui, &mut scale.value);
                        }
                    }

                }

                gui::ColorEdit::new("Pick a Color", gui::EditableColor::Float4(&mut color))
                .inputs(false)
                .alpha(true)
                .alpha_bar(true)
                .build(&ui);

                // Delete button
                for i in 0..objects.len()
                {
                    if objects[i].0.selected == true
                    {
                        if ui.button("🗑 Delete") == true
                        {
                            objects.remove(i);
                            Objects::recalculate_id(&mut objects);
                            break;
                        }
                    }
                }

                gameengine_objects[square]
                .change_color(color[0], color[1], color[2], color[3])
                .unwrap();
            
            });
        
    }).unwrap();


    
}