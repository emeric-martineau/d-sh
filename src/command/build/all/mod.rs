use super::build_some_application;
use command::build::BuildOptions;
use command::list::get_all;
use command::{CommandError, CommandParameter};
use config::Config;
///
/// Module to build all image.
///
/// Release under MIT License.
///
use std::path::PathBuf;

pub fn build_all(
    cmd_param: &CommandParameter,
    options: &BuildOptions,
    config: &Config,
    tmp_dir: &PathBuf,
) -> Result<(), CommandError> {
    match get_all(cmd_param.io_helper, config) {
        Ok(app_list) => build_some_application(cmd_param, &tmp_dir, &options, config, &app_list),
        Err(err) => Err(err),
    }
}
