use crate::git::tag::GitTag;

pub fn find_latest_semver_in_tags (tags: &Vec<GitTag>) -> Option<GitTag> {
  tags.clone().into_iter().max_by_key(|v| v.semver.clone())
}
