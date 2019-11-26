use prototty_file_storage::IfDirectoryMissing;
pub use prototty_file_storage::{FileStorage, Storage};
use roguelike_prototty::{Controls, RngSeed};
pub use simon;
use simon::*;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::PathBuf;

const DEFAULT_SAVE_FILE: &str = "save";
const DEFAULT_NEXT_TO_EXE_SAVE_DIR: &str = "save";
const DEFAULT_NEXT_TO_EXE_CONTROLS_FILE: &str = "controls.json";

pub struct NativeCommon {
    pub rng_seed: RngSeed,
    pub save_file: String,
    pub file_storage: FileStorage,
    pub controls: Controls,
}

fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

fn read_controls_file(path: &PathBuf) -> Option<Controls> {
    let mut buf = Vec::new();
    let mut f = File::open(path).ok()?;
    f.read_to_end(&mut buf).ok()?;
    serde_json::from_slice(&buf).ok()
}

impl NativeCommon {
    pub fn arg() -> impl Arg<Item = Self> {
        args_map! {
            let {
                rng_seed = opt::<String>("r", "rng-seed", "rng seed", "TEXT")
                    .option_map(|s: String| RngSeed::U64(hash_string(s.as_str())))
                    .with_default(RngSeed::Entropy);
                save_file = opt("s", "save-file", "save file", "PATH")
                    .with_default(DEFAULT_SAVE_FILE.to_string());
                save_dir = opt("d", "save-dir", "save dir", "PATH")
                    .with_default(DEFAULT_NEXT_TO_EXE_SAVE_DIR.to_string());
                controls_file = opt::<String>("c", "controls-file", "controls file", "PATH");
                delete_save = flag("", "delete-save", "delete save game file");
            } in {{
                let controls_file = if let Some(controls_file) = controls_file {
                    controls_file.into()
                } else {
                    env::current_exe().unwrap().parent().unwrap().join(DEFAULT_NEXT_TO_EXE_CONTROLS_FILE)
                        .to_path_buf()
                };
                let controls = read_controls_file(&controls_file).unwrap_or_else(Controls::default);
                let mut file_storage = FileStorage::next_to_exe(
                    &save_dir,
                    IfDirectoryMissing::Create,
                ).expect("failed to open directory");
                if delete_save {
                    let result = file_storage.remove(&save_file);
                    if result.is_err() {
                        log::error!("couldn't find save file to delete");
                    }
                }
                Self {
                    rng_seed,
                    save_file,
                    file_storage,
                    controls,
                }
            }}
        }
    }
}
