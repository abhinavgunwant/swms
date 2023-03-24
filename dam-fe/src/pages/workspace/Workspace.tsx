import React, { useEffect, useState, useTransition, MouseEvent } from 'react';
import { useNavigate, useParams } from 'react-router-dom';

import { Box, Grid, List, CircularProgress } from '@mui/material';

import {
    Check, Deselect, Visibility, Edit, Delete
} from '@mui/icons-material';

import WorkspaceTopRow from './WorkspaceTopRow';
import WorkspaceFab from './WorkspaceFab';
import {
    Thumbnail, ImageListItem, ImagePreview, Error,
} from '../../components';
import { DeleteImageDialog } from '../../components/dialogs';

import useAPI from '../../hooks/useAPI';

import useUserStore from '../../store/workspace/UserStore';
import useWorkspaceStore from '../../store/workspace/WorkspaceStore';

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
    const [ showDeleteDialog, setShowDeleteDialog ] = useState<booelan>(false);
    const [ deleteImageId, setDeleteImageId ] = useState<number>(-1);

    /**
     * ID of the image to be previewed
     */
    const [ previewId, setPreviewId ] = useState<number>();

    /* eslint-disable @typescript-eslint/no-unused-vars */
    const [ _, startTransition ] = useTransition();

    const store = useWorkspaceStore();
    const userStore = useUserStore();

    const navigate = useNavigate();
    /* eslint-disable @typescript-eslint/no-unused-vars */
    const { projectSlug, imageSlug } = useParams();

    const { getImages, deleteImage } = useAPI();

    const onThumbnailClicked = (path: string, imageId: number) => {
        store.setCurrentPath(window.location.pathname as string);

        navigate('/workspace/image/' + imageId);
    };

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
        await getImages(projectSlug || '');

        startTransition(() => setLoading(false));
    };

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
                        {
                            store.imageList.length ?
                            store.imageList.map(t => {
                                const selected = store.isSelected(t.id);

                                return <Thumbnail
                                    key={ t.id }
                                    id={ t.id }
                                    name={ t.name }
                                    thumbnailLocation=""
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
                                                    store.setSelecting(true);
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
                                            label: 'edit',
                                            icon: <Edit />,
                                            show: !store.selecting,
                                            action: (e: MouseEvent<HTMLDivElement>) => {
                                                e.stopPropagation();
                                                console.log('Edit clicked');
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
                                        () => onThumbnailClicked(store.currentPath, t.id)
                                    } />
                            })
                            :
                            <NothingMessage>
                                Nothing to show here. Click on "+ New" to get started!
                            </NothingMessage>
                        }
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
                                        () => onThumbnailClicked(store.currentPath, t.id)
                                    } />
                            )
                        }
                    </List>
        }

        <WorkspaceFab />

        <ImagePreview
            show={ showPreview }
            imageId={ previewId }
            onClose={ onPreviewClosed } />

        <Error on={ showError }> { errorText } </Error>

        <DeleteImageDialog
            open={ deleteImageId != -1 }
            onClose={ () => {startTransition(() => setDeleteImageId(-1))} }
            imageId={ deleteImageId } />
    </div>;
}

export default Workspace;

