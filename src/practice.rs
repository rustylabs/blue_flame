use std::io::Read;

#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_imports)]

use postcard;
use serde;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Object
{
    x       : u16,
    y       : u16,
    z       : [u16; 3],
    square  : Square,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Object1
{
    square      : Square,
    a           : [u16; 3],
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Square
{
    width       : u32,
    height      : u32,
}

pub fn main()
{

    /* Solutions
    - https://users.rust-lang.org/t/using-postcard-and-serde-how-do-i-append-version-number/91131

     */
    //const VERSION: &'static str = "0.0.1";
    const VERSION: f32= 0.1;

    let rect: Vec<(Object, Object1)> = vec!
    [
        (
            Object{x: 10, y: 20, z: [10, 20, 30], square: Square{width: 10, height: 20}},
            Object1{square: Square{width: 10, height: 20}, a: [10, 20, 30]},
        ),
        (
            Object{x: 20, y: 40, z: [20, 40, 60], square: Square{width: 15, height: 25}},
            Object1{square: Square{width: 20, height: 30}, a: [20, 40, 60]},
        ),
    ];

    save(VERSION, &rect);

    fn save(version: f32, objects: &[(Object, Object1)])
    {
        /*
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        struct Data<T>
        {
            version     : f32,
            object      : T,
        }
        */

        //let data = match postcard::to_stdvec(&Data::<&Object>{version, object})
        let data = match postcard::to_stdvec(&(version, objects))
        {
            Ok(d)           => d,
            Err(e)            => {println!("Error on postcard storage: {e}"); return;}
        };

        match std::fs::write("save", &data)
        {
            Ok(_)               => println!("File saved!"),
            Err(e)       => println!("Save error {e}"),
        }

        // ---------------------------LOAD----------------------------

        let mut file = match std::fs::File::open("save")
        {
            Ok(d)               => {println!("File loaded!"); d},
            Err(e)             => {println!("Load error {e}"); return},
        };

        let mut data = Vec::new();
        match file.read_to_end(&mut data)
        {
            Ok(_)               => {},
            Err(e)       => println!("read_to_end error {e}"),
        }

        //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
        let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                             => {println!("Error on load: {e}"); return;},
        };

        let version = value.0;
        let objects = value.1;

        println!("version: {}", version);
        for (i, object) in objects.iter().enumerate()
        {
            println!("Line: {i}: Object: {:?}", object);
        }

        
        
    }

    /* postcard
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct MyData {
        some: String,
        data: u32,
    }

    const VERSION: &[u8] = b"editor-v0.1.0";

    fn main() {
        let file = save(&MyData { some: "hello".to_string(), data: 7 });
        let data = load(&file);
        dbg!(&data);
    }

    fn save(data: &MyData) -> Vec<u8> {
        let data = postcard::to_vec(data);

        // When saving, put our version at the beginning of the file.
        let mut output = VERSION.to_vec();
        // And add the data after it.
        output.append(data);

        output
    }

    fn load(bytes: &[u8]) -> MyData {
        // When loading, take the VERSION off of the beginning of the file.
        if let Some(data) = bytes.strip_prefix(VERSION) {
            // Now load the data without the prefix
            postcard::from_bytes(data)
        // If the VERSION doesn't exist at the start of the file, then error,
        // we have the wrong format.
        } else {
            panic!("Invalid file format.");
        }
    }

    let bytes = /* serialize with postcard */;

    std::fs::write("/path/to/my/file", &bytes);

    
    */

}