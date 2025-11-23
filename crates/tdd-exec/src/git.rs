use anyhow::{Context, Result};
use git2::Repository;
use ignore::WalkBuilder;
use std::path::PathBuf;
use tdd_core::{RepoState, Vcs};

pub struct GitVcs {
    root: PathBuf,
}

impl GitVcs {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn repo(&self) -> Result<Repository> {
        Repository::open(&self.root).context("Failed to open git repo")
    }
}

impl Vcs for GitVcs {
    fn init_if_needed(&self) -> Result<()> {
        if !self.root.join(".git").exists() {
            Repository::init(&self.root).context("Failed to init git repo")?;
        }
        Ok(())
    }

    fn read_state(&self) -> Result<RepoState> {
        let repo = self.repo()?;

        // Get last commit message
        let (last_commit_message, last_diff) = if let Ok(head) = repo.head() {
            if let Ok(commit) = head.peel_to_commit() {
                let msg = commit.message().unwrap_or("").to_string();

                // Get diff of the last commit (parent vs commit)
                let diff_str = if commit.parent_count() > 0 {
                    let parent = commit.parent(0)?;
                    let tree = commit.tree()?;
                    let parent_tree = parent.tree()?;
                    let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;

                    // Format diff
                    let mut diff_buf = Vec::new();
                    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
                        let origin = line.origin();
                        match origin {
                            '+' | '-' | ' ' => {
                                diff_buf.push(origin as u8);
                                diff_buf.extend_from_slice(line.content());
                            }
                            _ => {
                                diff_buf.extend_from_slice(line.content());
                            }
                        }
                        true
                    })?;
                    String::from_utf8_lossy(&diff_buf).to_string()
                } else {
                    // Initial commit, diff against empty tree
                    let tree = commit.tree()?;
                    let diff = repo.diff_tree_to_tree(None, Some(&tree), None)?;
                    let mut diff_buf = Vec::new();
                    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
                        let origin = line.origin();
                        match origin {
                            '+' | '-' | ' ' => {
                                diff_buf.push(origin as u8);
                                diff_buf.extend_from_slice(line.content());
                            }
                            _ => {
                                diff_buf.extend_from_slice(line.content());
                            }
                        }
                        true
                    })?;
                    String::from_utf8_lossy(&diff_buf).to_string()
                };

                (msg, diff_str)
            } else {
                (String::new(), String::new())
            }
        } else {
            (String::new(), String::new())
        };

        // List files respecting .gitignore
        let mut files = Vec::new();
        for result in WalkBuilder::new(&self.root)
            .hidden(false)
            .git_ignore(true)
            .build()
        {
            let entry = result?;
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                if let Ok(path) = entry.path().strip_prefix(&self.root) {
                    if !path.starts_with(".git") && !path.starts_with("target") {
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
        files.sort();

        Ok(RepoState {
            last_commit_message,
            last_diff,
            files,
        })
    }

    fn stage_all(&self) -> Result<()> {
        let repo = self.repo()?;
        let mut index = repo.index()?;

        // Add all files (including new, modified, deleted)
        // git2 index.add_all is equivalent to `git add .`
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        Ok(())
    }

    fn commit(&self, message: &str) -> Result<String> {
        let repo = self.repo()?;
        let mut index = repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        let signature = repo.signature()?; // Use default user/email from git config

        let parent_commit = if let Ok(head) = repo.head() {
            Some(head.peel_to_commit()?)
        } else {
            None
        };

        let parents = if let Some(ref c) = parent_commit {
            vec![c]
        } else {
            vec![]
        };

        let oid = repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )?;

        Ok(oid.to_string())
    }

    fn checkout_all(&self) -> Result<()> {
        let repo = self.repo()?;
        // git checkout .
        // Force checkout head to discard changes in working directory
        repo.checkout_head(Some(git2::build::CheckoutBuilder::new().force()))?;
        Ok(())
    }
}
