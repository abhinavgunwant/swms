import ThumbnailItemModel from "../../models/ThumbnailItemModel";
import ProjectListItemModel from "../../models/ProjectListItemModel";
import Project from '../../models/Project';

export default interface WorkspaceState {
    /**
     * Whether user is selecting one or more image (or the "Selection" mode is
     * on)
     */
    selecting: boolean;

    /**
     * Array holding imageID of the thumbnail selected
     */
    selectedImages: Set<number> ,

    /**
     * Whether to display list or grid.
     */
    displayStyle: string,
    imageList: ThumbnailItemModel[],
    projectList: Project[],
    // sessionToken: string,

    setSelecting: (sel: boolean) => void;
    addImageToSelected: (imageID: number) => void;
    removeImageFromSelected: (imageID: number) => void;
    setDisplayStyle: (dstyle: string) => void;
    setProjectList: (projectList: Project[]) => void;
    // setSessionToken: (sessionToken: string) => void;

    /**
     * Whether the passed imageID matches the selected images.
     */
    isSelected: (imageID: number) => boolean;
}
