import {
    useState, useEffect, useTransition, ChangeEvent, useRef,
} from 'react';
import { useParams } from 'react-router-dom';

import { WorkspaceGrid } from '../Workspace';
import { LinkModel, Image, Rendition } from '../../../models';

import {
    Loading, Breadcrumbs, ImagePreview, SemiEditableTextField,
} from '../../../components';

import { Accordion } from '../../../components/rendition';

import {
    DeleteItemDialog, RenditionDialog, RenditionDialogMode
} from '../../../components/dialogs';

import {
    TextField as MuiTextField, Typography, Grid, IconButton, Box,
} from '@mui/material';
import { Delete, Visibility } from '@mui/icons-material';

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
    const [ editSlug, setEditSlug ] = useState<boolean>(false);
    const [ edited, setEdited ] = useState<boolean>(false);
    const [ slugEdited, setSlugEdited ] = useState<boolean>(false);
    const [ showPreview, setShowPreview ] = useState<boolean>(false);
    const [ showErrPopup, setShowErrPopup ] = useState<boolean>(false);
    const [ showErrPopupSlug, setShowErrPopupSlug ] = useState<boolean>(false);
    const [ errPopupText, setErrPopupText ] = useState<string>('Error!');
    const [ errPopupSlugText, setErrPopupSlugText ] = useState<string>('Error!');
    const [ updatingName, setUpdatingName ] = useState<boolean>(false);
    const [ updatingSlug, setUpdatingSlug ] = useState<boolean>(false);
    const [ showDeleteDialog, setShowDeleteDialog ] = useState<boolean>(false);
    const [ showRenditionDialog, setShowRenditionDialog ] = useState<boolean>(false);
    const [ renditionListUpdated, setRenditionListUpdated ] = useState<boolean>(false);
    // Rendition Dialog Mode
    const [ renDiagMode, setRenDiagMode ] = useState<RenditionDialogMode>('new');
    // Rendition Selection Index
    const [ renSelIndex, setRenSelIndex ] = useState<number>(-1);
    const [ renditionSaveError, setRenditionSaveError ] = useState<boolean>(false);
    const [ renditionErrorMessage, setRenditionErrorMessage ] = useState<string>('');
    const [ renditionErrorField, setRenditionErrorField ] = useState<string>('');

    const [ _, startTransition ] = useTransition();

    const imageTitleRef = useRef();

    const {
        getImage, updateImage, getRenditions, addRenditions, deleteRendition,
    } = useAPI();
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

    const onImageNameChanged = (changed: boolean) => startTransition(
        () => setEdited(changed)
    );

    const onImageSlugChanged = (changed: boolean) => startTransition(
        () => setSlugEdited(changed)
    );

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
    const onDeleteRendition = async (indx: number) => {
        if (renditionList.length) {
            const deleteResp = await deleteRendition(renditionList[indx].id);

            refreshRenditions();
            // const list = [ ...renditionList ];
            // list.splice(indx, 1);
            // startTransition(() => setRenditionList(list));
        }
    }

    const onRenditionClicked = () => startTransition(
        () => setShowRenditionDialog(true)
    );

    const onEagerRenditionChecked = (e: ChangeEvent<HTMLInputElement>) => {
        startTransition(() => setEagerRendition(e.target.checked));
    }

    const onRenditionDialogClosed = () => startTransition(() => {
        setShowRenditionDialog(false);
        setRenditionSaveError(false);
        setRenditionErrorMessage('');
        setRenditionErrorField('');
        setRenSelIndex(-1);
    });

    const onRenditionSaved = async (rendition: Rendition) => {
        if (rendition) {
            const saveResp = await addRenditions([rendition], eagerRendition);

            if (saveResp.success) {
                startTransition(() => {
                    setRenditionList([...renditionList, rendition]);
                    setShowRenditionDialog(false);
                    setRenditionListUpdated(true);
                });

                refreshRenditions();
            } else {
                console.log('ren messages: ', saveResp.renditionMessages);
                if (saveResp.renditionMessages) {
                    const message = saveResp.renditionMessages[0].message;
                    let field = '';

                    if (/slug/i.test(message)) {
                        field = 'slug';
                    }

                    startTransition(() => {
                        setRenditionSaveError(true);
                        setRenditionErrorMessage(message);

                        if (field) {
                            setRenditionErrorField(field);
                        }
                    });
                } else {
                    startTransition(() => {
                        setRenditionSaveError(true);
                    });
                }
            }
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

    const onPreview = () => startTransition(() => setShowPreview(true));
    const onPreviewClosed = () => startTransition(() => setShowPreview(false));

    const onEditSave = async (editedTitle: string) => {
        if (image && image.id) {
            startTransition(() => {
                setUpdatingName(true);
                setEdit(false);
            });

            const resp = await updateImage({ ...image, title: editedTitle });

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
        }
    };

    const onEditSlugSave = async (editedSlug: string) => {
        if (image && image.id) {
            startTransition(() => {
                setUpdatingSlug(true);
                setEditSlug(false);
            });

            const resp = await updateImage({ ...image, slug: editedSlug });

            if (resp.success) {
                startTransition(() => {
                    setUpdatingSlug(false);
                    setShowErrPopupSlug(false);
                    setImage({ ...image, title: editedSlug });
                    setSlugEdited(false);
                });
            } else {
                startTransition(() => {
                    setUpdatingSlug(false);
                    setShowErrPopupSlug(true);
                    setErrPopupSlugText(resp.message);
                    setSlugEdited(false);
                });
            }
        }
    };

    const getImageDetails = () => {
        const imgId = getImageId();
        if (imgId) {
            try {
                Promise.all([getImage(imgId), getRenditions(imgId)]).then((results) => {
                    const imageResponse = results[0];
                    let renditions: Rendition[];

                    if (results[1].renditions) {
                        renditions = results[1].renditions;
                    }

                    if (imageResponse) {
                        startTransition(() => {
                            setImage(imageResponse);
                            setLoading(false);
                            setRenditionList(renditions);
                        });
                    } else {
                        startTransition(() => {
                            setImageNotFound(true);
                            setLoading(false);
                        });
                    }
                })
            } catch (e) { console.log(e); }
        }
    };

    const refreshRenditions = async () => {
        console.log('refreshing rendition list');

        const renditionResp = await getRenditions(getImageId() || -1);

        if (renditionResp.success) {
            startTransition(() =>
                setRenditionList(renditionResp.renditions || [])
            );
        }
    }

    useEffect(() => {
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

        getImageDetails();
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
                            <SemiEditableTextField
                                label="Title"
                                sx={{ marginTop: 0, width: '100%' }}
                                value={ image?.title }
                                onEdited={ onImageNameChanged }
                                showErrPopup={ showErrPopup }
                                errPopupText={ errPopupText }
                                updating={ updatingName }
                                onSave={ onEditSave } />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <SemiEditableTextField
                                label="Slug"
                                sx={{ marginTop: 0, width: '100%' }}
                                value={ image?.slug }
                                onEdited={ onImageSlugChanged }
                                showErrPopup={ showErrPopupSlug }
                                errPopupText={ errPopupSlugText }
                                updating={ updatingSlug }
                                onSave={ onEditSlugSave } />
                        </Grid>

                        <Grid item xs={ 12 } md={ 6 }>
                            <TextField
                                value={ image?.name }
                                disabled={ true }
                                label="Original filename" />
                        </Grid>

                        <Grid item xs={ 6 } md={ 3 }>
                            <TextField
                                value={ image?.width }
                                disabled={ true }
                                label="Width" />
                        </Grid>

                        <Grid item xs={ 6 } md={ 3 }>
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
                                expand={ true }
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

        <DeleteItemDialog
            open={ showDeleteDialog }
            onClose={ () => startTransition(() => setShowDeleteDialog(false)) }
            imageIDs={ [getImageId() || -1] }
            navigateToAfterSuccess="/workspace" />

        <RenditionDialog
            open={ showRenditionDialog }
            error={ renditionSaveError }
            errorMessage={ renditionErrorMessage }
            errorField={ renditionErrorField }
            imageId={ getImageId() || -1 }
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

