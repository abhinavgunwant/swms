import { useState, useEffect, useTransition, Fragment } from 'react';
import { useNavigate, useParams } from 'react-router-dom';

import { WorkspaceGrid } from '../Workspace';
import LinkModel from '../../../models/LinkModel';
import Image from '../../../models/Image';
import WorkspaceTopRow from "../WorkspaceTopRow";
import { Thumbnail, Loading, ImageListItem, Breadcrumbs } from '../../../components';
import {
    List, TextField as MuiTextField, Typography, Grid, IconButton,
} from '@mui/material';
import { Edit, Delete } from '@mui/icons-material';

import useWorkspaceStore from '../../../store/workspace/WorkspaceStore';
import useAPI from '../../../hooks/useAPI';

import { styled } from '@mui/material/styles';

const TextField = styled(MuiTextField)`
    width: 100%;
    margin: 0.5rem 0;
`;

const PageTitle = styled(Typography)`
    display: flex;
    justify-content: space-between;
    align-items: center;
`;

interface ImageDetailsProps {
    projectSlug: string | undefined,
    path: string | undefined,
    imageSlug: string | undefined,
}

const ImageDetails = () => {
    const [ breadcrumbLinks, setBreadcrumbLinks ] =
        useState<Array<LinkModel | string>>(['Workspace']);
    const [ image, setImage ] = useState<Image>();
    const [ loading, setLoading ] = useState<boolean>(true);
    const [ edit, setEdit ] = useState<boolean>(false);

    const [ _, startTransition ] = useTransition();

    const navigate = useNavigate();
    const store = useWorkspaceStore();

    const { getImage } = useAPI();
    const { imageId } = useParams();

    const onEdit = () => {
        startTransition(() => setEdit(true));
    };

    const onDelete = () => {
        // TODO: Implement!
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
                            {'Image Details'}

                            <div>
                                <IconButton onClick={ onEdit }>
                                    <Edit />
                                </IconButton>

                                <IconButton color="error" onClick={ onDelete }>
                                    <Delete />
                                </IconButton>
                            </div>
                        </PageTitle>
                    </Grid>

                    <Grid item xs={ 12 } md={ 6 }>
                        <TextField
                            value={ image?.title }
                            disabled={ !edit }
                            label="Image Title" />
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
                            disabled={ !edit }
                            label="Width" />
                    </Grid>

                    <Grid item xs={ 12 } md={ 6 }>
                        <TextField
                            value={ image?.height }
                            disabled={ !edit }
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

