import React, { useState } from 'react';

import Grid from '@mui/material/Grid';
import List from '@mui/material/List';

import WorkspaceTopRow from './WorkspaceTopRow';
import WorkspaceFab from './WorkspaceFab';
import ImageThumbnailModel from '../../models/ImageThumbnailModel';
import ImageThumbnail from '../../components/ImageThumbnail';

import { styled } from '@mui/material/styles';
import useWorkspaceStore from '../../store/workspace/WorkspaceStore';
import ImageListItem from '../../components/ImageListItem';

const WorkspaceGrid = styled(Grid)`
    height: calc(100vh - 9.25rem);
    overflow: auto;
    margin-top: 1rem;
`;

const thumbnails: ImageThumbnailModel[] = [
    {
        imageID: '1',
        thumbnailLocation: '/logo512.png',
        title: 'Image 1',
    },
    {
        imageID: '2',
        thumbnailLocation: '/scrumtools-io-logo.png',
        title: 'Scrumtools.io Logo!',
    },
    {
        imageID: '3',
        thumbnailLocation: '/logo512.png',
        title: 'Image 3',
    },
    {
        imageID: '4',
        thumbnailLocation: '/logo512.png',
        title: 'Image 4',
    },
];

const Workspace = ():React.ReactElement => {
    const store = useWorkspaceStore();

    return <div className="page page--workspace">
        <WorkspaceTopRow links={ ['Workspace'] } />

        {
            store.displayStyle === 'GRID' ?
                <WorkspaceGrid container spacing={2}>
                    {
                        thumbnails.map(t =>
                            <ImageThumbnail {...t} key={t.imageID} />
                        )
                    }
                </WorkspaceGrid>
            :
                <List dense>
                    {
                        thumbnails.map(t =>
                            <ImageListItem {...t} key={t.imageID} />
                        )
                    }
                </List>
        }

        <WorkspaceFab />
    </div>;
}

export default Workspace;
