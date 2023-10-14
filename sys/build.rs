use std::{env, fs, path::PathBuf};

struct Builder {
    /// The bindgen::Builder is the main entry point
    /// to bindgen, and lets you build up options for
    /// the resulting bindings.
    bindgen: ::bindgen::Builder,
}

impl Default for Builder {
    fn default() -> Self {
        parse_ament_prefix_path();

        // Tell cargo to invalidate the built crate whenever the wrapper changes
        println!(
            "cargo:rerun-if-changed={header}",
            header = Self::C_HEADER_FILE,
        );

        Self {
            bindgen: ::bindgen::Builder::default()
                // The input header we would like to generate
                // bindings for.
                .header(Self::C_HEADER_FILE)
                // Tell cargo to invalidate the built crate whenever any of the
                // included header files changed.
                .parse_callbacks(Box::new(::bindgen::CargoCallbacks)),
        }
    }
}

impl Builder {
    const C_HEADER_FILE: &str = "wrapper.h";
    const RUST_BINDGEN_OUT: &str = "bindings.rs";

    fn build(self) {
        // Finish the builder and generate the bindings.
        let bindings = self
            .bindgen
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join(Self::RUST_BINDGEN_OUT))
            .expect("Couldn't write bindings!");
    }
}

fn parse_ament_prefix_path() {
    const ENV_C_INCLUDE_PATH: &str = "C_INCLUDE_PATH";

    let include_path = env::var(ENV_C_INCLUDE_PATH).unwrap_or_default();
    let ament_include_path = env::var("AMENT_PREFIX_PATH")
        .expect("ROS 2 not inited! ( Please source your ROS setup )")
        .split(':')
        .map(|path| path.trim())
        .filter(|path| !path.is_empty())
        .map(|path| format!("{path}/include"))
        .filter_map(|path| fs::read_dir(path).ok())
        .flat_map(|paths| {
            paths
                .into_iter()
                .filter_map(|path| path.ok())
                .map(|path| path.path().display().to_string())
        })
        .collect::<Vec<_>>()
        .join(":");
    eprintln!("{ament_include_path}");
    eprintln!("{:?}", env::var("AMENT_PREFIX_PATH"));

    env::set_var(
        ENV_C_INCLUDE_PATH,
        format!("{include_path}:{ament_include_path}"),
    );
}

fn main() {
    Builder::default().build()
}
