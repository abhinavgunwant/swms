use crate::model::{
    image::Image, rendition::Rendition, folder::Folder, project::Project,
};

pub enum Item {
    Project(Project),
    Folder(Folder),
    Image(Image),
    Rendition(Rendition),
}

