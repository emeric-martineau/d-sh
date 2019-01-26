///
/// Module to build all image.
///
/// Release under MIT License.
///
use std::path::{Path, PathBuf};
use command::CommandError;
use command::CommandExitCode;
use io::InputOutputHelper;
use docker::ContainerHelper;
use download::DownloadHelper;
use config::Config;
use command::build::BuildOptions;
use super::build_some_application;

pub fn build_all(io_helper: &InputOutputHelper, dck_helper: &ContainerHelper,
    options: &BuildOptions, config: &Config, dl_helper: &DownloadHelper,
    tmp_dir: &PathBuf) -> Result<(), CommandError> {
    let mut list_applications_file;

    match io_helper.dir_list_file(&config.applications_dir, "*.yml") {
        Ok(r) => list_applications_file = r,
        Err(err) => return Err(CommandError {
            msg: vec![format!("{}", err)],
            code: CommandExitCode::DockerBuildFail
        })
    };

    list_applications_file.sort();

    let mut app_list = Vec::new();

    // 2 - We have list of application
    for filename in list_applications_file  {
        let application_name = Path::new(&filename)
            .file_stem()
            .unwrap()   // get OsStr
            .to_str()
            .unwrap();

        app_list.push(String::from(application_name));
    };

    build_some_application(io_helper,
        dck_helper, &tmp_dir, &options, config, &app_list, dl_helper)
}
