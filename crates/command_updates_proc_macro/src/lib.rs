use git2::Repository;
use proc_macro::TokenStream;

#[proc_macro]
pub fn commits_string(_item: TokenStream) -> TokenStream {
    let repo = Repository::open("./").unwrap();
    let mut revwalk = repo.revwalk().unwrap();
    let mut string = String::new();

    revwalk.push_head().unwrap();

    for (i, oid) in revwalk.take(10).enumerate() {
        let oid = oid.unwrap();
        let commit = repo.find_commit(oid).unwrap();

        if i == 0 {
            string += &format!("The last change was <t:{}:R>.\n", commit.time().seconds());
        }

        string += &format!(
            "\n[{}](https://github.com/valentinegb/goober-bot/commit/{}): {}",
            &oid.to_string()[..7],
            oid,
            commit.message().unwrap().lines().next().unwrap(),
        );
    }

    format!("{string:?}").parse().unwrap()
}
