use colored::*;
use fp_core::chain::Chain;
use git2::Error;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use url::{ParseError, Url};

type UrlResult = Result<Url, ParseError>;

pub trait ToUrl {
    fn to_url(&self) -> UrlResult;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    BitBucket,
    GitLab,
    GitHub,
}

impl ToString for Source {
    fn to_string(&self) -> String {
        String::from(match &self {
            Source::BitBucket => "bb",
            Source::GitLab => "gl",
            Source::GitHub => "gh",
        })
    }
}

impl FromStr for Source {
    type Err = String;
    fn from_str(s: &str) -> Result<Source, Self::Err> {
        match s {
            "gh" => Ok(Source::GitHub),
            "gl" => Ok(Source::GitLab),
            "bb" => Ok(Source::BitBucket),
            _ => Err(format!(
                "Failed to parse {}, accepted inputs are gh|gl|bb",
                s
            )),
        }
    }
}

impl ToUrl for Source {
    fn to_url(&self) -> UrlResult {
        Url::parse(&format!(
            "https://{}",
            match self {
                Source::GitHub => "github.com",
                Source::GitLab => "gitlab.com",
                Source::BitBucket => "bitbucket.com",
            }
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Project {
    pub user: String,
    pub repo: String,
    pub source: Source,
}

impl FromStr for Project {
    type Err = String;
    fn from_str(s: &str) -> Result<Project, Self::Err> {
        let mut location = s.split(':');
        let source = location
            .next()
            .chain(|s| Source::from_str(s).ok())
            .ok_or("Error")?;
        let mut path = location
            .next()
            .map(|s| s.split('/'))
            .ok_or("Error")?
            .map(String::from)
            .collect::<Vec<String>>()
            .into_iter();
        Ok(Project {
            user: path.next().ok_or("Error")?,
            source,
            repo: path.next().ok_or("Error")?,
        })
    }
}

impl ToUrl for Project {
    fn to_url(&self) -> UrlResult {
        self.source
            .to_url()?
            .join(&format!("{}/{}", self.user, self.repo))
    }
}

pub trait ToPath {
    fn to_path<P: AsRef<Path>>(&self, root: P) -> PathBuf;
}

impl ToPath for Project {
    fn to_path<P: AsRef<Path>>(&self, root: P) -> PathBuf {
        root.as_ref().join(Path::new(&format!(
            "{}--{}--{}",
            self.source.to_string(),
            self.user,
            self.repo
        )))
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", &self.user.blue(), &self.repo.purple())
    }
}

type RepoResult = Result<Repository, Error>;
pub trait CloneRepo {
    fn clone_repo<P: AsRef<Path>>(&self, root: P) -> RepoResult;
}

impl CloneRepo for Project {
    fn clone_repo<P: AsRef<Path>>(&self, root: P) -> RepoResult {
        println!("{} {}...", "Cloning".green(), self);
        let err = |item: &str| {
            Error::from_str(&format!(
                "{} to parse {} from {}",
                "Failed".red(),
                item.underline(),
                self
            ))
        };
        Repository::clone(
            self.to_url().map_err(|_| err("url"))?.as_str(),
            self.to_path(root).to_str().ok_or_else(|| err("path"))?,
        )
    }
}

pub trait Ensure {
    fn ensure<P: AsRef<Path>>(&self, root: P) -> RepoResult;
}

impl Ensure for Project {
    fn ensure<P: AsRef<Path>>(&self, root: P) -> RepoResult {
        Repository::open(self.to_path(root))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn parse_from_json() {
        let result: Project = serde_json::from_str(
            r#"
        {
            "source": "github",
            "user": "brettm12345",
            "repo": "projection"
        }"#,
        )
        .unwrap();
        assert_debug_snapshot!(result)
    }
    #[test]
    fn parse_and_convert_to_url() {
        assert_debug_snapshot!(Project::from_str("gh:brettm12345/xmonad-config")
            .unwrap()
            .to_url())
    }
}
