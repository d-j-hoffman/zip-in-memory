use std::fs::File;
use std::io::Write;
use zip::result::{ZipResult, ZipError};
use std::result::IntoIter;
use zip::write::FileOptions;
use zip::ZipWriter;
use std::io::{Cursor};

// Do this for each file, start_file to initialize followed by write_all for adding byte values.
fn add_file(zip: &mut ZipWriter<std::io::Cursor<Vec<u8>>>, name: &str, content: &[u8]) {
    zip.start_file(name, FileOptions::default());
    zip.write_all(content);
}

pub fn get_file_handler(){
     // For this example we write to a buffer, but normally you should use a File
     let mut archive = Cursor::new(Vec::new());
     let mut zip = zip::ZipWriter::new(archive);

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

     let test = zip.finish().into_iter();    
     println!("{:?}", test)
}
