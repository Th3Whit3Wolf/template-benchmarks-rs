use cargo_toml::{Dependency, Manifest};
use crates_io_api::{Crate, Error, SyncClient};
use ructe::Ructe;

use std::{collections::BTreeMap,env,fs,path::{Path,PathBuf}, process::Command, thread, time::Duration};

const NOT_TEMPLATE_LIB: [&str; 8] = [
    "cargo_toml",
    "crates_io_api",
    "criterion",
    "sailfish-macros",
    "serde",
    "serde_derive",
    "serde_json",
    "serde_yaml",
];

const HEADER_MD: &str = "# Rust template engine benchmarks

This repo tries to assess Rust template engine performance. Following the
download ratings from [crates.io](https://crates.io/categories/template-engine), these nine projects are assessed:

";

const RESULTS_MD: &str = "
## Results

These results were produced by github actions.

As a [violin plot](https://en.wikipedia.org/wiki/Violin_plot) generated by [Criterion](https://japaric.github.io/criterion.rs/):

![Big table violin plot](big-table.svg)
![Teams violin plot](teams.svg)

Numbers, as output by Criterion:
";

fn main() {
    let in_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("templates_ructe");
    Ructe::from_env()
        .unwrap()
        .compile_templates(&in_dir)
        .expect("compile templates");

    #[cfg(feature = "make_md")]
    {
        let mut cargo_toml_path: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        cargo_toml_path.push("Cargo.toml");
        let cargo_toml_data = Manifest::from_path(cargo_toml_path);

        let msg = if let Ok(manifest) = cargo_toml_data {
            let mut m: String = HEADER_MD.to_string();
            if let Some(rustc_vers) = get_version_and_date() {
                if let Some(r_vers) = rustc_vers.0 {
                    m.push_str(&format!("- [write!(v{})](https://doc.rust-lang.org/std/macro.write.html): the std library `write!` macro\n", r_vers));
                } else {
                    m.push_str(&format!("- [write!](https://doc.rust-lang.org/std/macro.write.html): the std library `write!` macro\n"));
                } 
            } else {
                m.push_str(&format!("- [write!](https://doc.rust-lang.org/std/macro.write.html): the std library `write!` macro\n"));
            }

            let mut deps: BTreeMap<String, Dependency> = manifest.dependencies;
            for (name, kind) in manifest.build_dependencies {
                deps.insert(name, kind);
            }
            for (name, kind) in deps {
                if !NOT_TEMPLATE_LIB.contains(&name.as_str()) {
                    let vers = match kind {
                        Dependency::Simple(vers) => format!("(v{})", vers),
                        Dependency::Detailed(details) => {
                            if let Some(vers) = details.version {
                                format!("(v{})", vers)
                            } else {
                                "".to_string()
                            }
                        }
                    };

                    let crate_info = get_crate_info(&name);
                    match crate_info {
                        Ok(crate_info) => {
                            let md_desc = if let Some(desc) = crate_info.description {
                                format!(": {}", desc)
                            } else {
                                "".to_string()
                            };
                            let md_name = if let Some(homepage) = crate_info.homepage {
                                format!("- [{}{}]({})", crate_info.name, vers, homepage)
                            } else {
                                format!("- {}", crate_info.name)
                            };
                            m.push_str(&format!("{}{}", md_name, md_desc))
                        }
                        Err(e) => eprintln!("Oh, no: {}", e),
                    }
                    m.push('\n');
                }
            }
            m.push_str(RESULTS_MD);
            m
        } else {
            "Unable to read Cargo.toml\n".to_string()
        };

        let out_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("new_README.md");
        fs::write(&dest_path, msg).unwrap();
    }
}

#[cfg(feature = "make_md")]
fn get_crate_info(crate_name: &str) -> Result<Crate, Error> {
    // Instantiate the client.
    let client = SyncClient::new(
        "template-benchmark-rs (the.white.wolf.is.1337@gmail.com)",
        std::time::Duration::from_millis(2000),
    )?;
    // Retrieve crate data.
    let crate_info = client.get_crate(crate_name)?;
    thread::sleep(Duration::from_secs(3));

    Ok(crate_info.crate_data)
}

#[cfg(feature = "make_md")]
/// Parses (version, date) as available from rustc version string.
fn version_and_date_from_rustc_version(s: &str) -> (Option<String>, Option<String>) {
    let last_line = s.lines().last().unwrap_or(s);
    let mut components = last_line.trim().split(" ");
    let version = components.nth(1);
    let date = components.filter(|c| c.ends_with(')')).next()
        .map(|s| s.trim_end().trim_end_matches(")").trim_start().trim_start_matches('('));
    (version.map(|s| s.to_string()), date.map(|s| s.to_string()))
}

#[cfg(feature = "make_md")]
/// Returns (version, date) as available from `rustc --version`.
fn get_version_and_date() -> Option<(Option<String>, Option<String>)> {
    env::var("RUSTC").ok()
        .and_then(|rustc| Command::new(rustc).arg("--version").output().ok())
        .or_else(|| Command::new("rustc").arg("--version").output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| version_and_date_from_rustc_version(&s))
}