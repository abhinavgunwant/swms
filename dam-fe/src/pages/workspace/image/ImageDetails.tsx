import {
    useState, useEffect, useTransition, Fragment, ChangeEvent, useRef,
    KeyboardEvent
} from 'react';
import { useParams } from 'react-router-dom';

import { WorkspaceGrid } from '../Workspace';
import { LinkModel, Image, Rendition } from '../../../models';

import {
    Loading, Breadcrumbs, Error, ImagePreview,
} from '../../../components';

import { Accordion } from '../../../components/rendition';

import {
    DeleteImageDialog, RenditionDialog, RenditionDialogMode
} from '../../../components/dialogs';

import {
    TextField as MuiTextField, Typography, Grid, IconButton, OutlinedInput,
    InputAdornment, FormControl, InputLabel, CircularProgress, Box,
} from '@mui/material';
import { Edit, Delete, Check, Close, Visibility } from '@mui/icons-material';

import useAPI from '../../../hooks/useAPI';

import { styled } from '@mui/material/styles';

const TextField = styled(MuiTextField)`
    width: 100%;
`;

const PageTitle = styled(Typography)`
    display: flex;
    justify-content: space-between;
    align-items: center;
`;

const ImageNotFound = styled(Typography)`
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
`;

const ImageDetails = () => {
    const [ breadcrumbLinks, setBreadcrumbLinks ] =
        useState<Array<LinkModel | string>>(['Workspace']);
    const [ image, setImage ] = useState<Image>();
    const [ renditionList, setRenditionList ] = useState<Rendition[]>([]);
    const [ eagerRendition, setEagerRendition ] = useState<boolean>(true);
    const [ loading, setLoading ] = useState<boolean>(true);
    const [ imageNotFound, setImageNotFound ] = useState<boolean>(false);
    const [ edit, setEdit ] = useState<boolean>(false);
    const [ edited, setEdited ] = useState<boolean>(false);
    const [ showPreview, setShowPreview ] = useState<boolean>(false);
    const [ editedTitle, setEditedTitle ] = useState<string>('');
    const [ showErrPopup, setShowErrPopup ] = useState<boolean>(false);
    const [ errPopupText, setErrPopupText ] = useState<string>('Error!');
    const [ updatingName, setUpdatingName ] = useState<boolean>(false);
    const [ showDeleteDialog, setShowDeleteDialog ] = useState<boolean>(false);
    const [ showRenditionDialog, setShowRenditionDialog ] = useState<boolean>(false);
    const [ renditionListUpdated, setRenditionListUpdated ] = useState<boolean>(false);
    // Rendition Dialog Mode
    const [ renDiagMode, setRenDiagMode ] = useState<RenditionDialogMode>('new');
    // Rendition Selection Index
    const [ renSelIndex, setRenSelIndex ] = useState<number>(-1);

    const [ _, startTransition ] = useTransition();

    const imageTitleRef = useRef();

    const { getImage, updateImageTitle } = useAPI();
    const { imageId } = useParams();

    const getImageId: () => number | undefined = () => {
        try {
            if (typeof imageId !== 'undefined') {
                return parseInt(imageId);
            }
        } catch (e) {
            console.log('Error while getting imageId as a number: ', e);
        }

        return undefined;
    };

    const onImageNameChanged = (e: ChangeEvent<HTMLInputElement>) => {
        if (edit) {
            if (!edited) {
                setEdited(true);
            }

            setEditedTitle(e.target.value);
        }
    };

    const onImageNameKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Enter') {
            onEditSave();
        }
    };

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

    const onRenditionClicked = () => startTransition(
        () => setShowRenditionDialog(true)
    );

    const onEagerRenditionChecked = (e: ChangeEvent<HTMLInputElement>) => {
        startTransition(() => setEagerRendition(e.target.checked));
    }

    const onRenditionDialogClosed = () =>
        startTransition(() => setShowRenditionDialog(false));

    const onRenditionSaved = (rendition: Rendition) => {
        if (rendition) {
            startTransition(() => {
                setRenditionList([...renditionList, rendition]);
                setShowRenditionDialog(false);
                setRenditionListUpdated(true);
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
     * Only edits image name!
     */
    const onEdit = () => {
        startTransition(() => {
            setEdit(true);
            setShowErrPopup(false);
        });
    };

    const onPreview = () => startTransition(() => setShowPreview(true));
    const onPreviewClosed = () => startTransition(() => setShowPreview(false));

    const onEditSave = async () => {
        if (image && image.id) {
            startTransition(() => {
                setUpdatingName(true);
                setEdit(false);
            });

            updateImageTitle(image.id, editedTitle)
                .then((resp) => {
                    if (resp.success) {
                        startTransition(() => {
                            setUpdatingName(false);
                            setShowErrPopup(false);
                            setImage({ ...image, title: editedTitle });
                            setEdited(false);
                        });
                    } else {
                        startTransition(() => {
                            setUpdatingName(false);
                            setShowErrPopup(true);
                            setErrPopupText(resp.message);
                            setEdited(false);
                        });
                    }
                });
        }
    };

    const onEditCancel = () => {
        startTransition(() => {
            setEdit(false);
            setEdited(false);
        });
    };

    useEffect(() => {
        //// TODO: query backend and get the full details of the image from
        //// the image id passed into the props
        setBreadcrumbLinks([
            {
                text: 'Workspace',
                to: '/workspace',
            },
            {
                text: 'Product Images',
                to: '/workspace/tree/product-images',
            },
            'Scrumtools.io Logo!',
        ]);

        const exec = async () => {
            if (imageId) {
                try {
                    const imageResponse = await getImage(parseInt(imageId));
                    if (imageResponse) {
                        startTransition(() => {
                            setImage(imageResponse);
                            setLoading(false);
                        });
                    } else {
                        startTransition(() => {
                            setImageNotFound(true);
                            setLoading(false);
                        });
                    }
                } catch (e) { console.log(e); }
            }
        }

        exec();
    }, []);

    /**
     * Effect hook to bring the image title text input into focus with text
     * selected whenever edit button is clicked.
     */
    useEffect(() => {
        if (imageTitleRef && imageTitleRef.current) {
            const wrapper = imageTitleRef.current as HTMLDivElement;
            const inputEl = wrapper.querySelector('input') as HTMLInputElement;

            if (inputEl) {
                inputEl.select();
            }
        }
    }, [ edit ]);

    return <div className="page page--image-details">
        <Breadcrumbs links={ breadcrumbLinks } />

        <WorkspaceGrid>
            {
                loading ?
                    <Loading />
                :
                !imageNotFound ?
                    <Grid container spacing={ 2 }>
                        <Grid item xs={ 12 }>
                            <PageTitle variant="h5">
                                Image Details
                                
                                <Box>
                                    <IconButton
                                        onClick={ onPreview }>
                                        <Visibility />
                                    </IconButton>

                                    <IconButton
                                        color="error"
                                        onClick={ () => { setShowDeleteDialog(true) } }>
                                        <Delete />
                                    </IconButton>
                                </Box>
                            </PageTitle>
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <FormControl sx={{ width: '100%' }}>
                                <InputLabel htmlFor="image-details--image-title">
                                    Image Title
                                </InputLabel>

                                <OutlinedInput
                                    id="image-details--image-title"
                                    value={ edited ? editedTitle : image?.title }
                                    disabled={ !edit }
                                    label="Image Title"
                                    onChange={ onImageNameChanged }
                                    onKeyDown={ onImageNameKeyDown }
                                    ref={ imageTitleRef }
                                    endAdornment={
                                        <InputAdornment position="end">
                                            {
                                                updatingName?
                                                    <CircularProgress size={ 32 } />
                                                :
                                                    edit ?
                                                    <Fragment>
                                                        <IconButton
                                                            onClick={ onEditSave }>
                                                            <Check />
                                                        </IconButton>
                                                        <IconButton
                                                            onClick={
                                                                onEditCancel
                                                            }>
                                                            <Close />
                                                        </IconButton>
                                                    </Fragment>
                                                    :
                                                    <IconButton onClick={ onEdit }>
                                                        <Edit />
                                                    </IconButton>
                                            }
                                        </InputAdornment>
                                    } />

                                <Error on={ showErrPopup }>
                                    { errPopupText }
                                </Error>
                            </FormControl>
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ image?.name }
                                disabled={ true }
                                label="Original filename" />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ image?.width }
                                disabled={ true }
                                label="Width" />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ image?.height }
                                disabled={ true }
                                label="Height" />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ image?.createdOn }
                                disabled={ true }
                                label="Created On" />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ image?.createdBy }
                                disabled={ true }
                                label="Created By" />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ image?.modifiedOn }
                                disabled={ true }
                                label="Modified On" />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ image?.modifiedBy }
                                disabled={ true }
                                label="Modified By" />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <Accordion
                                renditionList={ renditionList }
                                eagerRendition={ eagerRendition }
                                showEagerCheckbox={ renditionListUpdated }
                                onEditRendition={ onEditRendition }
                                onDeleteRendition={ onDeleteRendition }
                                onRenditionClicked={ onRenditionClicked }
                                onEagerRenditionChecked={ onEagerRenditionChecked } />
                        </Grid>
                    </Grid>
                :
                    <ImageNotFound variant="h4" color="error">
                        Error 404: Image Not Found
                    </ImageNotFound>
            }
        </WorkspaceGrid>

        <ImagePreview
            show={ showPreview }
            imageId={ getImageId() }
            onClose={ onPreviewClosed } />

        <DeleteImageDialog
            open={ showDeleteDialog }
            onClose={ () => startTransition(() => setShowDeleteDialog(false)) }
            imageId={ getImageId() || -1 }
            navigateToAfterSuccess="/workspace" />

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

export default ImageDetails;

