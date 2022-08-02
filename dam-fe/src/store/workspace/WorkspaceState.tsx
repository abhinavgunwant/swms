import ThumbnailItemModel from "../../models/ThumbnailItemModel";
import ProjectListItemModel from "../../models/ProjectListItemModel";

export default interface WorkspaceState {
    /**
     * Whether user is selecting one or more image (or the "Selection" mode is
     * on)
     */
    selecting: boolean;

    /**
     * Array holding imageID of the thumbnail selected
     */
    selectedImages: Set<string> ,

    /**
     * Whether to display list or grid.
     */
    displayStyle: string,
    imageList: ThumbnailItemModel[],
    projectList: ThumbnailItemModel[],

    setSelecting: (sel: boolean) => void;
    addImageToSelected: (imageID: string) => void;
    removeImageFromSelected: (imageID: string) => void;
    setDisplayStyle: (dstyle: string) => void;

    /**
     * Whether the passed imageID matches the selected images.
     */
    isSelected: (imageID: string) => boolean;
}
