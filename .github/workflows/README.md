# Release Configurations

This README file documents the format of the `release_configurations.json` file located in this directory.  The file defines Warp's various release channels, and provides values for the various variables that are necessary to run the `create_new_releases.yml` GitHub workflow.

## Fork Releases

Fork maintainers can publish by pushing a `v*` tag or by running the `Fork Release (macOS and Windows)` workflow manually from the GitHub Actions page. Use a unique tag such as `v0.2026.07.10.1`; after all builds pass, the workflow creates one GitHub Release containing a universal macOS DMG plus Windows x64 and ARM64 installers.

The fork workflow uses the `oss` release channel and does not require Warp's private GCP, Sentry, Apple, or Azure credentials. Its artifacts are unsigned community builds, so macOS Gatekeeper and Windows SmartScreen may warn users during installation.

At some point, we may want to replace this document with a JSON schema file (which could be used to validate the correctness of the configuration as part of PR presubmit).

## Fields

* **channel**: The channel's unique identifier
* **type**: The release cadence.  At present, the valid values are "nightly" or "weekly".
* **is_prerelease**: If true, the GitHub release for this channel will be marked as prerelease.
* **is_autopush**: If true, this channel uses the "latest" keyword in `channel_versions.json` to automatically deploy new release candidates.  Non-autopush channels require a manual change in order to deploy them.
* **release_base_name**: The base name of GitHub releases created for this channel.
* **release_body_text**: The body text for GitHub releases created for this channel.
* **sentry_project**: Which Sentry project should receive crash and error reports for this channel.
* **sentry_environment**: The Sentry environment that corresponds to this channel.
* **changelog_slack_channel**: The Slack channel where new changelogs will be posted whenever a new release candidates is cut.
* **gcs_cache_control_value**: The value of the cache-control response header for release DMGs.
  - **IMPORTANT!!**: the value of the cache-control header _must_ be all lowercase; uppercase values will not be respected by Cloud CDN.
