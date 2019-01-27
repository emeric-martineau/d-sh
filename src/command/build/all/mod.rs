///
/// Module to build all image.
///
/// Release under MIT License.
///
use std::path::PathBuf;
use command::CommandError;
use command::build::BuildOptions;
use command::list::get_all;
use io::InputOutputHelper;
use docker::ContainerHelper;
use download::DownloadHelper;
use config::Config;
use super::build_some_application;

pub fn build_all(io_helper: &InputOutputHelper, dck_helper: &ContainerHelper,
    options: &BuildOptions, config: &Config, dl_helper: &DownloadHelper,
    tmp_dir: &PathBuf) -> Result<(), CommandError> {

    match get_all(io_helper, config) {
        Ok(app_list) => build_some_application(io_helper,
            dck_helper, &tmp_dir, &options, config, &app_list, dl_helper),
        Err(err) => Err(err)
    }
}
