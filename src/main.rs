extern crate arguably;
extern crate edit;

use arguably::ArgParser;
use std::path::Path;
use std::process::exit;


const HELP: &str = "
Usage: vimv [FLAGS] [ARGUMENTS]

  This utility lets you batch rename files using a text editor.

  The list of files to rename will be opened in the editor specified by the
  $EDITOR environment variable, one file per line. Edit the list, save, and
  exit. The files will be renamed to the edited filenames. Directories will
  be created as required.

Arguments:
  [files]               List of files to rename.

Flags:
  -f, --force           Force overwrite existing files.
  -h, --help            Print this help text.
  -v, --version         Print the application's version number.
";


fn main() {
    let mut parser = ArgParser::new()
        .helptext(HELP)
        .version(env!("CARGO_PKG_VERSION"))
        .flag("force f");

    if let Err(err) = parser.parse() {
        err.exit();
    }

    if parser.args.len() == 0 {
        eprintln!("Error: missing filename argument");
        exit(1);
    }

    let input_string = parser.args.join("\n");
    let output_string = match edit::edit(input_string) {
        Ok(edited) => edited,
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    };

    let input_filenames: Vec<&str> = parser.args.iter().map(|s| s.trim()).collect();
    let output_filenames: Vec<&str> = output_string.trim().lines().map(|s| s.trim()).collect();

    // Sanity check: make sure we have the same number of input and output files.
    if output_filenames.len() != input_filenames.len() {
        eprintln!("Error: number of input filenames does not match number of output filenames");
        exit(1);
    }

    // Sanity check: make sure all the input files exist.
    for input_filename in &input_filenames {
        if !Path::new(input_filename).exists() {
            eprintln!("Error: the input file '{}' does not exist", input_filename);
            exit(1);
        }
    }

    for (i, input_filename) in input_filenames.iter().enumerate() {
        let output_filename = &output_filenames[i];
        if input_filename == output_filename {
            continue;
        }
        let output_path = Path::new(output_filename);
        if output_path.exists() && !parser.found("force").unwrap() {
            eprintln!("Error: the output file '{}' already exists", output_filename);
            exit(1);
        }
        if let Some(parent_path) = output_path.parent() {
            if !parent_path.is_dir() {
                if let Err(err) = std::fs::create_dir_all(parent_path) {
                    eprintln!("Error: {}", err);
                    exit(1);
                }
            }
        }
        if let Err(err) = std::fs::rename(input_filename, output_filenames[i]) {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
}
