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
}
pub fn main()
{

    /* Solutions
    - https://users.rust-lang.org/t/using-postcard-and-serde-how-do-i-append-version-number/91131

     */
    //const VERSION: &'static str = "0.0.1";
    const VERSION: f32= 0.1;

    let rect = Object{x: 10, y: 20, z: [10, 20, 30]};

    save(&rect, VERSION);

    fn save(object: &Object, version: f32)
    {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        struct Data<T>
        {
            version     : f32,
            object      : T,
        }

        let data = match postcard::to_stdvec(&Data::<&Object>{version, object})
        {
            Ok(d)           => d,
            Err(e)            => {println!("Error on save: {e}"); return;}
        };

        let value: Data<Object> = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)            => {println!("Error on load: {e}"); return;},
        };

        println!("version: {:?}", value);
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