#[allow(non_snake_case)]

use std::{path::PathBuf, time::Duration};

#[derive(Default)]
struct Configuration {

    outputPath: Option<PathBuf>,
    timeout: Duration,
    check: bool
}

fn main( ) {

    let mut outputPath= PathBuf::new( );
    outputPath.push("./generated/proto");

    //* partial initialization of struct with default values
    let configuration= Configuration {
        outputPath: Some(outputPath),

        ..Default::default( )
    };

}