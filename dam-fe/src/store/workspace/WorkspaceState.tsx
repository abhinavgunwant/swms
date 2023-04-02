import ThumbnailItemModel from "../../models/ThumbnailItemModel";
import ProjectListItemModel from "../../models/ProjectListItemModel";
import Project from '../../models/Project';
import LinkModel from '../../models/LinkModel';
import Image from "../../models/Image";
import Folder from "../../models/Folder";

export default interface WorkspaceState {
    /**
     * Whether user is selecting one or more image/folders (or the "Selection"
     * mode is on).
     */
    selecting: boolean;

    /**
     * Array holding imageID of the thumbnail selected
     */
    selectedImages: Set<number>,

    /**
     * Array holding imageID of the thumbnail selected
     */
    selectedFolders: Set<number>,

    /**
     * Whether to display list or grid.
     */
    displayStyle: "LIST" | "GRID",
    imageList: Image[],
    projectList: Project[],
    folderList: Folder[],
    currentFolder: Folder,
    currentProject: Project,
    currentPath: string,
    breadcrumbList: Array<LinkModel | string>,

    setSelecting: (sel: boolean) => void;
    addImageToSelected: (imageID: number) => void;
    removeImageFromSelected: (imageID: number) => void;
    setDisplayStyle: (dstyle: "LIST" | "GRID") => void;
    setProjectList: (projectList: Project[]) => void;
    setImageList: (imageList: Image[]) => void;
    setFolderList: (folderList: Folder[]) => void;
    setCurrentFolder: (currentFolder: Folder) => void;
    setCurrentProject: (currentProject: Project) => void;
    setCurrentPath: (currentPath: string) => void;
    setBreadcrumbList: (breadcrumbList: Array<LinkModel | string>) => void,

    /**
     * Whether the passed imageID matches the selected images.
     */
    isSelected: (imageID: number) => boolean;

    /**
     * Whether the passed imageID matches the selected images.
     */
    isFolderSelected: (folderID: number) => boolean;
}

