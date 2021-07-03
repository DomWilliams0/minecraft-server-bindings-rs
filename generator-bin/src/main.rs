use generator::{ModuleGenerator, Schema};
use std::error::Error;
use std::path::{Path, PathBuf};

fn main() {
    let exit = match dew_it() {
        Ok(_) => 0,
        Err(err) => {
            println!("error: {}", err);
            let mut err = &*err;
            while let Some(cause) = err.source() {
                println!(" * error: {}", cause);
                err = cause;
            }
            1
        }
    };

    std::process::exit(exit)
}

// TODO get versions out of protocolVersions.json and store as constants

struct DeleteDirBomb<'a>(&'a Path);

fn dew_it() -> Result<(), Box<dyn Error>> {
    let args = argwerk::args! {
        /// CLI tool for packet structure generator
        "generator [-h] --protocol-dir DIR" {
            help: bool,
            #[required]
            protocol_dir: PathBuf,
            #[required]
            out_dir: PathBuf,
        }

        /// Protocol directory e.g. minecraft-data/data/pc/1.17
        ["-p" | "--protocol-dir", dir] => {
            protocol_dir = Some(PathBuf::from(dir));
        }

        /// Crate root dir to generate modules in
        ["-o" | "--out-dir", dir] => {
            out_dir = Some(PathBuf::from(dir));
        }

        /// Print this help
        ["-h" | "--help"] => {
            println!("{}", HELP);
            help = true;
        }
    }?;

    if args.help {
        return Ok(());
    }

    let protocol_dir: PathBuf = args.protocol_dir;
    let json_path = protocol_dir.join("protocol.json");
    if !json_path.is_file() {
        return Err(format!(
            "protocol.json not found within protocol dir '{}'",
            protocol_dir.display()
        )
        .into());
    }
    println!("found protocol.json at {}", json_path.display());

    let version = protocol_dir
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.replace('.', "-"))
        .ok_or("could not extract protocol_version")?;
    println!("version is {}", version);

    let module_dir = {
        let mut string = version.replace('-', "_");
        let mut path: PathBuf = args.out_dir;
        path.push("packets/src");

        if !path.is_dir() {
            return Err("could not find packets/src within out directory".into());
        }

        string.insert(0, 'v');
        path.push(string);
        path
    };
    println!("module path is {}", module_dir.display());

    println!("parsing {}", json_path.display());
    let json = std::fs::File::open(json_path)?;
    let schema = Schema::new(json)?;

    let bomb = DeleteDirBomb(&module_dir);
    let mut generator = ModuleGenerator::new(&module_dir)?;

    schema.per_state(|name, state| {
        let mut state_gen = generator.emit_state(name)?;
        state.per_packet(|packet| {
            state_gen.emit_packet(&packet)?;
            Ok(())
        })?;
        state_gen.finish()?;
        Ok(())
    })?;

    bomb.defuse();
    println!("done");
    Ok(())
}

impl<'a> DeleteDirBomb<'a> {
    fn defuse(self) {
        std::mem::forget(self);
    }
}

impl Drop for DeleteDirBomb<'_> {
    fn drop(&mut self) {
        eprintln!("deleting {} on failure", self.0.display());
        if let Err(err) = std::fs::remove_dir_all(self.0) {
            eprintln!("failed to delete directory: {}", err);
        }
    }
}
