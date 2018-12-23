///
/// Module that contain default dockerfile file.
///
/// Release under MIT License.
///

/// Default base from
pub const DOCKERFILE_DEFAULT_FROM: &'static str = "ubuntu:18.04";
/// Default tag base name
pub const DOCKERFILE_DEFAULT_TAG: &'static str = "d-base-image:v1.0.0";

/// Default docker file for base image
pub const DOCKERFILE_BASE_FILENAME: &str = "Dockerfile.hbs";
pub const DOCKERFILE_BASE: &str = r#"FROM {{dockerfile_from}}
{{#if dockerfile_base}}
ARG DEPENDENCIES_ALL

RUN apt-get update && \
    apt-get install -y \
      $DEPENDENCIES_ALL

COPY scripts/entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/bin/sh", "/entrypoint.sh"]
{{/if}}
"#;
