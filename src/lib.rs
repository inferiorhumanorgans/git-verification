#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufReader, Cursor};

    use anyhow::{anyhow, Result};
    use git_repository::bstr::ByteSlice;
    use lzma_rs::xz_decompress;
    use tar::Archive;
    use tempfile::{tempdir, TempDir};

    use git_repository::{discover, Repository, RepositoryState};

    fn run_testcase<F>(name: &'static str, f: F) -> Result<()>
    where F: FnOnce(TempDir, &Repository, &std::path::PathBuf) -> Result<()>
    {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
    
        let tar_path = format!("archives/{}.tar.xz", name);
        let mut tar_expanded = vec![];
        let mut tar_file = BufReader::new(File::open(tar_path).expect("couldn't open tarball"));
        xz_decompress(&mut tar_file, &mut tar_expanded).unwrap();
    
        let mut tarball = Archive::new(Cursor::new(&tar_expanded));
        tarball.unpack(&temp_dir).expect("couldn't find tarball");
    
        let repo_path = temp_dir.path().join(name);
        let repo = discover(&repo_path)?;
        f(temp_dir, &repo, &repo_path)
    }

    // Can we get the name of the (default, unborn) branch in a completely empty repo?
    #[test]
    fn empty_head_is_master() -> Result<()> {
        run_testcase("empty", |_temp_dir, repo, _repo_path| {
            let head = repo.head()?;

            let head_name = head
                .referent_name()
                .ok_or_else(|| anyhow!("oops"))?
                .shorten()
                .to_str()?;
            assert_eq!("master", head_name);

            assert_eq!(RepositoryState::None, repo.in_progress_operation());

            Ok(())
        })
    }

    // Can we count the number of untracked objects in a repo?
    #[test]
    #[ignore="TODO"]
    fn untracked_file() -> Result<()> {
        run_testcase("untracked-file", |_temp_dir, repo, _repo_path| {
            let head = repo.head()?;

            let head_name = head
                .referent_name()
                .ok_or_else(|| anyhow!("oops"))?
                .shorten()
                .to_str()?;
            assert_eq!("master", head_name);

            assert_eq!(RepositoryState::None, repo.in_progress_operation());

            let index = repo.load_index().ok_or_else(|| anyhow!("couldn't load index"))??;
            for entry in index.entries().iter() {
                eprintln!("{:?}", entry.path(&index.state));
            }
            Ok(())
        })
    }

    // Can we identify that a cherry pick operation in progress
    #[test]
    fn cherry_pick() -> Result<()> {
        run_testcase("cherry-pick", |_temp_dir, repo, _repo_path| {
            let head = repo.head()?;
            let head_name = head
                .referent_name()
                .ok_or_else(|| anyhow!("detached head?"))?
                .shorten()
                .to_str()?;
            assert_eq!("master", head_name);

            assert_eq!(RepositoryState::CherryPick, repo.in_progress_operation());

            let index = repo.load_index().ok_or_else(|| anyhow!("couldn't load index"))??;
            for entry in index.entries().iter() {
                eprintln!("{:?} {:?}", entry.path(&index.state), entry.flags);
            }
            Ok(())
        })
    }

    // Can we identify that we're in the middle of an interactive rebase?
    #[test]
    #[ignore="TODO"]
    fn rebase_interactive() -> Result<()> {
        run_testcase("rebase-interactive", |_temp_dir, repo, _repo_path| {
            let head = repo.head()?;
            // TODO: Get rebase head/target
            let head_name = head
                .referent_name()
                .ok_or_else(|| anyhow!("detached head?"))?
                .shorten()
                .to_str()?;
            assert_eq!("master", head_name);

            assert_eq!(RepositoryState::RebaseInteractive, repo.in_progress_operation());

            let index = repo.load_index().ok_or_else(|| anyhow!("couldn't load index"))??;
            for entry in index.entries().iter() {
                eprintln!("{:?} {:?}", entry.path(&index.state), entry.flags);
            }
            Ok(())
        })
    }

    // Can we identify a revert operation when we see it?
    #[test]
    fn revert() -> Result<()> {
        run_testcase("revert", |_temp_dir, repo, _repo_path| {
            let head = repo.head()?;
            let head_name = head
                .referent_name()
                .ok_or_else(|| anyhow!("detached head?"))?
                .shorten()
                .to_str()?;
            assert_eq!("master", head_name);

            assert_eq!(RepositoryState::Revert, repo.in_progress_operation());

            let index = repo.load_index().ok_or_else(|| anyhow!("couldn't load index"))??;
            for entry in index.entries().iter() {
                eprintln!("{:?} {:?}", entry.path(&index.state), entry.flags);
            }
            Ok(())
        })
    }
}
