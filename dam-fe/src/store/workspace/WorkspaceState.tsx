import ThumbnailItemModel from "../../models/ThumbnailItemModel";
import ProjectListItemModel from "../../models/ProjectListItemModel";
import Project from '../../models/Project';
import Image from "../../models/Image";
import Folder from "../../models/Folder";

export default interface WorkspaceState {
    /**
     * Whether user is selecting one or more image (or the "Selection" mode is
     * on)
     */
    selecting: boolean;

    /**
     * Array holding imageID of the thumbnail selected
     */
    selectedImages: Set<number>,

    /**
     * Whether to display list or grid.
     */
    displayStyle: string,
    imageList: Image[],
    projectList: Project[],
    folderList: Folder[],
    currentFolder: Folder,
    currentProject: Project,

    setSelecting: (sel: boolean) => void;
    addImageToSelected: (imageID: number) => void;
    removeImageFromSelected: (imageID: number) => void;
    setDisplayStyle: (dstyle: string) => void;
    setProjectList: (projectList: Project[]) => void;
    setImageList: (imageList: Image[]) => void;
    setFolderList: (folderList: Folder[]) => void;
    setCurrentFolder: (currentFolder: Folder) => void;
    setCurrentProject: (currentProject: Project) => void;

    /**
     * Whether the passed imageID matches the selected images.
     */
    isSelected: (imageID: number) => boolean;
}

