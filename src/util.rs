use merge::Merge;

pub fn merge_options<T: Merge> (a: Option<T>, b: Option<T>) -> Option<T> {
  if a.is_none() {
    return b;
  }

  if b.is_none() {
    return a;
  }

  let mut merge = a.unwrap();

  merge.merge(b.unwrap());

  return Some(merge);
}
