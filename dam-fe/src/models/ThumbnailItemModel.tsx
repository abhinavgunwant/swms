/**
 * Data model behind the image list in workspace
 */
export default interface ThumbnailItemModel {
    id: string;
    thumbnailLocation: string;  // Location (can be file system or other)
    title: string;
    slug: string;
    path: string;
    // extension?: string;
}
