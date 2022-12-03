import { ChangeEvent, useState, useEffect, useTransition } from 'react';

import {
    Box, Typography, Grid, TextField, Button, IconButton, Tooltip
} from '@mui/material';
import { UploadFile, Edit, Undo } from '@mui/icons-material';

import Breadcrumbs from "../../../components/Breadcrumbs";

import useWorkspaceStore from '../../../store/workspace/WorkspaceStore';

import { styled } from '@mui/material/styles';

const StyledTextField = styled(TextField)`
    width: 100%;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
`;

const StyledGrid = styled(Grid)`
    margin-top: 1rem;
`;

const ImagePreview = styled(Box)`
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: #dddddd;
    width: 100%;
    min-height: 240px;
    border-radius: 1rem;
`

const CenterGrid = styled(Grid)`
    display: flex;
    justify-content: center;
    align-items: center;
`;

const NewImage = () => {
    const [ folderPath, setFolderPath ] = useState<string>('/');
    const [ title, setTitle ] = useState<string>('');
    const [ details, setDetails ] = useState<string>('');
    const [ imageUploaded, setImageUploaded ] = useState<boolean>(false);
    const [ showEditFolderField, setShowEditFolderField ] = useState<boolean>(false);

    const [ _, startTransition ] = useTransition();

    const store = useWorkspaceStore();

    const onTitleChanged = (e: ChangeEvent<HTMLInputElement>) => {
        let slg = e.target.value;

        setTitle(slg);
    }

    const onDetailsChanged = (e: ChangeEvent<HTMLInputElement>) => {
        setDetails(e.target.value);
    }

    const onEditFolderButtonClicked = () => {
        if (showEditFolderField) {
            startTransition(() => setShowEditFolderField(false));
        } else {
            startTransition(() => setShowEditFolderField(true));
        }
    }

    const onFolderPathChanged = (e: ChangeEvent<HTMLInputElement>) => {
        setFolderPath(e.target.value);
    }

    useEffect(() => {
        let path = store.currentProject.slug
            + '/' + store.currentFolder.slug;

        path = path.replaceAll('//', '/');

        setFolderPath(path);
    }, []);

    return <div className="page page--new-image">
        <Breadcrumbs links={[{ text: 'Workspace', to: '/workspace' }, 'New Image']} />

        <Typography variant="h5">
            New Image
        </Typography>

        <Typography variant="subtitle1">
            Enter below details to create new image.
        </Typography>

        <StyledGrid container>
            <Grid item xs={12} lg={6}>
                <Grid container>
                    <Grid item xs={11}>
                        <StyledTextField
                            label="Path"
                            defaultValue="/"
                            disabled={ !showEditFolderField }
                            onChange={ onFolderPathChanged }
                            value={ folderPath }
                            required />
                    </Grid>

                    <CenterGrid item xs={1}>
                        <Tooltip title={
                            (showEditFolderField ? 'Undo ' : '')
                            + 'Edit Folder Path'
                            }>
                            <IconButton
                                color="secondary"
                                onClick={ onEditFolderButtonClicked }>
                                { showEditFolderField ? <Undo /> : <Edit /> }
                            </IconButton>
                        </Tooltip>
                    </CenterGrid>
                </Grid>

                <StyledTextField
                    label="Image Title"
                    onChange={ onTitleChanged }
                    value={ title }
                    required />

                <StyledTextField
                    label="Image Details"
                    rows={3}
                    value={ details }
                    onChange={ onDetailsChanged }
                    multiline />
            </Grid>

            <Grid item xs={12} lg={6} style={{ padding: '0.5rem 1rem' }}>
                <ImagePreview>
                    <Button startIcon={ <UploadFile /> } color="secondary">
                        Upload Image
                    </Button>
                </ImagePreview>
            </Grid>
        </StyledGrid>

        <Button
            variant="contained"
            style={{ marginRight: '0.5rem' }}
            disabled={ folderPath == '' || title == '' }>
            Save
        </Button>

        <Button variant="outlined">Cancel</Button>
    </div>
}

export default NewImage;

