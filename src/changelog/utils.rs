use crate::git::log::{GitLog, GitLogStakeholder};

pub fn get_contributors (logs: &Vec<GitLog>) -> Vec<GitLogStakeholder> {
  logs.iter().map(|v| v.comitter.clone()).collect()
}
