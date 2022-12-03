import create from 'zustand';
// import { devtools, persist } from 'zustand/middleware';

import WorkspaceState from './WorkspaceState';
import Folder from '../../models/Folder';
import Project from '../../models/Project';

const useWorkspaceStore = create<WorkspaceState>()(
    // devtools(
    //     persist(
        (set, get) => ({
            selecting: false,
            selectedImages: new Set<number>(),
            displayStyle: 'GRID', //// TODO: Make a const file and replace this...
            imageList: [],
            folderList: [],
            projectList: [],
            currentFolder: { id: 0, slug: ''},
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

            setSelecting: (sel) => set((state) => ({ ...state, selecting: sel})),
            addImageToSelected: (imageID) => set(
                (state) => {
                    const selImg = state.selectedImages;
                    selImg.add(imageID);
                    return {
                        ...state,
                        selectedImages: selImg,
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
                            selecting: false
                        };
                    }

                    return { ...state, selectedImages: selImgs }
                }),
            setDisplayStyle: (dstyle) => set(
                (state) => ({ ...state, displayStyle: dstyle })
            ),
            isSelected: (imageID) => get().selectedImages.has(imageID),
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
        })
);

export default useWorkspaceStore;

