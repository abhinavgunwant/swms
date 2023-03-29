import { useEffect, useState, useTransition, MouseEvent } from 'react';
import { useNavigate } from 'react-router-dom';

import { Grid, List, Typography, CircularProgress, Box } from '@mui/material';
import {
    Check, Deselect, Visibility, Delete, SelectAll, DriveFileMove, Add,
} from '@mui/icons-material';

import WorkspaceTopRow from '../WorkspaceTopRow';
import { Thumbnail, ImageListItem, WorkspaceFab } from '../../../components';

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
    const [ selected, setSelected ] = useState<boolean[]>([]);

    const [ _, startTransition ] = useTransition();

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

    const selecting: () => boolean = () => {
        for (let i=0; i<selected.length; ++i) {
            if (selected[i]) {
                return true;
            }
        }

        return false;
    }

    const selectAll = () => {
        if (store.projectList.length) {
            setSelected(Array(store.projectList.length).fill(true));
        }
    }

    const deselectAll = () => {
        if (store.projectList.length) {
            setSelected(Array(store.projectList.length).fill(false));
        }
    };

    const newProject = () => {
        startTransition(() => navigate("/workspace/new-image"));
    };

    /* eslint-disable react-hooks/exhaustive-deps */
    useEffect(() => {
        const func = async () => {
            await getProjects();

            startTransition(() => {
                setLoading(false);

                deselectAll();
            });
        };

        func();
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
                            store.projectList.map((t, i) =>
                                <Thumbnail
                                    id= { t.id }
                                    name={ t.name }
                                    thumbnailLocation=""
                                    key={ t.id }
                                    selected={ selected[i] }
                                    isImage={ false }
                                    actions={[
                                        {
                                            label: 'select',
                                            icon: selected[i] ? <Deselect /> : <Check />,
                                            show: true,
                                            action: (e: MouseEvent<HTMLDivElement>) => {
                                                e.stopPropagation();
                                                const newSelected = [ ...selected ];

                                                newSelected[i] = !selected[i];

                                                setSelected(newSelected);
                                            },
                                        },
                                        {
                                            label: 'delete',
                                            icon: <Delete />,
                                            show: !selecting(),
                                            action: (e: MouseEvent<HTMLDivElement>) => {
                                                e.stopPropagation();

                                                //startTransition(() => setDeleteImageId(t.id));
                                                //onThumbnailDeleteClicked(t.id);
                                            }
                                        },
                                    ]}
                                    onClick={ onThumbnailClicked(t) } />
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
                                    isImage={false}
                                    onClick={ onThumbnailClicked(t) } />
                            )
                        }
                    </List>
        }

        {
            /**
             * Only show new button if user has required permission
             */
            userStore.role.createProject &&
            <WorkspaceFab
                fabs={[
                    {
                        text: 'Select All',
                        onClick: selectAll,
                        variant: "extended",
                        icon: <SelectAll />,
                        show: !selected.reduce((acc, cur) => acc && cur, true),
                    },
                    {
                        text: 'Deselect All',
                        onClick: deselectAll,
                        variant: "extended",
                        icon: <Deselect />,
                        show: selecting(),
                    },
                    {
                        text: 'Move',
                        onClick: () => { /* TODO: Implement! */ },
                        variant: "extended",
                        icon: <DriveFileMove />,
                        show: selecting(),
                    },
                    {
                        text: 'Delete',
                        onClick: () => { /* TODO: Implement! */ },
                        variant: "extended",
                        color: "error",
                        icon: <Delete />,
                        show: selecting(),
                    },
                    {
                        text: 'New',
                        onClick: newProject,
                        variant: "extended",
                        color: 'secondary',
                        icon: <Add />,
                        show: !selecting(),
                    },
                ]} />
        }
    </div>
}

export default Project;

