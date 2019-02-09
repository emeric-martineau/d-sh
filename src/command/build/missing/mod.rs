///
/// Module to build missing image.
///
/// Release under MIT License.
///
use command::check::get_check_application;
use command::{CommandError, CommandParameter};
use config::Config;

///
/// Return missing applications (applications not build).
///
pub fn get_missing_application(
    cmd_param: &CommandParameter,
    config: &Config,
) -> Result<Vec<String>, CommandError> {
    let list_applications;

    // 1 - We have got configuration
    match get_check_application(cmd_param, config) {
        Ok(r) => list_applications = r,
        Err(err) => return Err(err),
    }

    let list_app: Vec<String> = list_applications
        .into_iter()
        .filter(|a| !a.is_error && !a.is_build)
        .map(|a| a.name)
        .collect();

    Ok(list_app)
}
