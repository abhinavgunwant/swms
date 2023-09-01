import {
    useEffect, useState, useRef, useTransition, ChangeEvent, Fragment,
} from 'react';

import { useNavigate } from 'react-router-dom';

import {
    Typography, Grid, TextField, Button, Box,
    CircularProgress,
} from '@mui/material';

import { UploadFile } from '@mui/icons-material';

import useWorkspaceStore from '../../../store/workspace/WorkspaceStore';
import { UploadImage, Rendition } from '../../../models';
import useAPI from '../../../hooks/useAPI';

import { Breadcrumbs, SemiEditableTextField } from "../../../components";

import {
    RenditionDialog, RenditionDialogMode
} from '../../../components/dialogs';

import { Accordion } from '../../../components/rendition';

import { generateSlug } from '../../../utils/validations';

import { styled } from '@mui/material/styles';

const StyledTextField = styled(TextField)`
    width: 100%;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
`;

const StyledGrid = styled(Grid)`
    margin-top: 1rem;
`;

const thumbnailRendition: Rendition = {
    id: 0,
    imageId: 0,
    height: 200,
    width: 300,
    targetDevice: '',
    slug: 'ui-thumb-default',
    isPublished: true,
    encoding: 'JPG',
    createdOn: '',
    createdBy: 0,
    modifiedOn: '',
    modifiedBy: 0,
};

const defaultRendition: Rendition = {
    ...thumbnailRendition,
    height: 0,
    width: 0,
    slug: 'default',
};

const NewImage = () => {
    const [ slug, setSlug ] = useState<string>('');
    const [ slugEdited, setSlugEdited ] = useState<boolean>(false);
    const [ title, setTitle ] = useState<string>('');
    const [ details, setDetails ] = useState<string>('');
    const [ file, setFile ] = useState<File>();
    const [ renditionList, setRenditionList ] = useState<Rendition[]>([
        thumbnailRendition, defaultRendition
    ]);
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

    const folderPath = (
        store.currentProject.slug+ '/' + store.currentFolder.slug
    ).replaceAll('//', '/');

    const onSlugEdited = (val: boolean) => setSlugEdited(val);

    const onTitleChanged = (e: ChangeEvent<HTMLInputElement>) =>
        setTitle(e.target.value);

    const onDetailsChanged = (e: ChangeEvent<HTMLInputElement>) =>
        setDetails(e.target.value);

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
                slug,
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
        if (!slugEdited) {
            startTransition(() => {
                setSlug(generateSlug(title));
            });
        }
    }, [ title, slugEdited, file ]);

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
                    <StyledTextField
                        label="Path"
                        value={ folderPath }
                        disabled />
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

                <SemiEditableTextField
                    label="Slug"
                    value={ slug }
                    onEdited={ onSlugEdited }
                    onSave={ (updatedVal) => setSlug(updatedVal) } />

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
                    showEagerCheckbox={ true }
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
                disabled={ title === '' || slug === '' || !file }
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

