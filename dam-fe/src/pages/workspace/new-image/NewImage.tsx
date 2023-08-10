import {
    ChangeEvent, useState, useEffect, useRef, useTransition, Fragment,
} from 'react';

import { useNavigate } from 'react-router-dom';

import {
    Typography, Grid, TextField, Button, IconButton, Tooltip, Box,
    CircularProgress,
} from '@mui/material';

import { UploadFile, Edit, Undo } from '@mui/icons-material';

import useWorkspaceStore from '../../../store/workspace/WorkspaceStore';
import UploadImage from '../../../models/UploadImage';
import Rendition from '../../../models/Rendition';
import useAPI from '../../../hooks/useAPI';

import { Breadcrumbs } from "../../../components";

import {
    RenditionDialog, RenditionDialogMode
} from '../../../components/dialogs';

import { Accordion } from '../../../components/rendition';

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
    const [ showEditFolderField, setShowEditFolderField ]
        = useState<boolean>(false);
    const [ file, setFile ] = useState<File>();
    const [ renditionList, setRenditionList ] = useState<Rendition[]>([]);
    // Rendition Selection Index
    const [ renSelIndex, setRenSelIndex ] = useState<number>(-1);
    // Rendition Dialog Mode
    const [ renDiagMode, setRenDiagMode ]
        = useState<RenditionDialogMode>('new');
    const [ saving, setSaving ] = useState<boolean>(false);
    const [ eagerRendition, setEagerRendition ] = useState<boolean>(false);
    const [ showRenditionDialog, setShowRenditionDialog ]
        = useState<boolean>(false);

    // eslint-disable-next-line @typescript-eslint/no-unused-vars 
    const [ _, startTransition ] = useTransition();

    const { uploadImage, addRenditions } = useAPI();

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
                console.log(f);
            }
        }
    }

    const onSave = async () => {
        if (saving) {
            return;
        }

        if (file) {
            console.log('project: ', store.currentProject);
            console.log('folder: ', store.currentFolder);

            const uploadImg: UploadImage = {
                uploadId: '',
                name: file.name || '',
                title,
                encoding: 'JPG',
                projectId: store.currentProject.id,
                folderId: store.currentFolder.id,
            };

            setSaving(true);

            const resp = await uploadImage(uploadImg, file);

            console.log('Response: ', resp);

            if (resp.success) {
                console.log('Saving image successful!');
                if (renditionList.length) {
                    console.log('Now creating renditions!');

                    const isoTime = (new Date()).toISOString();

                    let newRenList: Rendition[] = [];
                    
                    renditionList.forEach((rendition) => {
                        newRenList.push({
                            ...rendition,
                            imageId: resp.imageId,
                            createdOn: isoTime,
                            modifiedOn: isoTime,
                        });
                    });

                    const renditionResp = await addRenditions(
                        newRenList, eagerRendition
                    );

                    if (renditionResp.success) {
                        // TODO: What to do here?
                        console.log('Renditions saved successfully!');
                    } else {
                        // TODO: What to do here?
                        console.log('Saving rendition was not successful');
                        return;
                    }
                }

                navigate(-1);
            }
            
            setTimeout(() => setSaving(false), 100);
        }
    }

    const onCancel = () => {
        navigate('/workspace/tree' + store.currentPath);
    };

    const onEagerRenditionChecked = (e: ChangeEvent<HTMLInputElement>) => {
        startTransition(() => setEagerRendition(e.target.checked));
    }

    const onRenditionClicked = () => startTransition(
        () => setShowRenditionDialog(true)
    );

    const onRenditionDialogClosed = () =>
        startTransition(() => setShowRenditionDialog(false));

    const onRenditionSaved = (rendition: Rendition) => {
        if (rendition) {
            startTransition(() => {
                setRenditionList([...renditionList, rendition]);
                setShowRenditionDialog(false);
            });
        }
    }

    /**
     * After the rendition has been edited using the dialog.
     */
    const onRenditionUpdated = (rendition: Rendition) => {
        if (rendition) {
            const list = [ ...renditionList ];

            list.splice(renSelIndex, 1, rendition);

            startTransition(() => {
                setRenditionList(list);
                setShowRenditionDialog(false);
            });
        }
    }

    /**
     * When the edit button against a rendition is clicked.
     */
    const onEditRendition = (indx: number) => {
        startTransition(() => {
            setRenDiagMode('edit');
            setRenSelIndex(indx);
            setShowRenditionDialog(true);
        });
    }

    /**
     * When the delete button against a rendition is clicked.
     */
    const onDeleteRendition = (indx: number) => {
        if (renditionList.length) {
            const list = [ ...renditionList ];
            list.splice(indx, 1);

            startTransition(() => setRenditionList(list));
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
            Enter below details to create a new image.
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
                
                <Accordion
                    renditionList={ renditionList }
                    eagerRendition={ eagerRendition }
                    onEditRendition={ onEditRendition }
                    onDeleteRendition={ onDeleteRendition }
                    onRenditionClicked={ onRenditionClicked }
                    onEagerRenditionChecked={ onEagerRenditionChecked } />
            </Grid>
        </StyledGrid>

        <Box sx={{ marginTop: '1rem' }}>
            <Button
                variant="contained"
                style={{ marginRight: '0.5rem' }}
                disabled={ folderPath === '' || title === '' || !file }
                onClick={ onSave }>
                {
                    saving ?
                        <Fragment>
                            <CircularProgress
                                size={ 16 }
                                color="secondary"
                                sx={{
                                    color: '#ffffff',
                                    marginRight: '1rem',
                                }} />
                            Saving
                        </Fragment>
                    :
                        'Save'
                }
            </Button>

            <Button variant="outlined" onClick={ onCancel }>Cancel</Button>
        </Box>

        <RenditionDialog
            open={ showRenditionDialog }
            onDialogClosed={ onRenditionDialogClosed }
            onRenditionSaved={ onRenditionSaved }
            onRenditionUpdated={ onRenditionUpdated }
            mode={ renDiagMode }
            renditionToEdit={
                renSelIndex > -1 && renditionList.length ?
                    renditionList[renSelIndex]
                :
                    undefined
            } />
    </div>
}

export default NewImage;

