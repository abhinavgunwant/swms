import React, {
    useEffect, useState, useTransition, MouseEvent, Fragment
} from 'react';
import { useNavigate, useParams } from 'react-router-dom';

import {
    Box, Grid, List, CircularProgress, Dialog, DialogTitle, DialogContent,
    ListItem, ListItemText, ListItemIcon,
} from '@mui/material';

import {
    Check, Deselect, Visibility, Delete, SelectAll, Add, DriveFileMove,
    Image, Description,
} from '@mui/icons-material';

import WorkspaceTopRow from './WorkspaceTopRow';
import {
    Thumbnail, ImageListItem, ImagePreview, Error, WorkspaceFab,
} from '../../components';
import {
    DeleteImageDialog, DeleteFolderDialog, NewImageDialog,
} from '../../components/dialogs';

import useAPI from '../../hooks/useAPI';

import useUserStore from '../../store/workspace/UserStore';
import useWorkspaceStore from '../../store/workspace/WorkspaceStore';

import Folder from '../../models/Folder';

import { styled } from '@mui/material/styles';

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
    const [ deleteImageId, setDeleteImageId ] = useState<number>(-1);
    const [ deleteFolderId, setDeleteFolderId ] = useState<number>(-1);
    const [ openNewDialog, setOpenNewDialog ] = useState<boolean>(false);

    /**
     * ID of the image to be previewed
     */
    const [ previewId, setPreviewId ] = useState<number>();

    /* eslint-disable @typescript-eslint/no-unused-vars */
    const [ _, startTransition ] = useTransition();

    const store = useWorkspaceStore();
    //const userStore = useUserStore();

    const navigate = useNavigate();
    /* eslint-disable @typescript-eslint/no-unused-vars */
    const { projectSlug, imageSlug } = useParams();

    const { getImages, deleteImage, getChildren } = useAPI();

    const onImageThumbnailClicked = (path: string, imageId: number) => {
        store.setCurrentPath(window.location.pathname as string);

        navigate('/workspace/image/' + imageId);
    };

    const onFolderThumbnailClicked = (path: string, folder: Folder) => {
        store.setCurrentPath(window.location.pathname as string);
        store.setCurrentFolder(folder);
        navigate('/workspace/folder/' + folder.id);
    };

    const onFolderDescriptionClicked = (folderId: number) => {
        store.setCurrentPath(window.location.pathname as string);

        navigate('/workspace/folder/' + folderId);
    }

    const onPreviewClicked = (id: number) => startTransition(() => {
        console.log('Preview Clicked!!!!');
        setShowPreview(true);
        setPreviewId(id);
    });

    const onPreviewClosed = () => startTransition(() => setShowPreview(false));

    const loadImages = async () => {
        if (projectSlug) {
            for(let i=0; i<store.projectList.length; ++i) {
                if (projectSlug === store.projectList[i].slug) {
                    store.setBreadcrumbList([
                        {
                            text: 'Workspace',
                            to: '/workspace',
                        },
                        store.projectList[i].name,
                    ]);
                    break;
                }
            }
        }

        // TODO: pass the rquired slug (i.e. project slug if user is at root
        // of project and folder slug if user is in some project)
        await getChildren(projectSlug || '');

        startTransition(() => setLoading(false));
    };

    const selectAll = () => {
        store.imageList.forEach((img) => store.addImageToSelected(img.id));
        store.folderList.forEach((fol) => store.addFolderToSelected(fol.id));
    };

    const deselectAll = () => {
        store.resetSelectedImages();
        store.resetSelectedFolders();
    };

    const onNewClicked = () => startTransition(
        () => setOpenNewDialog(true)
    );

    const onNewDialogClosed = () => startTransition(
        () => setOpenNewDialog(false)
    );

    /* eslint-disable react-hooks/exhaustive-deps */
    useEffect(() => { loadImages(); }, []);

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
                            store.folderList.length && 
                            store.folderList.map(t => {
                                const selected = store.isFolderSelected(t.id);

                                return <Thumbnail
                                    key={ t.id }
                                    id={ t.id }
                                    name={ t.title }
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

                                                startTransition(() => setDeleteFolderId(t.id));
                                            }
                                        },
                                    ]}
                                    onClick={
                                        () => onFolderThumbnailClicked(store.currentPath, t)
                                    } />
                            }) 
                        }
                        </Fragment>
                        <Fragment>
                        {
                            store.imageList.length ?
                            store.imageList.map(t => {
                                const selected = store.isSelected(t.id);

                                return <Thumbnail
                                    key={ t.id }
                                    id={ t.id }
                                    name={ t.title }
                                    subtitle={ t.name }
                                    isImage={ true }
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
                                            label: 'preview',
                                            icon: <Visibility />,
                                            show: !store.selecting,
                                            action: (e: MouseEvent<HTMLDivElement>) => {
                                                e.stopPropagation();
                                                onPreviewClicked(t.id);
                                            }
                                        },
                                        {
                                            label: 'delete',
                                            icon: <Delete />,
                                            show: !store.selecting,
                                            action: (e: MouseEvent<HTMLDivElement>) => {
                                                e.stopPropagation();

                                                startTransition(() => setDeleteImageId(t.id));
                                                //onThumbnailDeleteClicked(t.id);
                                            }
                                        },
                                    ]}
                                    onClick={
                                        () => onImageThumbnailClicked(store.currentPath, t.id)
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
                                    thumbnailLocation=""
                                    isImage={true}
                                    onClick={
                                        () => onImageThumbnailClicked(store.currentPath, t.id)
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
                    show: store.imageList.length !== store.selectedImages.size
                        || store.folderList.length !== store.selectedFolders.size,
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
                    onClick: () => { /* TODO: Implement! */ },
                    variant: "extended",
                    icon: <DriveFileMove  />,
                    show: store.selecting,
                },
                {
                    text: 'Delete',
                    onClick: () => { /* TODO: Implement! */ },
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
            onClose={ onPreviewClosed } />

        <Error on={ showError }> { errorText } </Error>

        <DeleteImageDialog
            open={ deleteImageId != -1 }
            onClose={ () => startTransition(() => setDeleteImageId(-1)) }
            imageId={ deleteImageId } />

        <DeleteFolderDialog
            open={ deleteFolderId != -1 }
            onClose={ () => startTransition(() => setDeleteFolderId(-1)) }
            folderId={ deleteFolderId } />

        <NewImageDialog open={ openNewDialog } onClose={ onNewDialogClosed } />
    </div>;
}

export default Workspace;

