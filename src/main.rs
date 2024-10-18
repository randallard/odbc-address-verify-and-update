mod config;
mod error;
mod fs;

pub use self::error::{Error, Result};
pub use config::config;

use crate::fs::list_files;

use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;


fn main() -> Result<()> {

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("odbc-address-verify-and-update started");

    debug!("got config CONFIG_VAR_ONE: {}", &config().CONFIG_VAR_ONE);
    debug!("got config CONFIG_VAR_TWO: {}", &config().CONFIG_VAR_TWO);
    
    let paths = vec!["./some_dir", "./some_dir/empty_dir", "./some_dir/another_dir", "./non_existant"];
    
    for path in paths {
        debug!("Listing files for path: {}", path);        
        match list_files(path) {
            Ok(files) => {
                debug!("{files:#?}");
            },
            Err(e) => {
                error!("Error processing path '{}': {:?}", path, e);
            }
        }
    }

    info!("odbc-address-verify-and-update complete");

    Ok(())
}