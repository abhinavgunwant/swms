import { Fragment } from 'react';

import {
    ListItem, ListItemIcon, ListItemButton, ListItemAvatar, Avatar, Checkbox,
    Divider, Box, Button, ListItemText, Typography,
} from '@mui/material';

import { DriveFileMove, Delete } from '@mui/icons-material';

import ThumbnailExtendedProps from '../models/ThumbnailExtendProps';
import useWorkspaceStore from '../store/workspace/WorkspaceStore';

import { styled } from '@mui/material/styles';

const StyledAvatar = styled(Avatar)`
    border-radius: 0;
`;

const ActionBox = styled(Box)`
    display: flex;
    justify-content: flex-end;
    align-items: center;
`;

const ButtonLeftMargin = styled(Button)`
    margin-left: 1rem;
`;

const ImageText = styled(ListItemText)`
    width: 400px;
`

export const ImageListItem = (props: ThumbnailExtendedProps) => {
    const store = useWorkspaceStore();

    const selected = store.isSelected(props.id);

    const fileNameContent = props.thumbnailLocation.split('/');
    const subtitle = fileNameContent[fileNameContent.length - 1];

    const onSelectClicked = () => {
        if (selected) {
            store.removeImageFromSelected(props.id);
        } else {
            store.addImageToSelected(props.id);
            store.setSelecting(true);
        }
    };

    return <Fragment>
        <ListItem>
            {
                props.isImage &&
                <ListItemIcon>
                    <Checkbox
                        edge="start"
                        inputProps={{ 'aria-label': 'Select Image'}}
                        checked={ selected }
                        onChange={ onSelectClicked } />
                </ListItemIcon>
            }

            <ListItemButton onClick={ props.onClick }>
                <ListItemAvatar>
                    <StyledAvatar alt={ props.name } src={ props.thumbnailLocation } />
                </ListItemAvatar>

                <ImageText>
                    <Typography variant="h6">
                        { props.name }
                    </Typography>
                </ImageText>

                <ImageText>
                    <Typography variant="subtitle1">
                        { subtitle }
                    </Typography>
                </ImageText>
            </ListItemButton>
            
            <ActionBox>
                {
                    props.isImage &&
                    <ButtonLeftMargin
                        variant="outlined"
                        startIcon={ <DriveFileMove /> }>
                        Move
                    </ButtonLeftMargin>
                }

                <ButtonLeftMargin
                    variant="contained"
                    color="error"
                    startIcon={ <Delete /> }>
                    Delete
                </ButtonLeftMargin>
            </ActionBox>
        </ListItem>

        <Divider />
    </Fragment>;
}

export default ImageListItem;

