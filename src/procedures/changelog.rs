use crate::{changelog::handler::generate_changelog, config::Config, fs::write_str_to_file, git::{add::add, commit::commit, config::GitRulesetResult, log::GitLog}, std::command::CommandOptions};

pub struct CreateChangelogResult {
  pub changelog: String,
  pub git_ruleset_result: Option<GitRulesetResult>
}

pub fn create_changelog (
  logs: &Vec<GitLog>
) -> Option<CreateChangelogResult> {
  let config = Config::inject();

  let changelog_config = config.changelog.clone()?;

  if changelog_config.enabled.unwrap_or(false) {
    return None;
  }

  let changelog = generate_changelog(logs);

  if let Some(changelog_path) = changelog_config.path {
    write_str_to_file(&changelog_path, changelog.as_str());

    if let Some(git_ruleset) = changelog_config.git_ruleset {
      if git_ruleset.commit.unwrap_or(false) {
        add(&changelog_path, CommandOptions {
          cwd: config.cwd.clone()
        }).ok()?;

        commit(git_ruleset.commit_msg, command_options)
      }

      return Some(CreateChangelogResult {
        changelog,
        git_ruleset_result: Some(GitRulesetResult::from_ruleset(&git_ruleset, vec![changelog_path]))
      });
    }
  }

  Some(
    CreateChangelogResult {
      changelog,
      git_ruleset_result: None 
    }
  )
}
