import create from 'zustand';
// import { devtools, persist } from 'zustand/middleware';

import WorkspaceState from './WorkspaceState';
import LinkModel from '../../models/LinkModel';
import Folder, { DEFAULT_FOLDER } from '../../models/Folder';
import Project from '../../models/Project';

const useWorkspaceStore = create<WorkspaceState>()(
    // devtools(
    //     persist(
        (set, get) => ({
            selecting: false,
            selectedImages: new Set<number>(),
            selectedFolders: new Set<number>(),
            displayStyle: 'GRID', //// TODO: Make a const file and replace this...
            imageList: [],
            folderList: [],
            projectList: [],
            currentFolder: DEFAULT_FOLDER,
            currentProject: {
                id: 0,
                name: '',
                slug: '',
                description: '',
                restrictUsers: false,
                createdBy: 0,
                modifiedBy: 0,
                createdOn: '',
                modifiedOn: '',
            },
            currentPath: '',
            breadcrumbList: [],
            error: false,

            setSelecting: (sel) => set((state) => ({ ...state, selecting: sel})),
            addImageToSelected: (imageID) => set(
                (state) => {
                    const selImg = state.selectedImages;
                    selImg.add(imageID);

                    return {
                        ...state,
                        selectedImages: selImg,
                        selecting: true,
                    }
                }),
            addFolderToSelected: (folderID) => set(
                (state) => {
                    const selFold = state.selectedFolders;
                    selFold.add(folderID);

                    return {
                        ...state,
                        selectedFolders: selFold,
                        selecting: true,
                    }
                }),
            resetSelectedImages: () => set(
                (state) => {
                    return {
                        ...state,
                        selectedImages: new Set<number>(),
                        selecting: state.selectedFolders.size > 0,
                    }
                }),
            removeImageFromSelected: (imageID) => set(
                (state) => {
                    const selImgs = state.selectedImages;

                    selImgs.delete(imageID);

                    if (selImgs.size === 0) {
                        return {
                            ...state,
                            selectedImages: new Set<number>(),
                            selecting: state.selectedFolders.size > 0,
                        };
                    }

                    return { ...state, selectedImages: selImgs }
                }),
            removeFolderFromSelected: (folderID) => set(
                (state) => {
                    const selFold = state.selectedFolders;

                    selFold.delete(folderID);

                    if (selFold.size === 0) {
                        return {
                            ...state,
                            selectedFolders: new Set<number>(),
                            selecting: state.selectedImages.size > 0,
                        };
                    }

                    return { ...state, selectedFolders: selFold }
                }),
            resetSelectedFolders: () => set(
                (state) => {
                    return {
                        ...state,
                        selectedFolders: new Set<number>(),
                        selecting: state.selectedImages.size > 0,
                    }
                }),
            setDisplayStyle: (dstyle) => set(
                (state) => ({ ...state, displayStyle: dstyle })
            ),
            isSelected: (imageID) => get().selectedImages.has(imageID),
            isFolderSelected: (folderID) => get().selectedFolders.has(folderID),
            setProjectList: (projectList) => set((state) => ({
                ...state, projectList
            })),
            setImageList: (imageList) => set(
                (state) => ({ ...state, imageList: imageList })
            ),
            setFolderList: (folderList) => set(
                (state) => ({ ...state, folderList: folderList })
            ),
            setCurrentFolder: (currentFolder: Folder) => set(
                (state) => ({ ...state, currentFolder: currentFolder })
            ),
            setCurrentProject: (currentProject: Project) => set(
                (state) => ({ ...state, currentProject: currentProject })
            ),
            setCurrentPath: (currentPath: string) => set(
                (state) => ({ ...state, currentPath: currentPath })
            ),
            setBreadcrumbList: (breadcrumbList: Array<LinkModel | string>) => set(
                (state) => ({ ...state, breadcrumbList: breadcrumbList })
            ),
            setError: (error: false | string) => set(
                (state) => ({ ...state, error })
            ),
        })
);

export default useWorkspaceStore;

