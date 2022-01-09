use crate::utils::packages::get_packages;
use std::error::Error;

pub fn update_packages() -> Result<(), Box<dyn Error>> {
    let packages = get_packages();

    for package in packages.iter() {
        if package.exist() {
            println!("🟢 Update package: {}", package.name);
            package.update()?;
        } else {
            println!("🔴 {} is not exist", package.name);
        }
    }

    Ok(())
}
