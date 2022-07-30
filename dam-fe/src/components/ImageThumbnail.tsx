import { Fragment } from 'react';

import Grid from '@mui/material/Grid';
import Card from '@mui/material/Card';
import { CardActions, CardContent, CardMedia, IconButton } from '@mui/material';
import Typography from '@mui/material/Typography';
import CheckIcon from '@mui/icons-material/Check';
import DeselectIcon from '@mui/icons-material/Deselect';
import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import { styled } from '@mui/material/styles';

import ImageThumbnailModel from '../models/ImageThumbnailModel';
import useWorkspaceStore from '../store/workspace/WorkspaceStore';

const ThumbnailSubtitle = styled(Typography)`color: #888888;`;

const ThumbnailActions = styled(CardActions)`
    display: flex;
    justify-content: center;
`;

const ImageThumbnail = (props: ImageThumbnailModel) => {
    const store = useWorkspaceStore();

    const fileNameContent = props.thumbnailLocation.split('/');
    const subtitle = fileNameContent[fileNameContent.length - 1];

    const selected = store.isSelected(props.imageID);

    const onSelectClicked = () => {
        if (selected) {
            store.removeImageFromSelected(props.imageID);
        } else {
            store.addImageToSelected(props.imageID);
            store.setSelecting(true);
        }
    };

    return <Grid item xs={12} sm={6} lg={3} xl={2}>
        <Card
            variant="outlined"
            style={{
                backgroundColor: selected ? '#1976d244' : 'transparent',
                boxShadow: selected ? '0 0 0.5rem #1976d244' : 'none',
            }}
            onClick={ store.selecting ? onSelectClicked : undefined }>
            <CardMedia
                component="img"
                height="200"
                image="/logo192.png"
                alt={ props.title } />

            <CardContent>
                <Typography variant="h5">
                    { props.title }
                </Typography>
                <ThumbnailSubtitle>
                    { subtitle }
                </ThumbnailSubtitle>
            </CardContent>

            <ThumbnailActions disableSpacing>
                <IconButton aria-label="select" onClick={ onSelectClicked }>
                    { selected ? <DeselectIcon /> : <CheckIcon />}
                </IconButton>

                {
                    !store.selecting &&
                    <Fragment>
                        <IconButton aria-label="edit">
                            <EditIcon />
                        </IconButton>

                        <IconButton aria-label="edit">
                            <DeleteIcon />
                        </IconButton>
                    </Fragment>
                }
            </ThumbnailActions>
        </Card>
    </Grid>;
}

export default ImageThumbnail;
