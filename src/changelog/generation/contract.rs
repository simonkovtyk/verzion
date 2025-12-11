use crate::git::log::{GitLog, GitLogStakeholder};

#[derive(Clone, Debug)]
pub struct ChangelogMessage {
  pub content: Option<String>,
  pub log: GitLog
}

#[derive(Clone, Debug)]
pub struct ChangelogData {
  pub features: Option<Vec<ChangelogMessage>>,
  pub fixes: Option<Vec<ChangelogMessage>>,
  pub breaking_changes: Option<Vec<ChangelogMessage>>,
  pub contibutors: Option<Vec<GitLogStakeholder>>
}
