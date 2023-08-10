/**
 * Data model behind the project list in workspace
 */
export interface ProjectListItemModel {
    projectID: string;
    thumbnailLocation: string;  // Location (can be file system or other)
    title: string;
}

export default ProjectListItemModel;

