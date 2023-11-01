use crate::error;
use crate::error::Error;
use crate::repo::Repo;
use std::fs;

pub struct Init {
    /// Force initialization
    pub force: bool,

    /// The repository path, defaults to the current directory (.)
    pub path: String,
}

impl Init {
    pub fn create(path: String) -> Repo {
        let repo = Repo::new(&path, true);

        info!(
            "create repository worktree on {}",
            &repo.worktree.to_str().unwrap()
        );
        Self::create_worktree(&repo);

        info!("create repository directories...");
        Self::create_dirs(&repo);

        info!("create repository files...");
        Self::create_file(&repo);

        info!("create repository configuration file...");
        Self::create_config(&repo);

        repo
    }

    // Make sure the path either doesn't exist or is an empty dir.
    fn create_worktree(repo: &Repo) {
        let worktree = &repo.worktree;
        let lit_dir = &repo.lit_dir;

        if repo.worktree.exists() {
            if !(worktree.is_dir()) {
                Error::Repo(error::Repo::NotDirectory(worktree.clone())).panic();
            }

            if (lit_dir.exists()) && (lit_dir.read_dir().unwrap().next().is_some()) {
                Error::Repo(error::Repo::NotDirectory(worktree.clone())).panic()
            }
        } else {
            fs::create_dir_all(worktree).unwrap();
        }
    }

    fn create_dirs(repo: &Repo) {
        Repo::repo_dir(&repo.lit_dir, &["branchs"], true).unwrap();
        Repo::repo_dir(&repo.lit_dir, &["objects"], true).unwrap();
        Repo::repo_dir(&repo.lit_dir, &["refs", "tags"], true).unwrap();
        Repo::repo_dir(&repo.lit_dir, &["refs", "heads"], true).unwrap();
    }

    fn create_file(repo: &Repo) {
        fs::write(
            Repo::repo_file(&repo.lit_dir, &["description"], false).unwrap(),
            "Unnamed repository; edit this file 'description' to name the repository.\n",
        )
        .unwrap();

        fs::write(
            Repo::repo_file(&repo.lit_dir, &["HEAD"], false).unwrap(),
            "ref: refs/heads/master\n",
        )
        .unwrap();
    }

    fn create_config(repo: &Repo) {
        let mut conf = ini::Ini::new();
        conf.with_section(Some("core"))
            // The version of the lit_dir format.
            // 0 means the initial format,
            // 1 the same with extensions.
            // If > 1, lit will panic; lit will only accept 0.
            .set("repositoryformatversion", "0")
            // Disable tracking of file mode (permissions) changes in the work tree.
            .set("filemode", "false")
            // Dndicates that this repository has a worktree.
            .set("bare", "false");

        conf.write_to_file(Repo::repo_file(&repo.lit_dir, &["config"], false).unwrap())
            .unwrap()
    }
}