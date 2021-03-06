use gluesql::prelude::*;
use gluesql::sled_storage::sled::IVec;

use crate::object_settings::*;

use crate::{Objects, ObjectSettings};

pub struct Sql
{
    glue            : Glue<IVec, SledStorage>,
    table_names     : [&'static str; 5],
}
impl Sql
{
    pub fn init() -> Self
    {
        let storage         = SledStorage::new("project").unwrap();
        Self
        {
            glue            : Glue::new(storage),
            //table_names     : ["ObjectSettings", "Position"],
            table_names     : ["Object", "ObjectType", "Position", "Scale", "Texture"],
           //table_names     : ["ObjectSettings", "Position", "Scale"],
        }
    }
    pub fn load(&mut self, objects: &mut Vec<(Objects, ObjectSettings)>)
    {

        let mut sqls: Vec<String> = vec![];
        for table_name in self.table_names.iter()
        {
            sqls.push(format!("SELECT * FROM {table_name}"));
            //println!("table_name: {table_name}");
        }


        let mut outputs: Vec<Payload> = vec![];
        for sql in sqls
        {
            match self.glue.execute(sql)
            {
                // (Objects::init(0), ObjectSettings::init())
                Ok(o) => 
                {
                    outputs.push(o);
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
        // rows[Select { labels: ["object_type", "texture_mode"], rows: [[I64(1), I64(1)]] }, Select { labels: ["x", "y", "z"], rows: [[I64(0), I64(61), I64(0)]] }]

        for (i, output) in outputs.iter().enumerate()
        {
            let rows = match &output
            {
                Payload::Select{labels: _, rows} => rows,
                _ => panic!(),
            };
            //println!("{:?}", rows); // [[I64(1), I64(1)]] 2nd time: [[I64(0), I64(61), I64(0)]]

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
            // Scale [[I64(0), I64(61), I64(0)]]
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
                                objects[j].1.scale[pos].value = *v as f32;
                            }
                            _ => panic!(),
                        }
                    }
                }
            }
            // Texture [[Str("name"), Str("location")]]
            else if i == 4
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
            else if table_name == &"Scale"
            {
                sqls.push(format!("CREATE TABLE {table_name} (x FLOAT, y FLOAT, z FLOAT);"));

                for object in objects.iter()
                {
                    sqls.push(format!("INSERT INTO {table_name} VALUES ({}, {}, {})",
                        object.1.scale[0].value,
                        object.1.scale[1].value,
                        object.1.scale[2].value,
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
            
        }

        
        for sql in sqls
        {
            let _output = self.glue.execute(sql).unwrap();
            //println!("{:?}", _output);
        }
    }
}