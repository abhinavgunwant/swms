import {
    useState, useEffect, useTransition, Fragment, ChangeEvent, useRef,
    KeyboardEvent
} from 'react';

import {
    TextField as MuiTextField, Typography, Grid, IconButton, OutlinedInput,
    InputAdornment, FormControl, InputLabel, CircularProgress, Box,
} from '@mui/material';

import { Edit, Delete, Check, Close, Visibility } from '@mui/icons-material';

import {
    Loading, Breadcrumbs, Error, ImagePreview,
} from '../../../components';

import { WorkspaceGrid } from '../Workspace';

import { styled } from '@mui/material/styles';

const TextField = styled(MuiTextField)`
    width: 100%;
`;

const PageTitle = styled(Typography)`
    display: flex;
    justify-content: space-between;
    align-items: center;
`;

const FolderDetails = () => {
    const [ loading, setLoading ] = useState<boolean>(true);
    const [ imageNotFound, setImageNotFound ] = useState<boolean>(false);
    const [ showDeleteDialog, setShowDeleteDialog ] = useState<boolean>(false);
    const [ edited, setEdited ] = useState<boolean>(false);

    const [ _, startTransition ] = useTransition();

    useEffect(() => {
        // TODO: get folder info from the GET folder API.
    }, []);

    return <div className="page page--folder-details">
        <WorkspaceGrid>
            {
                loading ?
                    <Loading />
                :
                !imageNotFound ?
                    <Grid container spacing={ 2 }>
                        <Grid item xs={ 12 }>
                            <PageTitle variant="h5">
                                Folder Details
                                
                                <Box>
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
                            <Typography>Renditions</Typography>
                        </Grid>
                    </Grid>
                :
                    <ImageNotFound variant="h4" color="error">
                        Error 404: Image Not Found
                    </ImageNotFound>
            }
        </WorkspaceGrid>
        <Typography variant="h5">
        </Typography>

        <StyledGrid container>
            <Grid item xs={12}>
                <Typography><strong>Upload Path:</strong> { 'something' }</Typography>
                <StyledTextField required label="Folder Title" />
                <StyledTextField multiline label="Folder Details" rows={3} />
            </Grid>
        </StyledGrid>

        <Button variant="contained" style={{marginRight: '0.5rem'}} disabled>Save</Button>
        <Button variant="outlined">Cancel</Button>
    </div>
};

export default FolderDetails;

