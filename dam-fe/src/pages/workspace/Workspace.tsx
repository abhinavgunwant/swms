import React, { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';

import Grid from '@mui/material/Grid';
import List from '@mui/material/List';
import CircularProgress from '@mui/material/CircularProgress';

import WorkspaceTopRow from './WorkspaceTopRow';
import WorkspaceFab from './WorkspaceFab';
import Thumbnail from '../../components/Thumbnail';

import { styled } from '@mui/material/styles';
import useWorkspaceStore from '../../store/workspace/WorkspaceStore';
import ImageListItem from '../../components/ImageListItem';
import LinkModel from '../../models/LinkModel';
import useAPI from '../../hooks/useAPI';

export const WorkspaceGrid = styled(Grid)`
    height: calc(100vh - 9.25rem);
    overflow: auto;
    margin-top: 1rem;
`;

const Workspace = ():React.ReactElement => {
    const store = useWorkspaceStore();

    const navigate = useNavigate();
    /* eslint-disable @typescript-eslint/no-unused-vars */
    const { projectSlug, imageSlug } = useParams();

    const { getImages } = useAPI();

    const [ breadcrumbLinks, setBreadcrumbLinks ]
        = useState<Array<LinkModel | string>>(['Workspace']);
    
    const [ loading, setLoading ] = useState<boolean>(true);
    // const [ pageType, setPageType ] = useState<string>('PROJECT');

    const onThumbnailClicked = (path: string, slug: string) => {
        return () => navigate(
            '/workspace/tree/' + projectSlug +
            (path && path !== '/' ? path : '') + '/' + slug
        );
    };

    /* eslint-disable react-hooks/exhaustive-deps */
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
//        if (imageSlug?.endsWith('.jpg')) {
//            setPageType('IMAGE');
//        }

        // TODO: pass the rquired slug (i.e. project slug if user is at root
        // of project and folder slug if user is in some project)
        getImages(projectSlug||'');
        setLoading(false);
    }, []);

//    if (pageType === 'IMAGE') {
//        console.log('Image page type!');
//        
//        return <ViewImage 
//            projectSlug={ projectSlug }
//            path={ path }
//            imageSlug={ imageSlug } />;
//    }

    return <div className="page page--workspace">
        <WorkspaceTopRow links={ breadcrumbLinks } />

        {
            loading ?
                <CircularProgress />
                :
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
                                    onClick={
                                        onThumbnailClicked('', t.slug )
                                    } />
                            )
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
    </div>;
}

export default Workspace;
