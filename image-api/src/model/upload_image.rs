use serde::Deserialize;

use crate::model::encoding::Encoding;

/**
 * Represents data from the "New Image" form after the iamge has been uploaded.
 */
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadImage {
    pub upload_id: String,
    pub name: String,
    pub title: String,
    pub slug: String,
    pub encoding: Encoding,
    pub project_id: u32,
    pub folder_id: u32,
}

