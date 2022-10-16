import create from 'zustand';
// import { devtools, persist } from 'zustand/middleware';

import WorkspaceState from './WorkspaceState';

const useWorkspaceStore = create<WorkspaceState>()(
    // devtools(
    //     persist(
        (set, get) => ({
            selecting: false,
            selectedImages: new Set<number>(),
            displayStyle: 'GRID', //// TODO: Make a const file and replace this...
            imageList: [
                {
                    id: 1,
                    thumbnailLocation: '/logo512.png',
                    name: 'Image 1',
                    slug: 'image-1.png',
                    path: '/',
                },
                {
                    id: 2,
                    thumbnailLocation: '/scrumtools-io-logo.png',
                    name: 'Scrumtools.io Logo!',
                    slug: 'scrumtools-io-logo.jpg',
                    path: '/',
                },
                {
                    id: 3,
                    thumbnailLocation: '/logo512.png',
                    name: 'Image 3',
                    slug: 'image-3.jpg',
                    path: '/',
                },
                {
                    id: 4,
                    thumbnailLocation: '/logo512.png',
                    name: 'Image 4',
                    slug: 'image-4.jpg',
                    path: '/',
                },
            ],
            projectList: [],

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
        })
);

export default useWorkspaceStore;
