use ansi_term::Color::{Blue, Purple};
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use url::Url;

pub trait ToUrl {
    fn to_url(&self) -> Url;
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
    fn to_url(&self) -> Url {
        let l = match self {
            Source::GitHub => "github.com",
            Source::GitLab => "gitlab.com",
            Source::BitBucket => "bitbucket.com",
        };
        Url::parse(&format!("https://{}", l)).unwrap()
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
        let source = Source::from_str(location.nth(0).unwrap());
        let mut path = location.nth(0).unwrap().split("/");
        let user = path.nth(0);
        let path = path.nth(0);
        Ok(Project {
            user: user.unwrap().to_string(),
            source: source.unwrap(),
            repo: path.unwrap().to_string(),
        })
    }
}

impl ToUrl for Project {
    fn to_url(&self) -> Url {
        Url::parse(&format!(
            "{}{}/{}",
            self.source.to_url().as_str(),
            self.user,
            self.repo
        ))
        .unwrap()
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
        write!(f, "{}/{}", Blue.paint(&self.user), Purple.paint(&self.repo))
    }
}

pub trait CloneRepo {
    fn clone_repo(&self, root: &str) -> Result<Repository, git2::Error>;
}

impl CloneRepo for Project {
    fn clone_repo(&self, root: &str) -> Result<Repository, git2::Error> {
        Repository::clone(
            &format!("{}", &self.to_url().as_str()),
            format!("{}/{}", root, self.to_path().to_str().unwrap()),
        )
    }
}
#[cfg(test)]

mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    #[test]
    fn test_display() {
        let json = r#"
        {
            "source": "github",
            "user": "brettm12345",
            "repo": "projection"
        }"#;
        let result: Project = serde_json::from_str(json).unwrap();
        assert_debug_snapshot!(result)
    }
    #[test]
    fn project_to_url() {
        let project = Project {
            user: "brettm12345".to_owned(),
            repo: "projection".to_owned(),
            source: Source::GitHub,
        };
        assert_debug_snapshot!(project.to_url())
    }
    #[test]
    fn serialize_project() {
        let json = r#"
            "source": "github",
            "user": "brettm12345",
            "repo": "projection"
        }"#;
        let result: Project = serde_json::from_str(json).unwrap();
        assert_debug_snapshot!(result)
    }
    #[test]
    fn parse_project_source() {
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
