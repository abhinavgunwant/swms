import { useState, useEffect } from 'react';

import { WorkspaceGrid } from '../Workspace';
import LinkModel from '../../../models/LinkModel';
import WorkspaceTopRow from "../WorkspaceTopRow";
import ImageListItem from '../../../components/ImageListItem';
import Thumbnail from '../../../components/Thumbnail';
import List from '@mui/material/List';

import { useNavigate } from 'react-router-dom';
import useWorkspaceStore from '../../../store/workspace/WorkspaceStore';
import useAPI from '../../../hooks/useAPI';

interface ViewImageProps {
    projectSlug: string | undefined,
    path: string | undefined,
    imageSlug: string | undefined,
}

const ViewImage = ({ projectSlug, path, imageSlug }: ViewImageProps) => {
    const store = useWorkspaceStore();
    const [ breadcrumbLinks, setBreadcrumbLinks ] =
        useState<Array<LinkModel | string>>(['Workspace']);

    const navigate = useNavigate();

    const { getImages } = useAPI();

    useEffect(() => {
        console.log('ViewImage!!');
        //// TODO: query backend and get the full details of the image from
        //// the image id passed into the props
        setBreadcrumbLinks([
            {
                text: 'Workspace',
                to: '/workspace',
            },
            {
                text: 'Product Images',
                to: '/workspace/tree/product-images',
            },
            'Scrumtools.io Logo!',
        ]);

        getImages(projectSlug||'');
    }, []);

    const onThumbnailClicked = (path: string, slug: string) => {
        return () => navigate(
            '/workspace/tree/' + projectSlug +
            (path && path !== '/' ? path : '') + '/' + slug + '.jpg'
        );
    };

    return <div className="page page--view-image">
        <WorkspaceTopRow links={ breadcrumbLinks } />

        <WorkspaceGrid>
            {
                store.displayStyle === 'GRID' ?
                    <WorkspaceGrid container spacing={2}>
                        {
                            store.imageList.map(t =>
                                <Thumbnail
                                    key={ t.id }
                                    id={ t.id }
                                    name={ t.name }
                                    thumbnailLocation=""
                                    isImage={ true }
                                    onClick={ onThumbnailClicked('', t.slug ) } />
                            )
                        }
                    </WorkspaceGrid>
                :
                    <List dense>
                        {
                            store.imageList.map(t =>
                                <ImageListItem
                                    key={ t.id }
                                    id={ t.id }
                                    name={ t.name }
                                    thumbnailLocation=""
                                    isImage={ true } />
                            )
                        }
                    </List>
            }
        </WorkspaceGrid>
    </div>
}

export default ViewImage;
