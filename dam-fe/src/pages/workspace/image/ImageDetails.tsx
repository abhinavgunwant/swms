import { useState, useEffect, useTransition, Fragment, ChangeEvent } from 'react';
import { useNavigate, useParams } from 'react-router-dom';

import { WorkspaceGrid } from '../Workspace';
import LinkModel from '../../../models/LinkModel';
import Image from '../../../models/Image';
import { Loading, Breadcrumbs } from '../../../components';
import {
    TextField as MuiTextField, Typography, Grid, IconButton, OutlinedInput,
    InputAdornment, FormControl, InputLabel,
} from '@mui/material';
import { Edit, Delete, Check, Close } from '@mui/icons-material';

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

const ImageDetails = () => {
    const [ breadcrumbLinks, setBreadcrumbLinks ] =
        useState<Array<LinkModel | string>>(['Workspace']);
    const [ image, setImage ] = useState<Image>();
    const [ loading, setLoading ] = useState<boolean>(true);
    const [ edit, setEdit ] = useState<boolean>(false);
    const [ edited, setEdited ] = useState<boolean>(false);
    const [ editedTitle, setEditedTitle ] = useState<string>('');

    const [ _, startTransition ] = useTransition();

    const { getImage } = useAPI();
    const { imageId } = useParams();

    const onImageNameChanged = (e: ChangeEvent<HTMLInputElement>) => {
        if (edit) {
            if (!edited) {
                setEdited(true);
            }

            setEditedTitle(e.target.value);
        }
    };

    const onEdit = () => {
        startTransition(() => setEdit(true));
    };

    const onDelete = () => {
        // TODO: Implement!
    };

    const onEditSave = () => {
        // TODO: Implement!
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
                        console.log(imageResponse);
                    }
                } catch (e) {
                    console.log(e);
                }
            }
        }

        exec();
    }, []);


    return <div className="page page--view-image">
        <Breadcrumbs links={ breadcrumbLinks } />

        <WorkspaceGrid>
            {
                loading ?
                <Loading />
                :
                <Grid container spacing={ 2 }>
                    <Grid item xs={ 12 }>
                        <PageTitle variant="h5">
                            Image Details
                            
                            <IconButton
                                color="error"
                                onClick={ onDelete }>
                                <Delete />
                            </IconButton>
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
                                endAdornment={
                                    <InputAdornment position="end">
                                        {
                                            edit ?
                                            <Fragment>
                                                <IconButton
                                                    onClick={ onEditSave }>
                                                    <Check />
                                                </IconButton>
                                                <IconButton
                                                    onClick={ onEditCancel }>
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
            }
        </WorkspaceGrid>
    </div>
}

export default ImageDetails;

