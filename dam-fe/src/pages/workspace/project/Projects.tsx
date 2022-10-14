import React, { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

import Grid from '@mui/material/Grid';
import List from '@mui/material/List';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';

import WorkspaceTopRow from '../WorkspaceTopRow';
import Thumbnail from '../../../components/Thumbnail';
import ImageListItem from '../../../components/ImageListItem';

import useWorkspaceStore from '../../../store/workspace/WorkspaceStore';
import useAPI from '../../../hooks/useAPI';

import { styled } from '@mui/material/styles';

const WorkspaceGrid = styled(Grid)`
    height: calc(100vh - 14rem);
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

const StyledBox = styled(Box)`
    height: 60px;
`;

const Project = () => {
    const store = useWorkspaceStore();
    const { getProjects } = useAPI();
    const navigate = useNavigate();

    const onThumbnailClicked = (slug: string) => {
        return () => navigate('/workspace/tree/' + slug);
    }

    useEffect(() => {
        getProjects();
    }, []);

    return <div className="page page--project">
        <WorkspaceTopRow links={ ['Workspace'] } />

        <StyledBox>
            <Typography variant="h5">
                Projects
            </Typography>

            <Typography variant="subtitle1">
                Click on a project to view images
            </Typography>
        </StyledBox>

        {
            store.displayStyle === 'GRID' ?
                <WorkspaceGrid container spacing={2}>
                    {
                        store.projectList.map(t =>
                            <Thumbnail
                                { ...t }
                                key={ t.id }
                                isImage={false}
                                onClick={ onThumbnailClicked( t.slug ) } />
                        )
                    }
                </WorkspaceGrid>
            :
                <List dense>
                    {
                        store.projectList.map(t =>
                            <ImageListItem {...t} key={t.id} isImage={false} />
                        )
                    }
                </List>
        }
    </div>
}

export default Project;
