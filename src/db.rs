// Manages all projects and points to scenes
pub mod projects
{
    use std::io::Read;


    const VERSION: f32 = 0.1;
    const FILE_NAME: &'static str = "projects";


    pub fn save(projects: &[crate::Projects], file_paths: &crate::FilePaths)
    {

        // This is where we actually save the file

        let data = postcard::to_stdvec(&(VERSION, projects)).unwrap();

        match std::fs::write(format!("{}/{FILE_NAME}", file_paths.projects.display()), &data)
        {
            Ok(_)               => println!("Projects file saved!"),
            Err(e)       => println!("Save error: {e}"),
        }
    }

    pub fn load(projects: &mut Vec<crate::Projects>, file_paths: &crate::FilePaths)
    {
        let mut file = match std::fs::File::open(format!("{}/{FILE_NAME}", file_paths.projects.display(), ))
        {
            Ok(d)               => {println!("Projects: {FILE_NAME} loaded!"); d},
            Err(e)             => {println!("Load projects error on {FILE_NAME} {e}"); return;}
        };

        let mut data = Vec::new();
        match file.read_to_end(&mut data)
        {
            Ok(_)               => {},
            Err(e)       => println!("read_to_end error {e}"),
        }

        //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
        let value: (f32, Vec<crate::Projects>) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                                     => {println!("Error on load: {e}"); return;},
        };

        let version = value.0;
        *projects = value.1;

        println!("db version Projects {FILE_NAME}: {}", version);
    }
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
    const SAVE_FOLDER: &'static str = "blue_flame";
    //const FILE_NAME: &'static str = "project_save";

    pub fn save(objects: &[(crate::Objects, crate::ObjectSettings)], file_paths: &crate::FilePaths, file_name: &str)
    {
        let data = postcard::to_stdvec(&(VERSION, objects)).unwrap();
        

        match std::fs::write(format!("{}/{SAVE_FOLDER}/{file_name}", file_paths.scenes.display()), &data)
        {
            Ok(_)               => println!("File saved!"),
            Err(e)       => println!("Save error: {e}"),
        }
    }
    pub fn load(objects: &mut Vec<(crate::Objects, crate::ObjectSettings)>, file_paths: &crate::FilePaths, file_name: &str)
    {
        let mut file = match std::fs::File::open(format!("{}/{SAVE_FOLDER}/{file_name}", file_paths.scenes.display()))
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
}


// Specifically creates a json file in a specific format for the game engine
pub mod gameengine
{
    
}