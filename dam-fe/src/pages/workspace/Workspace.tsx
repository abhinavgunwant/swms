import React, { useEffect, useState, useTransition, MouseEvent } from 'react';
import { useNavigate, useParams } from 'react-router-dom';

import { Grid, List, CircularProgress } from '@mui/material';

import {
    Check, Deselect, Visibility, Edit, Delete
} from '@mui/icons-material';

import WorkspaceTopRow from './WorkspaceTopRow';
import WorkspaceFab from './WorkspaceFab';
import { Thumbnail, ImageListItem, ImagePreview } from '../../components';

import LinkModel from '../../models/LinkModel';
import useAPI from '../../hooks/useAPI';

import useUserStore from '../../store/workspace/UserStore';
import useWorkspaceStore from '../../store/workspace/WorkspaceStore';

import { styled } from '@mui/material/styles';

export const WorkspaceGrid = styled(Grid)`
    height: calc(100vh - 9.25rem);
    overflow: auto;
    margin-top: 1rem;
`;

const Workspace = ():React.ReactElement => {
    const [ loading, setLoading ] = useState<boolean>(true);
    const [ showPreview, setShowPreview ] = useState<boolean>(false);

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

    const { getImages } = useAPI();

    const onThumbnailClicked = (path: string, imageId: number) => {
        console.log(store.currentPath);

        navigate('/workspace/image/' + imageId);
    };

    const onPreviewClicked = (id: number) => startTransition(() => {
        console.log('Preview Clicked!!!!');
        setShowPreview(true);
        setPreviewId(id);
    });

    const onPreviewClosed = () => startTransition(() => setShowPreview(false));

    /* eslint-disable react-hooks/exhaustive-deps */
    useEffect(() => {
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

        //// TODO: query backend the path and see if it is valid
        //// if valid, return the type of resource it is.
//        if (imageSlug?.endsWith('.jpg')) {
//            setPageType('IMAGE');
//        }

        // TODO: pass the rquired slug (i.e. project slug if user is at root
        // of project and folder slug if user is in some project)
        getImages(projectSlug||'');
        setLoading(false);
    }, []);

    return <div className="page page--workspace">
        <WorkspaceTopRow links={ store.breadcrumbList } />

        {
            loading ?
                <CircularProgress />
                :
                store.displayStyle === 'GRID' ?
                    <WorkspaceGrid container spacing={2}>
                        {
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
                                                console.log('Delete clicked');
                                            }
                                        },
                                    ]}
                                    onClick={
                                        () => onThumbnailClicked(store.currentPath, t.id)
                                    } />
                            })
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
                                    isImage={true} />
                            )
                        }
                    </List>
        }

        <WorkspaceFab />

        <ImagePreview
            show={ showPreview }
            imageId={ previewId }
            onClose={ onPreviewClosed } />
    </div>;
}

export default Workspace;

