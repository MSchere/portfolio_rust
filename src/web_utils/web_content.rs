use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Deserialize, Clone)]
pub struct Jobs {
    pub tab: String,
    pub title: String,
    pub date: String,
    pub description: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Projects {
    pub title: String,
    pub description: Option<String>,
    pub img: Option<String>,
    #[serde(rename = "ghLink")]
    pub gh_link: Option<String>,
    #[serde(rename = "demoLink")]
    pub demo_link: Option<String>,
    pub technologies: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Header {
    pub item1: String,
    pub item2: String,
    pub item3: String,
    pub item4: String,
    #[serde(rename = "cvBtn")]
    pub cv_btn: String,
    #[serde(rename = "cvName")]
    pub cv_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Banner {
    pub pretitle: String,
    pub description: Vec<String>,
    #[serde(rename = "actionBtn")]
    pub action_btn: String,
}

#[derive(Debug, Deserialize)]
pub struct AboutMe {
    pub title: String,
    pub paragraphs: Vec<String>,
    pub skills: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Experience {
    pub title: String,
    pub jobs: Vec<Jobs>,
}

#[derive(Debug, Deserialize)]
pub struct FeaturedProjects {
    pub title: String,
    pub label: String,
    pub projects: Vec<Projects>,
}

#[derive(Debug, Deserialize)]
pub struct OtherProjects {
    pub title: String,
    pub projects: Vec<Projects>,
}

#[derive(Debug, Deserialize)]
pub struct Contact {
    pub pretitle: String,
    pub title: String,
    pub content: String,
    pub btn: String,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub header: Header,
    pub banner: Banner,
    #[serde(rename = "aboutMe")]
    pub about_me: AboutMe,
    pub experience: Experience,
    #[serde(rename = "featuredProjects")]
    pub featured_projects: FeaturedProjects,
    #[serde(rename = "otherProjects")]
    pub other_projects: OtherProjects,
    pub contact: Contact,
}

pub struct WebContent {
    content: Content,
}

impl WebContent {
    pub fn new() -> WebContent {
    let json_str = include_str!("../assets/i18n/en.json");
    let content: Content = from_str(json_str).expect("Error parsing JSON");
    WebContent { content }
    }

    pub fn get_header(&self) -> &Header {
        &self.content.header
    }

    pub fn get_banner(&self) -> &Banner {
        &self.content.banner
    }

    pub fn get_about_me(&self) -> &AboutMe {
        &self.content.about_me
    }

    pub fn get_experience(&self) -> &Experience {
        &self.content.experience
    }

    pub fn get_featured_projects(&self) -> &FeaturedProjects {
        &self.content.featured_projects
    }

    pub fn get_other_projects(&self) -> &OtherProjects {
        &self.content.other_projects
    }

    pub fn get_contact(&self) -> &Contact {
        &self.content.contact
    }
}
