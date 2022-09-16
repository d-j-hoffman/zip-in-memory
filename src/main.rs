use std::fs::File;
use std::io::Write;
use zip::result::ZipResult;
use zip::write::FileOptions;
use zip::ZipWriter;

// Do this for each file, start_file to initialize followed by write_all for adding byte values.
fn add_file(zip: &mut ZipWriter<File>, name: &str, content: &[u8]) {
    zip.start_file(name, FileOptions::default());
    zip.write_all(content);
}

fn main() {
    fn doit() {
        // For this example we write to a buffer, but normally you should use a File
        let mut buf: &mut [u8] = &mut [0u8; 65536];
        println!("{:?}", std::env::current_dir());
        let mut file = match File::create("./src/testFiles/zippedFile.zip") {
            Err(e) => panic!("{}", e),
            Ok(file) => file,
        };
        let mut zip = zip::ZipWriter::new(file);

        add_file(
            &mut zip,
            "innerFile1.json",
            b"{testKey: 'testValue', testSecondKey: true}",
        );
        add_file(
            &mut zip,
            "innerFile2.json",
            b"{test2Key1: 'testValue', test2SecondKey: true}",
        );

        zip.finish();
    }

    println!("Result: {:?}", doit());
}
