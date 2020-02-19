use ansi_term::Color::{Blue, Purple};
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub trait ToUrl {
    fn to_url(&self) -> String;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    BitBucket,
    GitLab,
    GitHub,
}

impl ToUrl for Source {
    fn to_url(&self) -> String {
        let base = match self {
            Source::GitHub => "github.com",
            Source::GitLab => "gitlab.com",
            Source::BitBucket => "bitbucket.com",
        };
        format!("https://{}", base)
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match self {
            Source::GitHub => "github",
            Source::BitBucket => "bitbucket",
            Source::GitLab => "gitlab",
        };
        write!(f, "{}", name)
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
        let source = match location.nth(0) {
            Some("gh") => Some(Source::GitHub),
            Some("gl") => Some(Source::GitLab),
            Some("bb") => Some(Source::BitBucket),
            _ => None,
        };
        let mut path = location.nth(0).unwrap().split("/");
        let (user, repo) = (path.nth(0), path.nth(0));
        Ok(Project {
            user: user.unwrap().to_string(),
            repo: repo.unwrap().to_string(),
            source: source.unwrap(),
        })
    }
}

impl ToUrl for Project {
    fn to_url(&self) -> String {
        format!("{}/{}/{}", self.source.to_url(), self.user, self.repo)
    }
}

pub trait ToPath {
    fn to_path(&self) -> PathBuf;
}

impl ToPath for Project {
    fn to_path(&self) -> PathBuf {
        Path::new(&format!("{}--{}--{}", self.source, self.user, self.repo)).to_path_buf()
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
            &self.to_url(),
            format!("{}/{}", root, self.to_path().to_str().unwrap()),
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_display() {
        let json = r#"
        {
            "source": "github",
            "user": "brettm12345",
            "repo": "projection"
        }"#;
        let result: Project = serde_json::from_str(json).unwrap();
        println!("Printing");
        println!("{}", result);
    }
    #[test]
    fn project_to_url() {
        let project = Project {
            user: "brettm12345".to_owned(),
            repo: "projection".to_owned(),
            source: Source::GitHub,
        };
        assert_eq!(
            project.to_url(),
            "https://github.com/brettm12345/projection"
        )
    }
    // #[test]
    // fn serialize_project() {
    //     let json = r#"
    //         "source": "github",
    //         "user": "brettm12345",
    //         "repo": "projection"
    //     }"#;
    //     let result: Project = serde_json::from_str(json).unwrap();
    //     let expected = Project {
    //         user: "brettm12345".to_owned(),
    //         repo: "projection".to_owned(),
    //         source: Source::GitHub,
    //     };
    //     assert_eq!(result, expected)
    // }
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
