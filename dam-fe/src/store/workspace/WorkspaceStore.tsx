import create from 'zustand';
// import { devtools, persist } from 'zustand/middleware';

import WorkspaceState from './WorkspaceState';

const useWorkspaceStore = create<WorkspaceState>()(
    // devtools(
    //     persist(
        (set, get) => ({
            selecting: false,
            selectedImages: [],
            displayStyle: 'GRID', //// TODO: Make a const file and replace this...

            setSelecting: (sel) => set((state) => ({ ...state, selecting: sel})),
            addImageToSelected: (imageID) => set(
                (state) => ({
                    ...state,
                    selectedImages: [...state.selectedImages, imageID]
                })),
            removeImageFromSelected: (imageID) => set(
                (state) => {
                    const selImgs = [...state.selectedImages];

                    for(let i=0; i<selImgs.length; ++i) {
                        if (selImgs[i] === imageID) {
                            selImgs.splice(i, 1);
                            break;
                        }
                    }

                    if (selImgs.length === 0) {
                        return {
                            ...state,
                            selectedImages: [],
                            selecting: false
                        };
                    }

                    return { ...state, selectedImages: selImgs }
                }),
            setDisplayStyle: (dstyle) => set(
                (state) => ({ ...state, displayStyle: dstyle })
            ),
            isSelected: (imageID) => {
                const selImgs = get().selectedImages;

                for(let i=0; i<selImgs.length; ++i) {
                    if (selImgs[i] === imageID) {
                        return true
                    }
                }

                return false;
            }
        })
    //     )
    // )
);

export default useWorkspaceStore;
