import React, {
    useEffect, useRef, useState, useTransition, MouseEvent, Fragment, useCallback,
} from 'react';
import { throttle } from 'lodash';
import { useNavigate, useParams } from 'react-router-dom';

import { Box, Grid, List, CircularProgress } from '@mui/material';

import {
    Check, Deselect, Visibility, Delete, SelectAll, Add, DriveFileMove,
    Description, ContentCopy, Close,
} from '@mui/icons-material';

import WorkspaceTopRow from './WorkspaceTopRow';
import {
    Thumbnail, ImageListItem, ImagePreview, WorkspaceFab,
} from '../../components';
import {
    DeleteItemDialog, NewImageDialog, WIPDialog, ErrorDialog
} from '../../components/dialogs';

import useAPI from '../../hooks/useAPI';
import useImageURL from '../../hooks/useImageURL';

import useWorkspaceStore from '../../store/workspace/WorkspaceStore';

import Folder, { DEFAULT_FOLDER } from '../../models/Folder';

import { generateThumbnailURL } from '../../utils/PathUtils';

import { styled } from '@mui/material/styles';

/**
 * Since the copy button fetches the image (rendition, to be precise) url
 * through an API, we need some state representations for:
 * - When the button is idle (no processing/api calls).
 * - When the url is being fetched from the API (right after the copyt button
 *      is pressed).
 * - Success or error in fetching the rendition url.
 */
type CopyButtonStates = 'loading' | 'success' | 'error' | 'idle';

interface CopyButtonIconProps {
    copyButtonState: CopyButtonStates,
}

const CopyButtonIcon = (props: CopyButtonIconProps) => {
    switch (props.copyButtonState) {
        case 'loading':
            return <CircularProgress />

        case 'success':
            return <Check sx={{ color: 'green' }} />

        case 'error':
            return <Close sx={{ color: 'red' }} />

        default:
            return <ContentCopy />;
    }
};

export const WorkspaceGrid = styled(Grid)`
    height: calc(100vh - 9.25rem);
    overflow: auto;
    margin-top: 1rem;

    &::-webkit-scrollbar {
        background-color: #dddddd;
        width: 10px;
        border-radius: 5px;

        &-thumb {
            background-color: #1976d2;
            border-radius: 5px;
        }
    }
`;

export const NothingMessage = styled(Box)`
    display: flex;
    justify-content: center;
    align-items: center;
    text-align: center;
    color: #777777;

    width: 100%;
    height: 100%;
`;

const Workspace = ():React.ReactElement => {
    const [ loading, setLoading ] = useState<boolean>(true);
    const [ showPreview, setShowPreview ] = useState<boolean>(false);
    const [ showError, setShowError ] = useState<boolean>(false);
    const [ errorText, setErrorText ] = useState<string>('');
    const [ showDeleteDialog, setShowDeleteDialog ] = useState<boolean>(false);
    const [ deleteImageIDs, setDeleteImageIDs ] = useState<Array<number>>([]);
    const [ deleteFolderIDs, setDeleteFolderIDs ] = useState<Array<number>>([]);
    const [ openNewDialog, setOpenNewDialog ] = useState<boolean>(false);
    const [ itemsDeleted, setItemsDeleted ] = useState<boolean>(false);
    const [ showMoveDialog, setShowMoveDialog ] = useState<boolean>(false);
    /**
     * ID of the image to be previewed
     */
    const [ previewId, setPreviewId ] = useState<number>(-1);
    const [ previewSlug, setPreviewSlug ] = useState<string>('');

    const [ copyImageId, setCopyImageId ] = useState<number>(-1);
    const [ copyButtonState, setCopyButtonState ] =
        useState<CopyButtonStates>('idle');

    /* eslint-disable @typescript-eslint/no-unused-vars */
    const [ _, startTransition ] = useTransition();

    const store = useWorkspaceStore();

    const navigate = useNavigate();
    /* eslint-disable @typescript-eslint/no-unused-vars */
    const { '*': path } = useParams();

    const childrenFetched = useRef<boolean>(false);
    
    //console.log('Workspace path: ', path);

    const { getChildren, getFolder } = useAPI(navigate);
    const getImageURL = useImageURL();

    const onImageThumbnailClicked = (imageId: number) => {
        store.setCurrentPath(window.location.pathname as string);

        navigate('/workspace/image/' + imageId);
    };

    const onFolderThumbnailClicked = (folder: Folder) => {
        store.setCurrentPath(window.location.pathname as string);
        store.setCurrentFolder(folder);

        navigate(window.location.pathname + '/' + folder.slug);
    };

    const onFolderDescriptionClicked = (folderId: number) => {
        store.setCurrentPath(window.location.pathname as string);

        navigate('/workspace/folder/' + folderId);
    }

    const showCopyTooltip = (imageId: number) => imageId === copyImageId;

    const onHideCopyTooltip = () => startTransition(
        () => setCopyButtonState('idle')
    );

    const copyButtonTooltip = (imageId: number) => {
        if (!showCopyTooltip(imageId)) {
            return '';
        }

        if (copyButtonState === 'success') {
            return 'URL Copied!';
        }

        if (copyButtonState  === 'error') {
            return 'Error Copying URL! Please try again...';
        }

        return '';
    }

    const onCopyClicked = async (imageId: number, slug: string) => {
        if (store.currentProject && store.currentProject.id) {
            try {
                startTransition(() => {
                    setCopyImageId(imageId);
                    setCopyButtonState('loading');
                });

                const url = await getImageURL(
                    store.currentProject.id, imageId, slug + '/default'
                );

                if (url) {
                    let imageUrl;

                    if (location.origin) {
                        imageUrl = location.origin + url;
                    } else {
                        imageUrl = url;
                    }

                    startTransition(() => setCopyButtonState('success'));

                    navigator.clipboard.writeText(imageUrl);

                } else {
                    startTransition(() => setCopyButtonState('error'));
                    // TODO: error here
                }

                setTimeout(() => {
                    startTransition(() => setCopyButtonState('idle'));
                }, 3000);
            } catch (e) {
                console.log(e);
                // TODO: error here
            }
        } else {
            // TODO: error here
        }
    };

    const onPreviewClicked = (id: number, slug: string) => startTransition(() => {
        setShowPreview(true);
        setPreviewSlug(slug + '/default');
        setPreviewId(id);
    });

    const onPreviewClosed = () => startTransition(() => setShowPreview(false));

    const onErrorDialogClosed = () => startTransition(() => {
        setShowError(false);
        setErrorText('');
    });

    const loadImages = async () => {
        // console.log('current folder: ', store.currentFolder);
        let _type: ('project' | 'folder') = 'project';

        let _path = window.location.pathname as string;
        let breadcrumbList = [];

        if (path) {
            let pathEnd = path.substring(_path.lastIndexOf('/') + 1, _path.length);

            // console.log('pathEnd: ', pathEnd);

            const pathSegments = path.split('/');

            // console.log('pathSegments', pathSegments);

            const projectSlug = path ? pathSegments[0] : '';

            if (pathEnd === store.currentFolder.slug) {
                _type = 'folder';
            }

            if (projectSlug) {
                breadcrumbList.push({
                    text: 'Workspace',
                    to: '/workspace',
                });
            }

            let pathBuilder = [];

            if (pathSegments.length > 0) {
                for (let i=0; i<pathSegments.length; ++i) {
                    const ps = pathSegments[i];
                    pathBuilder.push(ps);

                    breadcrumbList.push({
                        text: ps,
                        to: '/workspace/tree/' + pathBuilder.join('/'),
                    });
                }
            }

            store.setBreadcrumbList(breadcrumbList);
        }

        return getChildren(path || '', _type);
    };

    const selectAll = () => {
        store.imageList.forEach((img) => store.addImageToSelected(img.id));
        store.folderList.forEach((fol) => store.addFolderToSelected(fol.id));
    };

    const deselectAll = () => {
        store.resetSelectedImages();
        store.resetSelectedFolders();
    };

    const onDeleteClicked = () => startTransition(() => {
        if (store.selectedImages.size > 0 || store.selectedFolders.size > 0) {
            setDeleteImageIDs(Array.from(store.selectedImages.values()));
            setDeleteFolderIDs(Array.from(store.selectedFolders.values()));
            setShowDeleteDialog(true);
        }
    });

    const onDeleteDialogClosed = (success: boolean) => startTransition(() => {
        setDeleteImageIDs([]);
        setDeleteFolderIDs([]);
        setShowDeleteDialog(false);
        setItemsDeleted(success);

        if (success) {
            store.setSelecting(false);
            store.resetSelectedImages();
            store.resetSelectedFolders();
        }
    });

    const onNewClicked = () => startTransition(() => {
        let currentPath = path || '';

        setOpenNewDialog(true);

        if (currentPath[0] != '/') {
            currentPath = '/' + currentPath;
        }

        store.setCurrentPath(currentPath);
    });

    const onNewDialogClosed = () => startTransition(
        () => setOpenNewDialog(false)
    );

    const openMoveDialog = () => startTransition(
        () => setShowMoveDialog(true)
    );

    const closeMoveDialog = () => startTransition(
        () => setShowMoveDialog(false)
    );

    const onBackButtonClicked = useCallback(throttle(() => {
        console.log('back button pressed on workspace');
        console.log('current folder: ', store.currentFolder);

        if (store.currentFolder) {
            if (store.currentFolder.parentFolderId) {
                console.log('parent folder id is present');
                getFolder(store.currentFolder.parentFolderId)
                .then(fResp => {
                    if (fResp && fResp.success && fResp.folder) {
                        console.log('setting parent folder as current folder', fResp.folder);
                        store.setCurrentFolder(fResp.folder);
                    }
                });
            } else {
                console.log('reset current folder to default');
                store.setCurrentFolder(DEFAULT_FOLDER);
            }
        }
    }, 100), []);

    useEffect(() => {
        if (!childrenFetched.current) {
            loadImages()
                .then(() => { startTransition(() => setLoading(false)); });
            childrenFetched.current = true;
        }

        if (path) {
            store.setCurrentPath(path);
        }

        // Note: couldn't get addEventListener working for 'popstate'.
        window.onpopstate = onBackButtonClicked;

        return () => {
            window.removeEventListener('popstate', onBackButtonClicked);
        };
    }, []);

    useEffect(() => {
        if (!showDeleteDialog && itemsDeleted) {
            loadImages().then(() => { startTransition(() => setLoading(false)); });

            startTransition(() => {
                setItemsDeleted(false)
                store.setSelecting(false);
            });
        }
    }, [ showDeleteDialog ]);

    useEffect(() => {
        if (store.error) {
            if (showError === false) {
                startTransition(() => {
                    setShowError(true);
                    setErrorText(store.error || '');
                });
            }
        }
    }, [ store.error ]);

    return <div className="page page--workspace">
        <WorkspaceTopRow links={ store.breadcrumbList } />

        {
            loading ?
                <CircularProgress />
                :
                store.displayStyle === 'GRID' ?
                    <WorkspaceGrid container spacing={2}>
                        <Fragment>
                        {
                            store.folderList.length > 0 && 
                            store.folderList.map(t => {
                                const selected = store.isFolderSelected(t.id);

                                return <Thumbnail
                                    key={ t.id }
                                    id={ t.id }
                                    name={ t.title }
                                    subtitle={ t.slug }
                                    thumbnailLocation=""
                                    type="FOLDER"
                                    selected={ selected }
                                    actions={[
                                        {
                                            label: 'select',
                                            icon: selected ? <Deselect /> : <Check />,
                                            show: true,
                                            action: (e: MouseEvent<HTMLDivElement>) => {
                                                e.stopPropagation();

                                                if (selected) {
                                                    store.removeFolderFromSelected(t.id);
                                                } else {
                                                    store.addFolderToSelected(t.id);
                                                }
                                            },
                                        },
                                        {
                                            label: 'description',
                                            icon: <Description />,
                                            show: !store.selecting,
                                            action: (e: MouseEvent<HTMLDivElement>) => {
                                                e.stopPropagation();

                                                onFolderDescriptionClicked(t.id);
                                            }
                                        },
                                        {
                                            label: 'delete',
                                            icon: <Delete />,
                                            show: !store.selecting,
                                            action: (e: MouseEvent<HTMLDivElement>) => {
                                                e.stopPropagation();

                                                startTransition(() => setDeleteFolderIDs([t.id]));
                                            }
                                        },
                                    ]}
                                    onClick={
                                        () => onFolderThumbnailClicked(t)
                                    } />
                            })
                        }
                        </Fragment>
                        <Fragment>
                        {
                            store.imageList.length > 0 ?
                            store.imageList.map(t => {
                                const selected = store.isSelected(t.id);

                                return <Thumbnail
                                    key={ t.id }
                                    id={ t.id }
                                    name={ t.title }
                                    subtitle={ t.slug }
                                    isImage={ true }
                                    thumbnailLocation={
                                        generateThumbnailURL(path||'', t.slug)
                                    }
                                    selected={ selected }
                                    actions={[
                                        {
                                            label: 'select',
                                            icon: selected ? <Deselect /> : <Check />,
                                            show: true,
                                            action: (e: MouseEvent<HTMLDivElement>) => {
                                                e.stopPropagation();
                                                if (selected) {
                                                    store.removeImageFromSelected(t.id);
                                                } else {
                                                    store.addImageToSelected(t.id);
                                                }
                                            },
                                        },
                                        {
                                            label: 'copy',
                                            icon: copyImageId === t.id ?
                                                <CopyButtonIcon
                                                    copyButtonState={
                                                        copyButtonState
                                                    } />
                                                :
                                                <ContentCopy />,
                                            show: !store.selecting,
                                            tooltip: copyButtonTooltip(t.id),
                                            showTooltip: showCopyTooltip(t.id),
                                            onHideTooltip: onHideCopyTooltip,
                                            action: (e: MouseEvent<HTMLDivElement>) => {
                                                e.stopPropagation();
                                                onCopyClicked(t.id, t.slug);
                                            },
                                        },
                                        {
                                            label: 'preview',
                                            icon: <Visibility />,
                                            show: !store.selecting,
                                            action: (e: MouseEvent<HTMLDivElement>) => {
                                                e.stopPropagation();
                                                onPreviewClicked(t.id, t.slug);
                                            }
                                        },
                                        {
                                            label: 'delete',
                                            icon: <Delete />,
                                            show: !store.selecting,
                                            action: (e: MouseEvent<HTMLDivElement>) => {
                                                e.stopPropagation();

                                                startTransition(() => setDeleteImageIDs([t.id]));
                                            }
                                        },
                                    ]}
                                    onClick={
                                        () => onImageThumbnailClicked(t.id)
                                    } />
                            })
                            :
                            <NothingMessage>
                                Nothing to show here. Click on "+ New" to get started!
                            </NothingMessage>
                        }
                        </Fragment>
                    </WorkspaceGrid>
                    :
                    <List dense>
                        {
                            store.imageList.map(t =>
                                <ImageListItem
                                    key={t.id}
                                    id={ t.id }
                                    name={ t.name }
                                    slug={ t.slug }
                                    thumbnailLocation=""
                                    isImage={true}
                                    actions={[
                                        {
                                            label: 'copy',
                                            icon: copyImageId === t.id ?
                                                <CopyButtonIcon
                                                    copyButtonState={
                                                        copyButtonState
                                                    } />
                                                :
                                                <ContentCopy />,
                                            show: true,
                                            text: 'Copy URL',
                                            tooltip: copyButtonTooltip(t.id),
                                            showTooltip: showCopyTooltip(t.id),
                                            onHideTooltip: onHideCopyTooltip,
                                            action: (e: MouseEvent<HTMLDivElement>) => {
                                                e.stopPropagation();
                                                onCopyClicked(t.id, t.slug);
                                            },
                                        },
                                    ]}
                                    onClick={
                                        () => onImageThumbnailClicked(t.id)
                                    } />
                            )
                        }
                    </List>
        }

        <WorkspaceFab
            fabs={[
                {
                    text: 'Select All',
                    onClick: selectAll,
                    variant: "extended",
                    icon: <SelectAll />,
                    show: (store.imageList.length > 0 || store.folderList.length > 0)
                        && (store.imageList.length !== store.selectedImages.size
                        || store.folderList.length !== store.selectedFolders.size),
                },
                {
                    text: 'Deselect All',
                    onClick: deselectAll,
                    variant: "extended",
                    icon: <Deselect />,
                    show: store.selecting,
                },
                {
                    text: 'Move',
                    onClick: () => { openMoveDialog(); },
                    variant: "extended",
                    icon: <DriveFileMove  />,
                    show: store.selecting,
                },
                {
                    text: 'Delete',
                    onClick: onDeleteClicked,
                    variant: "extended",
                    color: "error",
                    icon: <Delete />,
                    show: store.selecting,
                },
                {
                    text: 'New',
                    onClick: onNewClicked,
                    variant: "extended",
                    color: 'secondary',
                    icon: <Add />,
                    show: !store.selecting,
                },
            ]} />

        <ImagePreview
            show={ showPreview }
            imageId={ previewId }
            slug={ previewSlug }
            previewType="rendition"
            onClose={ onPreviewClosed } />

        <ErrorDialog
            open={ showError }
            text={ errorText }
            onClose={ onErrorDialogClosed } />

        <DeleteItemDialog
            open={ showDeleteDialog }
            onClose={ onDeleteDialogClosed }
            imageIDs={ deleteImageIDs }
            folderIDs={ deleteFolderIDs } />

        <NewImageDialog open={ openNewDialog } onClose={ onNewDialogClosed } />

        <WIPDialog open={ showMoveDialog } onClose={ closeMoveDialog } />
    </div>;
}

export default Workspace;

