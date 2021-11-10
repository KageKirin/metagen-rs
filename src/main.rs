use std::env;
use std::io::Write;
use std::path::PathBuf;

use argh::FromArgs;
use twox_hash::xxh3::hash128_with_seed;
use twox_hash::xxh3::hash64;

fn default_seed() -> String {
    String::from(env::current_dir().unwrap().to_str().unwrap())
}

#[derive(FromArgs)]
/// Generate Unity .meta files
struct Args {
    /// seed string, e.g. package name
    #[argh(option, short = 's', default = "default_seed()")]
    seed: String,

    /// overwrite existing .meta files
    #[argh(switch, short = 'o')]
    overwrite: bool,

    /// files to generate .meta files for
    #[argh(positional)]
    files: Vec<PathBuf>,
}

fn main() {
    let args: Args = argh::from_env();
    println!("seed: {}", args.seed);

    let seed: u64 = hash64(args.seed.as_bytes());
    println!("seed hash: {:x}", seed);

    for f in args.files {
        if f.exists() && (f.extension() != None && f.extension().unwrap() != "meta") {
            let guid = hash128_with_seed(f.to_str().unwrap().as_bytes(), seed);
            println!("{} -> {:x}", f.to_str().unwrap(), guid);
            generate_meta_file(f, guid, args.overwrite);
        }
    }
}

fn generate_meta_file(filename: PathBuf, guid: u128, overwrite: bool) {
    let metafilename = PathBuf::from(format!("{}.meta", &filename.to_string_lossy()));

    if metafilename.exists() && !overwrite {
        println!("skipping {}", &metafilename.to_string_lossy());
        return;
    }

    println!("writing {}", &metafilename.to_string_lossy());
    let data = format!(
        "fileFormatVersion: 2
guid: {:x}
{}",
        guid,
        r#"MonoImporter:
  externalObjects: {}
  serializedVersion: 2
  defaultReferences: []
  executionOrder: 0
  icon: {instanceID: 0}
  userData: 
  assetBundleName: 
  assetBundleVariant:
"#
    );
    println!("{}", data);
    let mut file = std::fs::File::create(&metafilename).expect("create failed");
    file.write_all(data.as_bytes()).expect("write failed");
}
