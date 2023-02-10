#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_imports)]
pub fn main()
{

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