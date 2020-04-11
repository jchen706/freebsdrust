// extern crate bindgen;

// use std::env;
// use std::path::PathBuf;

// //place to dump the bind gen process
// const FILEPATH_CODE: &'static str = "src/os/bindings.rs";


// fn main() {
//     //env::set_var("LIBCLANG_PATH", "/usr/local/llvm90/lib");

//     // Tell cargo to tell rustc to link the system bzip2
//     // shared library.
//     println!("cargo:rustc-link-lib=bz2");

//     // Tell cargo to invalidate the built crate whenever the wrapper changes
//     println!("cargo:rerun-if-changed=wrapper.h");

//     let out_path = PathBuf::from(FILEPATH_CODE);
    

//     let output = std::fs::File::create(FILEPATH_CODE)
//         .unwrap_or_else(|e| panic!("Failed to create file {:?}: {}", FILEPATH_CODE, e));

//     // The bindgen::Builder is the main entry point
//     // to bindgen, and lets you build up options for
//     // the resulting bindings.
//     let bindings = bindgen::Builder::default()
//         // The input header we would like to generate
//         // bindings for.
//         .header("wrapper.h")
//         // Tell cargo to invalidate the built crate whenever any of the
//         // included header files changed.
//         .parse_callbacks(Box::new(bindgen::CargoCallbacks))
//         // Finish the builder and generate the bindings.
//         .generate()
//         // Unwrap the Result and panic on failure.
//         .expect("Unable to generate bindings");

//     // Write the bindings to the $OUT_DIR/bindings.rs file.
   

//     bindings
//         .write_to_file(out_path)
//         .expect("Couldn't write bindings!");

//     let build_path = env::current_dir().unwrap().as_path().to_owned();

// //     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
// //     bindings
// //         .write_to_file(out_path.join("bindings.rs"))
// //         .expect("Couldn't write bindings!");
//     env::set_current_dir(&Path::new(&kernel_path)).unwrap();
// }

use std::env;
use std::io::Write;
use std::path::Path;

extern crate bindgen;
extern crate shlex;

/// Path to the kernel bindings source file to generate
const FILEPATH_CODE:   &'static str = "src/os/kernel.rs";

/// Name of the file to store include directives for the C preprocessor in
const FILENAME_HEADER: &'static str = "kernel-include.h";

/// List of headers required by linux-std itself
const CLANG_HEADER_REQUIRED: [&'static str; 6] = [
    "stdint.h",
    "sys/types.h",
    "sys/module.h",
    "sys/systm.h",  /* uprintf */
    "sys/param.h" , /* defines used in kernel.h */
    "sys/malloc.h", 
];

/// List of parameters not ever to pass to the clang parser of rust-bindgen
const CLANG_ARGS_BLACKLIST: [&'static str; 10] = [
    "-mno-fp-ret-in-387",
    "-mpreferred-stack-boundary=3",
    "-mskip-rax-setup",
    "-mindirect-branch=thunk-extern",
    "-mindirect-branch-register",
    "-fno-var-tracking-assignments",
    "-mrecord-mcount",
    "-fconserve-stack",
    "-fmacro-prefix-map=./=",
    "-DCC_HAVE_ASM_GOTO",
];

fn main() {
    env::set_var("LIBCLANG_PATH", "/usr/local/llvm90/lib");

 // Read and parse required environment variables
	let mut clang_args: Vec<String> = match env::var("STD_CLANG_ARGS") {
		Ok(string) =>
			match shlex::split(string.as_str()) {
				Some(mut args) => {
					// Find positions of arguments to remove
					let mut remove_indices = Vec::with_capacity(CLANG_ARGS_BLACKLIST.len());
					for (index, clang_arg) in args.iter().enumerate() {
						if CLANG_ARGS_BLACKLIST.contains(&clang_arg.as_str()) {
							remove_indices.push(index);
						}
					}
					
					// Remove the found positions from the argument list
					for index in remove_indices.iter().rev() {
						args.swap_remove(*index);
					}
					
					args
				},
				None => {
					panic!("Malformed environment variable STD_CLANG_ARGS");
				}
			},
		Err(error) => {
			panic!("Missing environment variable STD_CLANG_ARGS: {:?}", error);
		}
	};
	let clang_files = match env::var("STD_CLANG_FILES") {
		Ok(string) =>
			match shlex::split(string.as_str()) {
				Some(args) => args,
				None => {
					panic!("Malformed environment variable STD_CLANG_FILES");
				}
			},
		Err(error) => {
			panic!("Missing environment variable STD_CLANG_FILES: {:?}", error);
		}
	};
	let kernel_path = match env::var("STD_KERNEL_PATH") {
		Ok(string) => string,
		Err(error) => {
			panic!("Missing environment variable STD_KERNEL_PATH: {:?}", error);
		}
	};
	let out_dir = match env::var("OUT_DIR") {
		Ok(string) => string,
		Err(error) => {
			panic!("Missing environment variable OUT_DIR: {:?}", error);
		}
	};
	
	let filepath_header = format!("{}/{}", out_dir, FILENAME_HEADER);

	// Prevent the kernel from declaring datatypes that are rust internal datatypes
	clang_args.push(String::from("-Dfalse=__false"));
	clang_args.push(String::from("-Dtrue=__true"));
	clang_args.push(String::from("-Du64=__u64"));
	
    // Push supplied header file paths (relative to the kernel directory)
	match std::fs::File::create(filepath_header.clone()) {
		Ok(mut file) => {
			// Generate include lines for all requested headers
			for clang_file in clang_files.iter() {
				writeln!(file, "#include <{}>", clang_file).unwrap();
			}
			
			// Generate include lines for headers required by linux-std itself
			for clang_file in CLANG_HEADER_REQUIRED.iter() {
				writeln!(file, "#include <{}>", clang_file).unwrap();
			}
		},
		Err(error) => {
			panic!("Failed to open file \"{}\": {}", filepath_header, error);
		}
	}
	
	// Tell clang to process the generated header file
	clang_args.push(filepath_header);

    // Open the output file before changing directory.
    let output = std::fs::File::create(FILEPATH_CODE)
        .unwrap_or_else(|e| panic!("Failed to create file {:?}: {}", FILEPATH_CODE, e));

    let build_path = env::current_dir().unwrap().as_path().to_owned();
    env::set_current_dir(&Path::new(&kernel_path)).unwrap();

    bindgen::builder()
        .emit_builtins()
        .clang_args(clang_args)
        .derive_debug(false)
        .opaque_type("timex") // large types with bitfields are broken; see rust-bindgen#1325
        .rustfmt_bindings(true)
        .generate()
        .unwrap()
        .write(Box::new(output))
        .unwrap();

    // Don't re-run this thing, ever. It takes too long. Do a clean rebuild if the kernel changes.
    println!("cargo:rerun-if-changed=build.rs");

    env::set_current_dir(&build_path).unwrap();
}

