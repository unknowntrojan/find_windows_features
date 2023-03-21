use clap::Parser;

const PATH: &'static str = "E:\\Projects\\windows";

#[derive(Parser)]
struct Args {
    #[arg(index = 1)]
    func: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let target_string = format!(r"fn {}[^A-Za-z]", args.func);
    let target_regex = regex::Regex::new(&target_string)?;

    assert!(target_regex.is_match("pub unsafe fn OpenProcess<P0>(dwdesiredaccess: PROCESS_ACCESS_RIGHTS, binherithandle: P0, dwprocessid: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>"));

    for entry in glob::glob(format!("{}/crates/libs/**/*.rs", PATH).as_str())? {
        if let Ok(path) = entry {
            let file = std::fs::read(&path)?;

            let Ok(file) = String::from_utf8(file) else {
				continue;
			};

            let file: Vec<&str> = file.split("\n").collect();

            let Some(idx) = file.iter().position(|line| {
				target_regex.is_match(line)
			}) else {
				continue;
			};

            for i in 0..=8 {
                if let Some(line) = file.get(idx - i) {
                    if line.contains("Required features:") {
                        let (_, line) = line.split_once("Required features:").unwrap();
                        let (line, _) = line.split_once("\"]").unwrap();

                        let mut features: Vec<&str> = line.split("\\\"").collect();

                        features.retain(|segment| segment.contains("Win32"));

                        for feature in features {
                            println!("{}", feature);
                        }

                        return Ok(());
                    }
                }
            }
        }
    }

    eprintln!("Could not find function");
    Ok(())
}
