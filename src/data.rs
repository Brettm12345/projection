use colored::*;
use fp_core::chain::Chain;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use url::{ParseError, Url};

pub trait ToUrl {
    fn to_url(&self) -> Result<Url, ParseError>;
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
        match &self {
            Source::BitBucket => "bb",
            Source::GitLab => "gl",
            Source::GitHub => "gh",
        }
        .to_string()
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
    fn to_url(&self) -> Result<Url, ParseError> {
        let l = match self {
            Source::GitHub => "github.com",
            Source::GitLab => "gitlab.com",
            Source::BitBucket => "bitbucket.com",
        };
        Url::parse(&format!("https://{}", l))
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
        let source = location.nth(0).chain(|s| Source::from_str(s).ok());
        let mut p = location.nth(0).map(|s| s.split("/")).unwrap();
        let user = p.nth(0).map(String::from);
        let path = p.nth(0).map(String::from);
        Ok(Project {
            user: user.unwrap(),
            source: source.unwrap(),
            repo: path.unwrap(),
        })
    }
}

impl ToUrl for Project {
    fn to_url(&self) -> Result<Url, ParseError> {
        Url::parse(&format!(
            "{}{}/{}",
            self.source.to_url()?.as_str(),
            self.user,
            self.repo
        ))
    }
}

pub trait ToPath {
    fn to_path(&self) -> PathBuf;
}

impl ToPath for Project {
    fn to_path(&self) -> PathBuf {
        Path::new(&format!(
            "{}--{}--{}",
            self.source.to_string(),
            self.user,
            self.repo
        ))
        .to_path_buf()
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", &self.user.blue(), &self.repo.purple())
    }
}

pub trait CloneRepo {
    fn clone_repo(&self, root: &str) -> Result<Repository, git2::Error>;
}

impl CloneRepo for Project {
    fn clone_repo(&self, root: &str) -> Result<Repository, git2::Error> {
        Repository::clone(
            &format!("{}", &self.to_url().unwrap().as_str()),
            format!("{}/{}", root, self.to_path().to_str().unwrap()),
        )
    }
}
#[cfg(test)]

mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use pretty_assertions::assert_eq;

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
    fn convert_to_url() {
        assert_debug_snapshot!(Project {
            user: "brettm12345".to_owned(),
            repo: "projection".to_owned(),
            source: Source::GitHub,
        }
        .to_url())
    }
    #[test]
    fn parse_from_input() {
        assert_eq!(
            Project::from_str("gh:brettm12345/xmonad-config"),
            Ok(Project {
                user: "brettm12345".to_owned(),
                repo: "xmonad-config".to_owned(),
                source: Source::GitHub
            })
        )
    }
}
