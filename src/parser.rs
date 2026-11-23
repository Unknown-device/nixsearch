use crate::package::Package;

pub fn parse_packages(input: &str) -> Result<Vec<Package>, String> {
    let mut packages = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let package: Package = serde_json::from_str(line)
            .map_err(|e| format!("JSON parsing error: {e}"))?;

        packages.push(package);
    }

    Ok(packages)
}
