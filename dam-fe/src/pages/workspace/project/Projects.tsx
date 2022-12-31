import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import Grid from '@mui/material/Grid';
import List from '@mui/material/List';
import Typography from '@mui/material/Typography';
import CircularProgress from '@mui/material/CircularProgress';
import Box from '@mui/material/Box';

import WorkspaceTopRow from '../WorkspaceTopRow';
import WorkspaceFab from '../WorkspaceFab';
import Thumbnail from '../../../components/Thumbnail';
import ImageListItem from '../../../components/ImageListItem';

import useWorkspaceStore from '../../../store/workspace/WorkspaceStore';
import ProjectModel from '../../../models/Project';
import useUserStore from '../../../store/workspace/UserStore';
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
    const [ loading, setLoading ] = useState(true);

    const store = useWorkspaceStore();
    const userStore = useUserStore();
    const { getProjects } = useAPI();
    const navigate = useNavigate();

    const onThumbnailClicked = (p: ProjectModel) => {
        return () => {
            store.setImageList([]);
            store.setCurrentProject(p);
            store.setCurrentFolder({ id: 0, slug: '/' });

            navigate('/workspace/tree/' + p.slug);
        };
    }

    /* eslint-disable react-hooks/exhaustive-deps */
    useEffect(() => {
        getProjects();
        setLoading(false);
    }, []);

    return <div className="page page--project">
        <WorkspaceTopRow links={ ['Workspace'] } />

        <StyledBox>
            <Typography variant="h5">
                Projects
            </Typography>

            {
                !loading &&
                <Typography variant="subtitle1">
                    Click on a project to view images
                </Typography>
            }
        </StyledBox>

        {
            loading ?
                <CircularProgress />
            :
                store.displayStyle === 'GRID' ?
                    <WorkspaceGrid container spacing={2}>
                        {
                            store.projectList.map(t =>
                                <Thumbnail
                                    id= { t.id }
                                    name={ t.name }
                                    thumbnailLocation=""
                                    key={ t.id }
                                    isImage={false}
                                    onClick={ onThumbnailClicked( t) } />
                            )
                        }
                    </WorkspaceGrid>
                :
                    <List dense>
                        {
                            store.projectList.map(t =>
                                <ImageListItem
                                    id= { t.id }
                                    name={ t.name }
                                    thumbnailLocation=""
                                    key={t.id}
                                    isImage={false} />
                            )
                        }
                    </List>
        }

        {
            /**
             * Only show new button if user has required permission
             */
            userStore.role.createProject &&
            <WorkspaceFab inWorkspaceHome={ true } />
        }
    </div>
}

export default Project;

