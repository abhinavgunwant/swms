import React, { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';

import Grid from '@mui/material/Grid';
import List from '@mui/material/List';

import WorkspaceTopRow from './WorkspaceTopRow';
import WorkspaceFab from './WorkspaceFab';
import Thumbnail from '../../components/Thumbnail';

import { styled } from '@mui/material/styles';
import useWorkspaceStore from '../../store/workspace/WorkspaceStore';
import ImageListItem from '../../components/ImageListItem';
import LinkModel from '../../models/LinkModel';
import ViewImage from './view-image/ViewImage';

const WorkspaceGrid = styled(Grid)`
    height: calc(100vh - 9.25rem);
    overflow: auto;
    margin-top: 1rem;
`;

const Workspace = ():React.ReactElement => {
    const store = useWorkspaceStore();

    const navigate = useNavigate();
    const { projectSlug, path, imageSlug } = useParams();

    console.log('Image Slug: ', imageSlug);
    

    const [ breadcrumbLinks, setBreadcrumbLinks ]
        = useState<Array<LinkModel | string>>(['Workspace']);

    const [ pageType, setPageType ] = useState('LIST');

    const onThumbnailClicked = (path: string, slug: string) => {
        return () => navigate('/workspace/tree/' + projectSlug + (path && path !== '/' ? path : '') + '/' + slug);
    };

    useEffect(() => {
        if (projectSlug) {
            for(let i=0; i<store.projectList.length; ++i) {
                if (projectSlug === store.projectList[i].slug) {
                    setBreadcrumbLinks([
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
        if (imageSlug === 'scrumtools-io-logo.jpg') {
            setPageType('IMAGE');
        }
    }, []);

    if (pageType === 'IMAGE') {
        console.log('Image page type!');
        
        return <ViewImage imageId={ imageSlug } />;
    }

    return <div className="page page--workspace">
        <WorkspaceTopRow links={ breadcrumbLinks } />

        {
            store.displayStyle === 'GRID' ?
                <WorkspaceGrid container spacing={2}>
                    {
                        store.imageList.map(t =>
                            <Thumbnail
                                // { ...t }
                                key={ t.id }
                                id={ t.id }
                                name={ t.name }
                                thumbnailLocation=""
                                isImage={ true }
                                onClick={ onThumbnailClicked( t.path, t.slug ) } />
                        )
                    }
                </WorkspaceGrid>
            :
                <List dense>
                    {
                        store.imageList.map(t =>
                            <ImageListItem
                                // {...t}
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
    </div>;
}

export default Workspace;
