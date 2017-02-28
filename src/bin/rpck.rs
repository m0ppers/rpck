extern crate rpck;

use std::fs::{self, OpenOptions, File};
use std::path::Path;
use std::error::Error;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        panic!("Must provide exactly 2 parameters. <infile> and <outfile>");
    }
    let rfilename = args[1].as_str();
    let wfilename = args[2].as_str();
    // Create a path to the desired file
    let rpath = Path::new(rfilename);
    let wpath = Path::new(wfilename);

    // Open the path in read-only mode, returns `io::Result<File>`
    let rfile = match File::open(&rpath) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", rfilename,
                                                   why.description()),
        Ok(file) => file,
    };

    let metadata = match fs::metadata(rfilename) {
        Err(why) => panic!("couldn't read metadata {}: {}", rfilename,
                                            why.description()),
        Ok(metadata) => metadata,
    };

    let wfile = match OpenOptions::new()
                             .create(true)
                             .truncate(true)
                             .write(true)
                             .open(&wpath) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", wfilename,
                                                   why.description()),
        Ok(file) => file,
    };

    match rpck::archive(rfile, metadata.len(), wfile) {
        Err(why) => panic!("couldn't write: {}", why.description()),
        Ok(_) => ()
    }
}
