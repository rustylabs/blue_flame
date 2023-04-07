

/* Notes
For ast-builder check out https://github.com/gluesql/gluesql/tree/main/test-suite/src/ast_builder
*/

use gluesql::prelude::*;
//use gluesql::sled_storage::sled::IVec;


//use crate::object_settings::*;

use crate::{Objects, ObjectSettings, Scenes};

// Manages all projects and points to scenes
pub mod projects
{
    // Directory related libraries
    use std::path::PathBuf;
    use dirs;

    const VERSION: f32 = 0.1;
    const FILE_NAME: &'static str = "projects";


    pub fn save(projects: &[crate::Projects])
    {
        // ~/.config.blue_flame
        let mut config_dir: PathBuf =  match dirs::home_dir()
        {
            Some(v)         => v,
            None                     => {println!("Unable to obtain home dir"); return;},
        };
        config_dir.push(".config");
        config_dir.push("blue_flame");

        println!("config_dir: {:?}", config_dir);
        match std::fs::create_dir(&config_dir)
        {
            Ok(_)       => println!("Config dir created succesfully in {}", config_dir.display()),
            Err(e)      => println!("Unable to create config dir due to {e}"),
        }

        // This is where we actually save the file

        let data = postcard::to_stdvec(&(VERSION, projects)).unwrap();

        match std::fs::write(format!("{}/{FILE_NAME}", config_dir.display()), &data)
        {
            Ok(_)               => println!("Projects file saved!"),
            Err(e)       => println!("Save error: {e}"),
        }
    }


    /*
    use super::*;
    pub struct Sql
    {
        glue            : Glue<SledStorage>,
        table_names     : [&'static str; 1],
    }
    impl Sql
    {
        pub fn init() -> Self
        {
            let storage         = SledStorage::new("projects").unwrap();
            Self
            {
                glue            : Glue::new(storage),
                //table_names     : ["ObjectSettings", "Position"],
                table_names     : ["ProjectInfo"],
               //table_names     : ["ObjectSettings", "Position", "Scale"],
            }
        }
        pub fn load(&mut self, projects: &mut Vec<crate::Projects>)
        {
            let mut sqls: Vec<String> = vec![];
            for table_name in self.table_names.iter()
            {
                sqls.push(format!("SELECT * FROM {table_name}"));       //table_names     : ["Object", "ObjectType", "Position", "Scale", "Texture"]
                //println!("table_name: {table_name}");
            }
    
    
            // rows[Select { labels: ["ObjectType", "Texture"], rows: [[I64(1), I64(1)]] }, Select { labels: ["x", "y", "z"], rows: [[I64(0), I64(61), I64(0)]] }]
            let mut outputs: Vec<Payload> = vec![];
            for sql in sqls
            {
                match self.glue.execute(sql)
                {
                    // (Objects::init(0), ObjectSettings::init())
                    Ok(o) => 
                    {
                        outputs.push(o.into_iter().next().unwrap());
                    }
                    Err(_) =>
                    {
                        println!("Table does not exist");
                        return;
                    }
                }
            }
            //println!("rows{:?}", outputs);
            
    
            //println!("{:?}", rows); // [[I64(1), I64(1)]] 2nd time: [[I64(0), I64(61), I64(0)]]
            /*
            Converts this: // rows[Select { labels: ["object_type", "texture_mode"], rows: [[I64(1), I64(1)]] }, Select { labels: ["x", "y", "z"], rows: [[I64(0), I64(61), I64(0)]] }]
            To this: [[I64(1), I64(1)]] 2nd time: [[I64(0), I64(61), I64(0)]]
            */
            for (i, output) in outputs.iter().enumerate()
            {
                let rows = match &output
                {
                    Payload::Select{labels: _, rows} => rows,
                    _ => panic!(),
                };
                
                // ProjectInfo
                if i == 0
                {
                    // Rows
                    for (j, row) in rows.iter().enumerate()
                    {
                        projects.push(crate::Projects::init());
                        for (pos, element) in row.iter().enumerate()
                        {
                            match element
                            {
                                Value::Str(v) =>
                                {
                                    if pos == 0
                                    {
                                        projects[j].name = v.clone();
                                    }
                                    else if pos == 1
                                    {
                                        projects[j].dir = v.clone();
                                    }
                                    
                                }
                                Value::Bool(v) => projects[j].status = *v,
        
                                _ => panic!(),
                            };
                        }
                    }
                }
            }
        }
        pub fn save(&mut self, projects: &[crate::Projects])
        {
            let mut sqls: Vec<String> = vec![];
            for table_name in self.table_names.iter()
            {
                sqls.push(format!("DROP TABLE IF EXISTS {table_name};"));

                if table_name == &"ProjectInfo"
                {
                    sqls.push(format!("CREATE TABLE {table_name} (name TEXT, dir TEXT, game_type , status BOOLEAN);"));
    
                    for project in projects.iter()
                    {
                        sqls.push(format!("INSERT INTO {table_name} VALUES ('{}', '{}', {})",
                            project.name,
                            project.dir,
                            project.status,

                        ));
                    }
                }
            }

            // Executes sql commands
            for sql in sqls
            {
                let _output = self.glue.execute(sql).unwrap();
                println!("{:?}", _output);
            }
        }
    }
    */
}

// Scene manager
pub mod scenes
{
    use std::io::Read;

    const VERSION: f32 = 0.1;
    //const FILE_NAME: &'static str = "project_save";

    pub fn save(scenes: &[(crate::Scenes, crate::SceneSettings)], file_name: &str)
    {
        let data = postcard::to_stdvec(&(VERSION, scenes)).unwrap();

        match std::fs::write(file_name, &data)
        {
            Ok(_)               => println!("File saved!"),
            Err(e)       => println!("Save error: {e}"),
        }
    }
    pub fn load(objects: &mut Vec<(crate::Scenes, crate::SceneSettings)>, file_name: &str)
    {
        let mut file = match std::fs::File::open(file_name)
        {
            Ok(d)               => {println!("Objects: {file_name} loaded!"); d},
            Err(e)             => {println!("Load error on {file_name} {e}"); return;}
        };

        let mut data = Vec::new();
        match file.read_to_end(&mut data)
        {
            Ok(_)               => {},
            Err(e)       => println!("read_to_end error {e}"),
        }

        //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
        let value: (f32, Vec<(crate::Scenes, crate::SceneSettings)>) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                                     => {println!("Error on load: {e}"); return;},
        };

        let version = value.0;
        *objects = value.1;

        println!("db version Objects {file_name}: {}", version);
    }


    /*
    use crate::SceneSettings;

    use super::*;

    pub struct Db
    {
        
    }


    pub struct Sql
    {
        glue            : Glue<SledStorage>,
        table_names     : [&'static str; 2],
    }
    impl Sql
    {
        pub fn init() -> Self
        {
            let storage         = SledStorage::new("scenes").unwrap();
            Self
            {
                glue            : Glue::new(storage),
                table_names     : ["Scenes", "SceneSettings"],
            }
        }
        pub fn load(&mut self, scenes: &mut Vec<(Scenes, SceneSettings)>)
        {
            let mut sqls: Vec<String> = vec![];
            for table_name in self.table_names.iter()
            {
                sqls.push(format!("SELECT * FROM {table_name}"));       //table_names     : ["Object", "ObjectType", "Position", "Scale", "Texture"]
                //println!("table_name: {table_name}");
            }

            // rows[Select { labels: ["ObjectType", "Texture"], rows: [[I64(1), I64(1)]] }, Select { labels: ["x", "y", "z"], rows: [[I64(0), I64(61), I64(0)]] }]
            let mut outputs: Vec<Payload> = vec![];
            for sql in sqls
            {
                match self.glue.execute(sql)
                {
                    Ok(o) => 
                    {
                        outputs.push(o.into_iter().next().unwrap());
                    }
                    // First time scene is being created
                    Err(_) =>
                    {
                        println!("Table scenes does not exist");
                        scenes.push((Scenes::init(0), SceneSettings::default()));
                        return;
                    }
                }
            }

            /*
            Get each individual tables
            Converts this: // rows[Select { labels: ["object_type", "texture_mode"], rows: [[I64(1), I64(1)]] }, Select { labels: ["x", "y", "z"], rows: [[I64(0), I64(61), I64(0)]] }]
            To this: [[I64(1), I64(1)]] 2nd time: [[I64(0), I64(61), I64(0)]]
            */
            for (i, output) in outputs.iter().enumerate()
            {
                let rows = match &output
                {
                    Payload::Select{labels: _, rows} => rows,
                    _ => panic!(),
                };


                // Scenes [[I64(0), I64(61), I64(0)]]
                let mut id: u16 = 0;
                let mut label: String = String::new();
                let mut dir_save: String = String::new();
                let mut selected: bool = false;

                // SceneSettings
                let mut background_color: u32 = 0;
                let mut high_power_mode: bool = true;

                // Scenes
                if i == 0
                {
                    for row in rows.iter()
                    {
                        for (pos, element) in row.iter().enumerate()
                        {
                            match element
                            {
                                Value::I64(v) =>
                                {
                                    id = *v as u16;
                                }
                                Value::Str(v) =>
                                {
                                    if pos == 1
                                    {
                                        label = v.clone();
                                    }
                                    else if pos == 2
                                    {
                                        dir_save = v.clone();
                                    }
                                }
                                Value::Bool(v) =>
                                {
                                    selected = *v;
                                }
                                _ => panic!(),
                            }
                        }
                        /*
                        scenes.push(Scenes
                        {
                            id,
                            label: label.clone(),
                            selected,
                        });
                        */
                    }
                }

                // SceneSettings
                else if i == 1
                {
                    for row in rows.iter()
                    {
                        for element in row.iter()
                        {
                            match element
                            {
                                // background_color
                                Value::I64(v) =>
                                {
                                    background_color = *v as u32;
                                }
                                // high_power_mode
                                Value::Bool(v) =>
                                {
                                    high_power_mode = *v;
                                }
                                _ => panic!(),
                            }
                        }
                        /*
                        scenes.push(Scenes
                        {
                            id,
                            label: label.clone(),
                            selected,
                        });
                        */
                    }
                }

                scenes.push((Scenes{id, label, dir_save, selected}, SceneSettings{background_color, high_power_mode}));


            }
        }
        pub fn save(&mut self, scenes: &[(Scenes, SceneSettings)])
        {
            let mut sqls: Vec<String> = vec![];

            for table_name in self.table_names.iter()
            {
                sqls.push(format!("DROP TABLE IF EXISTS {table_name};"));

                if table_name == &"Scenes"
                {
                    sqls.push(format!("CREATE TABLE {table_name} (id INTEGER, label TEXT, dir_save TEXT, selected BOOLEAN);"));

                    for scene in scenes.iter()
                    {
                        sqls.push(format!("INSERT INTO {table_name} VALUES ({}, '{}', '{}', {})",
                            scene.0.id,
                            scene.0.label,
                            scene.0.dir_save,
                            scene.0.selected,
                        ));
                    }
                }

                if table_name == &"SceneSettings"
                {
                    sqls.push(format!("CREATE TABLE {table_name} (background_color INTEGER, high_power_mode BOOLEAN);"));

                    for scene in scenes.iter()
                    {
                        sqls.push(format!("INSERT INTO {table_name} VALUES ({}, {})",
                            scene.1.background_color,
                            scene.1.high_power_mode,
                        ));
                    }
                }
            }

            // Executes sql commands
            for sql in sqls
            {
                let _output = self.glue.execute(sql).unwrap();
                //println!("{:?}", _output);
            }
        }
    }
    */
}

// These could be levels, however you want to interpret it as
pub mod objects
{
    //use super::*;
    use std::io::Read;

    const VERSION: f32 = 0.1;
    //const FILE_NAME: &'static str = "project_save";

    pub fn save(objects: &[(crate::Objects, crate::ObjectSettings)], file_name: &str)
    {
        let data = postcard::to_stdvec(&(VERSION, objects)).unwrap();

        match std::fs::write(file_name, &data)
        {
            Ok(_)               => println!("File saved!"),
            Err(e)       => println!("Save error: {e}"),
        }
    }
    pub fn load(objects: &mut Vec<(crate::Objects, crate::ObjectSettings)>, file_name: &str)
    {
        let mut file = match std::fs::File::open(file_name)
        {
            Ok(d)               => {println!("Objects: {file_name} loaded!"); d},
            Err(e)             => {println!("Load error on {file_name} {e}"); return;}
        };

        let mut data = Vec::new();
        match file.read_to_end(&mut data)
        {
            Ok(_)               => {},
            Err(e)       => println!("read_to_end error {e}"),
        }

        //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
        let value: (f32, Vec<(crate::Objects, crate::ObjectSettings)>) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                                     => {println!("Error on load: {e}"); return;},
        };

        let version = value.0;
        *objects = value.1;

        println!("db version Objects {file_name}: {}", version);
    }

    /*
    pub struct Sql
    {
        glue            : Glue<SledStorage>,
        table_names     : [&'static str; 7],
    }
    impl Sql
    {
        pub fn init() -> Self
        {
            let storage         = SledStorage::new("objects").unwrap();
            Self
            {
                glue            : Glue::new(storage),
                //table_names     : ["ObjectSettings", "Position"],
                table_names     : ["Object" /*0*/, "ObjectType" /*1*/, "Position" /*2*/, "Size" /*3*/, "Rotation" /*4*/, "Texture" /*5*/, "Color" /*6*/],
               //table_names     : ["ObjectSettings", "Position", "Scale"],
            }
        }
        pub fn load(&mut self, objects: &mut Vec<(Objects, ObjectSettings)>)
        {
    
            let mut sqls: Vec<String> = vec![];
            for table_name in self.table_names.iter()
            {
                sqls.push(format!("SELECT * FROM {table_name}"));       //table_names     : ["Object", "ObjectType", "Position", "Scale", "Texture"]
                //println!("table_name: {table_name}");
            }
    
    
            // rows[Select { labels: ["ObjectType", "Texture"], rows: [[I64(1), I64(1)]] }, Select { labels: ["x", "y", "z"], rows: [[I64(0), I64(61), I64(0)]] }]
            let mut outputs: Vec<Payload> = vec![];
            for sql in sqls
            {
                match self.glue.execute(sql)
                {
                    // (Objects::init(0), ObjectSettings::init())
                    Ok(o) => 
                    {
                        outputs.push(o.into_iter().next().unwrap());
                    }
                    Err(_) =>
                    {
                        println!("Table does not exist");
                        objects.push((Objects::init(0), ObjectSettings::init()));
                        return;
                    }
                }
            }
            //println!("rows{:?}", outputs);
            
    
            //println!("{:?}", rows); // [[I64(1), I64(1)]] 2nd time: [[I64(0), I64(61), I64(0)]]
            /*
            Converts this: // rows[Select { labels: ["object_type", "texture_mode"], rows: [[I64(1), I64(1)]] }, Select { labels: ["x", "y", "z"], rows: [[I64(0), I64(61), I64(0)]] }]
            To this: [[I64(1), I64(1)]] 2nd time: [[I64(0), I64(61), I64(0)]]
            */
            for (i, output) in outputs.iter().enumerate()
            {
                let rows = match &output
                {
                    Payload::Select{labels: _, rows} => rows,
                    _ => panic!(),
                };
                
                // Object
                if i == 0
                {
                    for (j, row) in rows.iter().enumerate()
                    {
                        objects.push((Objects::init(j as u16), ObjectSettings::init()));
                        for (pos, element) in row.iter().enumerate()
                        {
                            match element
                            {
                                Value::I64(v) =>
                                {
                                    objects[j].0.id = *v as u16;
                                }
                                Value::Bool(v) =>
                                {
                                    if pos == 1
                                    {
                                        objects[j].0.visible = *v;
                                    }
                                    else if pos == 2
                                    {
                                        objects[j].0.selected = *v;
                                    }
                                }
                                Value::Str(v) =>
                                {
                                    objects[j].0.label.0 = v.clone();
                                }
        
                                _ => panic!(),
                            };
                        }
                    }
                }
    
                // ObjectType rows: [[I64(1), I64(1)]]
                
                if i == 1
                {
                    for (j, row) in rows.iter().enumerate()
                    {
                        for element in row.iter()
                        {
                            match element
                            {
                                Value::I64(v) =>
                                {
                                    radio_options::change_choice(&mut objects[j].1.object_type, *v as u8);
                                }
                                _ => panic!(),
                            }
    
                        }
                    }
                }
                // Position [[I64(0), I64(61), I64(0)]]
                else if i == 2
                {
                    for (j, row) in rows.iter().enumerate()
                    {
                        for (pos, element) in row.iter().enumerate()
                        {
                            match element
                            {
                                Value::F64(v) =>
                                {
                                    objects[j].1.position[pos].value = *v as f32;
                                }
                                _ => panic!(),
                            }
                        }
                    }
                }
                // Size [[I64(0), I64(61), I64(0)]]
                else if i == 3
                {
                    for (j, row) in rows.iter().enumerate()
                    {
                        for (pos, element) in row.iter().enumerate()
                        {
                            match element
                            {
                                Value::F64(v) =>
                                {
                                    objects[j].1.size[pos].value = *v as f32;
                                }
                                _ => panic!(),
                            }
                        }
                    }
                }
                // Rotation [[I64(0), I64(61), I64(0)]]
                else if i == 4
                {
                    for (j, row) in rows.iter().enumerate()
                    {
                        for (pos, element) in row.iter().enumerate()
                        {
                            match element
                            {
                                Value::F64(v) =>
                                {
                                    objects[j].1.rotation[pos].value = *v as f32;
                                }
                                _ => panic!(),
                            }
                        }
                    }
                }
                // Texture [[Str("name"), Str("location")]]
                else if i == 5
                {
                    for (j, row) in rows.iter().enumerate()
                    {
                        for element in row.iter()
                        {
                            match element
                            {
                                Value::Str(v) =>
                                {
                                    objects[j].1.texture.data = v.clone();
                                }
                                Value::I64(v) =>
                                {
                                    radio_options::change_choice(&mut objects[j].1.texture.mode, *v as u8);
                                }
                                _ => panic!(),
                            }
                        }
                    }
                }
                // Color
                else if i == 6
                {
                    for (j, row) in rows.iter().enumerate()
                    {
                        for (color_index, element) in row.iter().enumerate()
                        {
                            match element
                            {
                                Value::F64(v) =>
                                {
                                    objects[j].1.color[color_index] = *v as f32;
                                }
                                _ => panic!(),
                            }
                        }
                    }
                }
    
                // Texture [[Str("name"), Str("location")]]
                /*
                else if i == 3
                {
                    for j in 0..rows[0].len()
                    {
                        let mut value = String::new();
                        let mut index: u8 = 0;
    
    
                        match &rows[0][j]
                        {
                            Value::Str(v)       => value = format!("{}", v),
                            Value::I64(v)       => index = *v as u8,
                            _                   => panic!(),
                        };
    
                        // date of texture
                        if j == 0
                        {
                            object_settings.texture.data = value.clone();
                        }
                        else if j == 1
                        {
                            radio_options::change_choice(&mut object_settings.texture.mode, index);
                        }
    
                    }
    
                    //println!("{:?}", rows);
                }
                */
    
    
            }
            
        }
        
        pub fn save(&mut self, objects: &[(crate::Objects, crate::ObjectSettings)])
        {
            let mut sqls: Vec<String> = vec![];

            for table_name in self.table_names.iter()
            {
                sqls.push(format!("DROP TABLE IF EXISTS {table_name};"));
                
                if table_name == &"Object"
                {
                    sqls.push(format!("CREATE TABLE {table_name} (id INTEGER, visible BOOLEAN, selected BOOLEAN, label TEXT);"));
    
                    for object in objects.iter()
                    {
                        sqls.push(format!("INSERT INTO {table_name} VALUES ({}, {}, {}, '{}')",
                            object.0.id,
                            object.0.visible,
                            object.0.selected,
                            object.0.label.0,
                        ));
                    }
    
                }
    
                
                // ObjectType
                else if table_name == &"ObjectType"
                {
                    sqls.push(format!("CREATE TABLE {table_name} (object_type INTEGER);"));
    
                    for object in objects.iter()
                    {
                        sqls.push(format!("INSERT INTO {table_name} VALUES ({})",
                            radio_options::enabled_index(&object.1.object_type),
                        ));
                    }
                }
                else if table_name == &"Position"
                {
                    sqls.push(format!("CREATE TABLE {table_name} (x FLOAT, y FLOAT, z FLOAT);"));
    
                    for object in objects.iter()
                    {
                        sqls.push(format!("INSERT INTO {table_name} VALUES ({}, {}, {})",
                            object.1.position[0].value,
                            object.1.position[1].value,
                            object.1.position[2].value,
                        ));
                    }
                }
                else if table_name == &"Size"
                {
                    sqls.push(format!("CREATE TABLE {table_name} (x FLOAT, y FLOAT, z FLOAT);"));
    
                    for object in objects.iter()
                    {
                        sqls.push(format!("INSERT INTO {table_name} VALUES ({}, {}, {})",
                            object.1.size[0].value,
                            object.1.size[1].value,
                            object.1.size[2].value,
                        ));
                    }
                }
                else if table_name == &"Rotation"
                {
                    sqls.push(format!("CREATE TABLE {table_name} (x FLOAT, y FLOAT, z FLOAT);"));
    
                    for object in objects.iter()
                    {
                        sqls.push(format!("INSERT INTO {table_name} VALUES ({}, {}, {})",
                            object.1.rotation[0].value,
                            object.1.rotation[1].value,
                            object.1.rotation[2].value,
                        ));
                    }
                }
                else if table_name == &"Texture"
                {
                    sqls.push(format!("CREATE TABLE {table_name} (location TEXT, texture_mode INTEGER);"));
    
                    for object in objects.iter()
                    {
                        sqls.push(format!("INSERT INTO {table_name} VALUES ('{}', {})",
                            object.1.texture.data,
                            radio_options::enabled_index(&object.1.texture.mode),
                        ));
                    }
    
                }
                else if table_name == &"Color"
                {
                    sqls.push(format!("CREATE TABLE {table_name} (r FLOAT, g FLOAT, b FLOAT, a FLOAT);"));
    
                    for object in objects.iter()
                    {
                        sqls.push(format!("INSERT INTO {table_name} VALUES ({}, {}, {}, {})",
                            object.1.color[0],
                            object.1.color[1],
                            object.1.color[2],
                            object.1.color[3],
                        ));
                    }
                }
                
                
            }
    
            // Executes sql commands
            for sql in sqls
            {
                let _output = self.glue.execute(sql).unwrap();
                //println!("{:?}", _output);
            }
        }
    }
    */
}


// Specifically creates a json file in a specific format for the game engine
pub mod gameengine
{
    
}