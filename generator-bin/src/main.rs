use std::error::Error;
use std::path::PathBuf;
use generator::{ModuleGenerator, Schema};

fn main() {
    let exit = match dew_it() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        }
    };

    std::process::exit(exit)
}

fn dew_it() -> Result<(), Box<dyn Error>> {
    let args = argwerk::args! {
        /// CLI tool for packet structure generator
        "generator [-h] --protocol JSON_PATH --out-dir OUT_DIR" {
            help: bool,
            #[required]
            protocol: PathBuf, // TODO just pass version?
            #[required]
            out_dir: PathBuf,
        }

        /// Protocol json file
        ["-p" | "--protocol", prot] => {
            protocol = Some(PathBuf::from(prot));
        }

        /// Output directory to generate module in
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
        return Ok(())
    }

    println!("reading {}", args.protocol.display());
    let json = std::fs::File::open(args.protocol)?;
    let schema = Schema::new(json)?;

    let mut out_dir: PathBuf = args.out_dir;
    out_dir.push("generated");

    println!("creating {}", out_dir.display());
    let mut generator = ModuleGenerator::new(out_dir)?;

    println!("generating packets");
    schema.per_state(|name, state| {
        let mut state_gen = generator.emit_state(name)?;
        state.per_packet(|packet| state_gen.emit_packet(&packet))
    })?;

    println!("done");
    Ok(())
}
