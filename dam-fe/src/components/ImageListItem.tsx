import { Fragment } from 'react';

import ListItem from '@mui/material/ListItem';
import ListItemIcon from '@mui/material/ListItemIcon';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemAvatar from '@mui/material/ListItemAvatar';
import Avatar from '@mui/material/Avatar';
import Checkbox from '@mui/material/Checkbox';
import Divider from '@mui/material/Divider';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import ListItemText from '@mui/material/ListItemText';
import Typography from '@mui/material/Typography';
import EditIcon from '@mui/icons-material/Edit';
import DriveFileMoveIcon from '@mui/icons-material/DriveFileMove';
import DeleteIcon from '@mui/icons-material/Delete';

import ImageThumbnailModel from '../models/ImageThumbnailModel';
import useWorkspaceStore from '../store/workspace/WorkspaceStore';

import { styled } from '@mui/material/styles';

const StyledAvatar = styled(Avatar)`
    border-radius: 0;
`;

const ActionBox = styled(Box)`
    display: flex;
    justify-content: flex-end;
    align-item: center;
`;

const ButtonLeftMargin = styled(Button)`
    margin-left: 1rem;
`;

const ImageText = styled(ListItemText)`
    width: 400px;
`

const ImageListItem = (props: ImageThumbnailModel) => {
    const store = useWorkspaceStore();

    const selected = store.isSelected(props.imageID);

    const fileNameContent = props.thumbnailLocation.split('/');
    const subtitle = fileNameContent[fileNameContent.length - 1];

    const onSelectClicked = () => {
        if (selected) {
            store.removeImageFromSelected(props.imageID);
        } else {
            store.addImageToSelected(props.imageID);
            store.setSelecting(true);
        }
    };

    return <Fragment>
        <ListItem>
            {/* <ListItemButton> */}
                <ListItemIcon>
                    <Checkbox
                        edge="start"
                        inputProps={{ 'aria-label': 'Select Image'}}
                        checked={ selected }
                        onChange={ onSelectClicked } />
                </ListItemIcon>
            {/* </ListItemButton> */}

            <ListItemAvatar>
                <StyledAvatar alt={ props.title } src={ props.thumbnailLocation } />
            </ListItemAvatar>

            <ListItemButton>
                <ImageText>
                    <Typography variant="h6">
                        { props.title }
                    </Typography>
                </ImageText>

                <ImageText>
                    <Typography variant="subtitle1">
                        { subtitle }
                    </Typography>
                </ImageText>
            </ListItemButton>
            
            <ActionBox>
                <Button variant="outlined" startIcon={ <EditIcon /> }>
                    Edit
                </Button>

                <ButtonLeftMargin
                    variant="outlined"
                    startIcon={ <DriveFileMoveIcon /> }>
                    Move
                </ButtonLeftMargin>

                <ButtonLeftMargin
                    variant="contained"
                    color="error"
                    startIcon={ <DeleteIcon /> }>
                    Delete
                </ButtonLeftMargin>
            </ActionBox>
        </ListItem>

        <Divider />
    </Fragment>;
}

export default ImageListItem;
