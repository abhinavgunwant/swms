import { useState, useEffect, useTransition } from 'react';
import { useNavigate, useParams } from 'react-router-dom';

import { WorkspaceGrid } from '../Workspace';
import LinkModel from '../../../models/LinkModel';
import Image from '../../../models/Image';
import WorkspaceTopRow from "../WorkspaceTopRow";
import { Thumbnail, Loading, ImageListItem } from '../../../components';
import { List, TextField as MuiTextField } from '@mui/material';

import useWorkspaceStore from '../../../store/workspace/WorkspaceStore';
import useAPI from '../../../hooks/useAPI';

import { styled } from '@mui/material/styles';

const TextField = styled(MuiTextField)`
    margin: 0.5rem 0;
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
        <WorkspaceTopRow links={ breadcrumbLinks } />

        <WorkspaceGrid>
            {
                loading ?
                <Loading />
                :
                <TextField value={ image?.title } disabled={ !edit } label="Image Title" />
            }
        </WorkspaceGrid>
    </div>
}

export default ImageDetails;

