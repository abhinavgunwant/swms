import {
    useState, useEffect, useTransition, Fragment,
} from 'react';

import { useParams, useNavigate } from 'react-router-dom';

import {
    TextField as MuiTextField, Typography, Grid, IconButton, Box, Button,
} from '@mui/material';

import { Edit, Delete } from '@mui/icons-material';

import { Loading, Breadcrumbs } from '../../../components';
import { DeleteItemDialog } from '../../../components/dialogs';
import { WorkspaceGrid } from '../Workspace';

import useAPI from '../../../hooks/useAPI';

import LinkModel from '../../../models/LinkModel';
import Folder from '../../../models/Folder';

import useWorkspaceStore from '../../../store/workspace/WorkspaceStore';

import { styled } from '@mui/material/styles';

const TextField = styled(MuiTextField)`
    width: 100%;
`;

const PageTitle = styled(Typography)`
    display: flex;
    justify-content: space-between;
    align-items: center;
`;

const FolderNotFound = styled(Typography)`
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
`;

const FolderDetails = () => {
    const [ breadcrumbLinks, setBreadcrumbLinks ] =
        useState<Array<LinkModel | string>>(['Workspace']);
    const [ folder, setFolder ] = useState<Folder>();
    const [ editedFolder, setEditedFolder ] = useState<Folder|null>(null);
    const [ edit, setEdit ] = useState<boolean>();
    const [ loading, setLoading ] = useState<boolean>(true);
    const [ folderNotFound, setFolderNotFound ] = useState<boolean>(false);
    const [ showDeleteDialog, setShowDeleteDialog ] = useState<boolean>(false);
    const [ edited, setEdited ] = useState<boolean>(false);

    const [ _, startTransition ] = useTransition();

    const navigate = useNavigate();
    const { getFolder } = useAPI(navigate);
    const { folderId } = useParams();

    const store = useWorkspaceStore();

    const getFolderId: () => number | undefined = () => {
        try {
            if (typeof folderId !== 'undefined') {
                return parseInt(folderId);
            }
        } catch (e) {
            console.log('Error while getting imageId as a number: ', e);
        }

        return undefined;
    };

    const onEdit = () => startTransition(() => setEdit(true));
    const onCancel = () => startTransition(() => setEdit(false));

    const onDeleteDialogClosed = () => startTransition(
        () => setShowDeleteDialog(false)
    );

    useEffect(() => {
        const loadFolder = async () => {
            const id = getFolderId();

            if (typeof id === 'number') {
                const fldr = await getFolder(id);

                if (fldr.success) {
                    startTransition(() => {
                        setFolder(fldr.folder);
                        setLoading(false);
                    });
                    console.log(folder);

                    return;
                }

                if (fldr.message === 'NOT_FOUND') {
                    startTransition(() => {
                        setFolderNotFound(true);
                        setLoading(false);
                    });
                }
            }
        };

        loadFolder();
    }, []);

    return <div className="page page--folder-details">
        <Breadcrumbs links={ breadcrumbLinks } />

        <WorkspaceGrid>
            {
                loading ?
                    <Loading />
                :
                !folderNotFound ?
                    <Grid container spacing={ 2 }>
                        <Grid item xs={ 12 }>
                            <PageTitle variant="h5">
                                Folder Details
                                
                                {
                                    !edit &&
                                    <Box>
                                        <IconButton onClick={ onEdit }>
                                            <Edit />
                                        </IconButton>

                                        <IconButton
                                            color="error"
                                            onClick={ () => {
                                                setShowDeleteDialog(true)
                                            }}>
                                            <Delete />
                                        </IconButton>
                                    </Box>
                                }
                            </PageTitle>
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ folder?.title }
                                disabled={ !edit }
                                label="Title" />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ folder?.slug }
                                disabled={ !edit }
                                label="Slug" />
                        </Grid>

                        <Grid item xs={ 12 }>
                            <TextField
                                value={ folder?.description }
                                disabled={ !edit }
                                rows={ 3 }
                                label="Description"
                                multiline />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ folder?.createdBy }
                                disabled={ true }
                                label="Created By" />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ folder?.createdOn }
                                disabled={ true }
                                label="Created On" />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ folder?.modifiedBy }
                                disabled={ true }
                                label="Last Modified By" />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ folder?.modifiedOn }
                                disabled={ true }
                                label="Last Modified On" />
                        </Grid>

                        {
                            edit &&
                            <Grid item xs={12}>
                                <Button
                                    variant="contained"
                                    style={{marginRight: '0.5rem'}}
                                    disabled={ editedFolder == null }>
                                    Save
                                </Button>

                                <Button variant="outlined" onClick={ onCancel }>
                                    Cancel
                                </Button>
                            </Grid>
                        }
                    </Grid>
                :
                    <FolderNotFound variant="h4" color="error">
                        Error 404: Folder Not Found
                    </FolderNotFound>
            }
            <Fragment>
            
            </Fragment>
        </WorkspaceGrid>

        <DeleteItemDialog
            open={ showDeleteDialog }
            onClose={ onDeleteDialogClosed }
            folderIDs={ [ getFolderId() || -1 ] } />
    </div>
};

export default FolderDetails;

