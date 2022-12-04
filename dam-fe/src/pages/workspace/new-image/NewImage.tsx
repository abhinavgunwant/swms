import {
    ChangeEvent, useState, useEffect, useRef, useTransition
} from 'react';

import { useNavigate } from 'react-router-dom';

import {
    Typography, Grid, TextField, Button, IconButton, Tooltip
} from '@mui/material';
import { UploadFile, Edit, Undo } from '@mui/icons-material';

import useWorkspaceStore from '../../../store/workspace/WorkspaceStore';
import ImageModel, { default_image } from '../../../models/Image';
import useAPI from '../../../hooks/useAPI';

import Breadcrumbs from "../../../components/Breadcrumbs";

import { styled } from '@mui/material/styles';

const StyledTextField = styled(TextField)`
    width: 100%;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
`;

const StyledGrid = styled(Grid)`
    margin-top: 1rem;
`;

const CenterGrid = styled(Grid)`
    display: flex;
    justify-content: center;
    align-items: center;
`;

const NewImage = () => {
    const [ folderPath, setFolderPath ] = useState<string>('/');
    const [ title, setTitle ] = useState<string>('');
    const [ details, setDetails ] = useState<string>('');
    const [ showEditFolderField, setShowEditFolderField ] = useState<boolean>(false);
    const [ file, setFile ] = useState<File>();
    const [ saving, setSaving ] = useState<boolean>(false);

    const [ _, startTransition ] = useTransition();

    const { addImage } = useAPI();

    const navigate = useNavigate();

    const fileUploadRef = useRef<HTMLInputElement>(null);

    const store = useWorkspaceStore();

    const onTitleChanged = (e: ChangeEvent<HTMLInputElement>) =>
        setTitle(e.target.value);

    const onDetailsChanged = (e: ChangeEvent<HTMLInputElement>) =>
        setDetails(e.target.value);

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

    const onFileChanged = (e: ChangeEvent<HTMLInputElement>) => {
        if (e && e.target && e.target.files && e.target.files.length) {
            const f = e.target.files[0];

            if (f) {
                startTransition(() => setFile(f));
            }
        }
    }

    const onSave = async () => {
        if (file) {
            const image = default_image();

            const resp = await addImage(image, file);
            console.log(resp);

            if (resp.success) {
                navigate(-1);
            }
        }
    }

    useEffect(() => {
        let path = store.currentProject.slug
            + '/' + store.currentFolder.slug;

        path = path.replaceAll('//', '/');

        setFolderPath(path);
    }, []);

    return <div className="page page--new-image">
        <Breadcrumbs links={[
            { text: 'Workspace', to: '/workspace' }, 'New Image'
        ]} />

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

                <Grid container>
                    <Button
                        variant="outlined"
                        startIcon={ <UploadFile /> }
                        color="secondary"
                        component="label">

                        Upload Image
                        <input
                            accept="image/*"
                            type="file"
                            ref={ fileUploadRef }
                            onChange={ onFileChanged }
                            hidden />
                    </Button>

                    {
                        file &&
                        <Typography sx={{
                            display: 'flex',
                            alignItems: 'center',
                            paddingLeft: '1rem',
                        }}>
                            { file && file.name }
                        </Typography>
                    }
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
        </StyledGrid>

        <Button
            variant="contained"
            style={{ marginRight: '0.5rem' }}
            disabled={ folderPath == '' || title == '' || !file }
            onClick={ onSave }>
            { saving ? 'Saving' : 'Save' }
        </Button>

        <Button variant="outlined">Cancel</Button>
    </div>
}

export default NewImage;

