/**
 * Data model behind the image list in workspace
 */
export default interface ThumbnailItemModel {
    id: number;
    thumbnailLocation: string;  // Location (can be file system or other)
    name: string;
    slug: string;
    path: string;
    // extension?: string;
}
