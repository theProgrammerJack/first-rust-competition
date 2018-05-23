use serde_json;
use serde_json::Value;
use std::path::PathBuf;
use subprocess;

#[derive(Debug)]
pub struct FrcConfig {
    pub team_number: Option<u64>,
    pub rio_address_override: Option<String>,
    pub target_dir: PathBuf,
    pub executable: PathBuf,
}

pub fn get_config() -> Result<FrcConfig, String> {
    debug!("Reading from `cargo read-manifest`");
    let manifest: Value = serde_json::from_str(&subprocess::Exec::cmd("cargo")
        .arg("read-manifest")
        .capture()
        .map_err(|e| e.to_string())?
        .stdout_str())
        .map_err(|e| e.to_string())?;
    let err = "FRC Config not found in package manifest.
        Config should be in [package.metadata.frc] in Cargo.toml";

    let frc = manifest
        .get("metadata")
        .ok_or(err)?
        .get("frc")
        .ok_or(err)?
        .as_object()
        .ok_or(err)?;
    // executable
    let target_dir_json = frc
        .get("target-dir")
        .ok_or("target-dir not specified in FRC Config.")?;
    let target_dir_string = match target_dir_json {
        Value::String(x) => Ok(x),
        _ => Err("target-dir must be a string"),
    }?;
    let mut target_dir = PathBuf::new();
    target_dir.push(target_dir_string);

    let executable_name = match frc.get("executable-name") {
        Some(Value::String(x)) => Ok(x.clone()),
        _ => {
            warn!("Executable name not specified, using package name instead.");
            match manifest["name"] {
                Value::String(ref x) => Ok(x.clone()),
                _ => Err("Could not find package name"),
            }
        }
    }?;
    let mut executable = PathBuf::new();
    executable.push(executable_name);

    // rio address
    let team_number = match frc.get("team-number") {
        Some(Value::Number(x)) => x.as_u64(),
        _ => {
            warn!("No team number found, or it is not an unsigned integer.");
            None
        }
    };
    let rio_address_override = match frc.get("rio-address") {
        Some(Value::String(x)) => Some(x),
        _ => {
            info!("No rio address override found, or it is not a string.");
            None
        }
    }.cloned();
    if team_number == None && rio_address_override == None {
        error!("Neither a team number or rio address was specified.");
    }

    Ok(FrcConfig {
        team_number,
        rio_address_override,
        target_dir,
        executable,
    })
}
