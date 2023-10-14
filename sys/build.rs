use std::{env, fs, path::PathBuf};

use regex::Regex;

struct Builder {
    /// The bindgen::Builder is the main entry point
    /// to bindgen, and lets you build up options for
    /// the resulting bindings.
    bindgen: ::bindgen::Builder,
    parser: RmwIncludeParser,
}

impl Default for Builder {
    fn default() -> Self {
        // Tell cargo to invalidate the built crate whenever the wrapper changes
        println!(
            "cargo:rerun-if-changed={header}",
            header = Self::C_HEADER_FILE,
        );

        // note: ORDERED!
        let parser = RmwIncludeParser::default();

        Self {
            bindgen: ::bindgen::Builder::default()
                // The input header we would like to generate
                // bindings for.
                .header(Self::C_HEADER_FILE)
                // Tell cargo to invalidate the built crate whenever any of the
                // included header files changed.
                .parse_callbacks(Box::new(::bindgen::CargoCallbacks)),
            parser,
        }
    }
}

impl Builder {
    const C_HEADER_FILE: &str = "wrapper.h";
    const RUST_BINDGEN_OUT: &str = "bindings.rs";
    const RUST_RMW_OUT: &str = "rmw.rs";

    fn build(self) {
        // Finish the builder and generate the bindings.
        let bindings = self
            .bindgen
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let binding_path = out_path.join(Self::RUST_BINDGEN_OUT);
        bindings
            .write_to_file(&binding_path)
            .expect("Couldn't write bindings!");

        // Write RMW specification
        self.parser
            .build(binding_path, out_path.join(Self::RUST_RMW_OUT))
    }
}

struct RmwIncludeParser {
    header: String,
}

impl Default for RmwIncludeParser {
    fn default() -> Self {
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
            .collect::<Vec<_>>();

        env::set_var(
            ENV_C_INCLUDE_PATH,
            format!("{include_path}:{}", ament_include_path.join(":")),
        );

        Self {
            header: fs::read_to_string(format!(
                "{home}/rmw/rmw.h",
                home = ament_include_path
                    .into_iter()
                    .find(|path| path.ends_with("/rmw"))
                    .expect("ROS RMW package is missing."),
            ))
            .expect("Failed to read rmw/rmw.h"),
        }
    }
}

impl RmwIncludeParser {
    fn build(self, binding_path: PathBuf, out_path: PathBuf) {
        let re = Regex::new(r"^rmw_[_a-z]+\($").unwrap();
        let mut functions = self
            .header
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| re.is_match(line))
            .map(|function| &function[..function.len() - 1])
            .collect::<Vec<_>>();
        functions.sort();

        let binding = fs::read_to_string(binding_path)
            .expect("Failed to find binding")
            .replace('\n', " ");
        let template_functions = functions
            .iter()
            .map(|function| {
                let type_ = r"(\*[a-z]+ +)*[:_a-zA-Z0-9]+";
                let f_name = format!(r" *([_a-z0-9]+): *({type_})");
                let re = Regex::new(&format!(
                    r"pub +(fn *{function}\(({f_name},)*({f_name},?)?\ *\) *-> *({type_});)"
                ))
                .unwrap();

                eprintln!("{function}");
                let f_def = &re
                    .captures(&binding)
                    .expect("Cannot find function definition")[1];

                format!("    unsafe extern \"C\" {f_def}")
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        let template_trait = format!(
            r#"
pub unsafe trait RmwExtern {{
{template_functions}
}}
"#,
        );
        fs::write(out_path, template_trait).expect("Failed to write rmw interface")
    }
}

fn main() {
    Builder::default().build()
}
