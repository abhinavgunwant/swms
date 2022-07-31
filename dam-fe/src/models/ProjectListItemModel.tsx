/**
 * Data model behind the project list in workspace
 */
export default interface ProjectListItemModel {
    projectID: string;
    thumbnailLocation: string;  // Location (can be file system or other)
    title: string;
}
