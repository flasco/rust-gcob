use git2::Repository;

pub fn check_branch(branch_name: &str) {
  let repo = Repository::open_from_env();
  repo.unwrap().checkout_head(opts: Option<&mut CheckoutBuilder<'_>>)
}