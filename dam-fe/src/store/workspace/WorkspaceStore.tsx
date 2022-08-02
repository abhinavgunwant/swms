import create from 'zustand';
// import { devtools, persist } from 'zustand/middleware';

import WorkspaceState from './WorkspaceState';

const useWorkspaceStore = create<WorkspaceState>()(
    // devtools(
    //     persist(
        (set, get) => ({
            selecting: false,
            selectedImages: new Set<string>(),
            displayStyle: 'GRID', //// TODO: Make a const file and replace this...
            imageList: [
                {
                    id: '1',
                    thumbnailLocation: '/logo512.png',
                    title: 'Image 1',
                    slug: 'image-1.png',
                    path: '/',
                    // extension: 'jpg',
                },
                {
                    id: '2',
                    thumbnailLocation: '/scrumtools-io-logo.png',
                    title: 'Scrumtools.io Logo!',
                    slug: 'scrumtools-io-logo.jpg',
                    path: '/',
                    // extension: 'jpg',
                },
                {
                    id: '3',
                    thumbnailLocation: '/logo512.png',
                    title: 'Image 3',
                    slug: 'image-3.jpg',
                    path: '/',
                    // extension: 'jpg',
                },
                {
                    id: '4',
                    thumbnailLocation: '/logo512.png',
                    title: 'Image 4',
                    slug: 'image-4.jpg',
                    path: '/',
                    // extension: 'jpg',
                },
            ],
            projectList: [
                {
                    id: '1',
                    thumbnailLocation: '/logo512.png',
                    title: 'Product Images',
                    slug: 'product-images',
                    path: '/',
                },
                {
                    id: '2',
                    thumbnailLocation: '/scrumtools-io-logo.png',
                    title: 'Stock Images',
                    slug: 'stock-images',
                    path: '/',
                },
                {
                    id: '3',
                    thumbnailLocation: '/logo512.png',
                    title: 'Cars',
                    slug: 'cars',
                    path: '/',
                },
                {
                    id: '4',
                    thumbnailLocation: '/logo512.png',
                    title: 'Stickers',
                    slug: 'stickers',
                    path: '/',
                },
            ],

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

                    // for(let i=0; i<selImgs.length; ++i) {
                    //     if (selImgs[i] === imageID) {
                    //         selImgs.splice(i, 1);
                    //         break;
                    //     }
                    // }

                    selImgs.delete(imageID);

                    if (selImgs.size === 0) {
                        return {
                            ...state,
                            selectedImages: new Set<string>(),
                            selecting: false
                        };
                    }

                    return { ...state, selectedImages: selImgs }
                }),
            setDisplayStyle: (dstyle) => set(
                (state) => ({ ...state, displayStyle: dstyle })
            ),
            isSelected: (imageID) => get().selectedImages.has(imageID),
            // {
                // const selImgs = get().selectedImages;

                // for(let i=0; i<selImgs.length; ++i) {
                //     if (selImgs[i] === imageID) {
                //         return true
                //     }
                // }

                // return false;

            //     return get().selectedImages.has(imageID);
            // }
        })
    //     )
    // )
);

export default useWorkspaceStore;
