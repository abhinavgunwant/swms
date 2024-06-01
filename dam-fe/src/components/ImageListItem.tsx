import { Fragment } from 'react';

import {
    ListItem, ListItemIcon, ListItemButton, ListItemAvatar, Avatar, Checkbox,
    Divider, Box, Button, ListItemText, Typography, ClickAwayListener, Tooltip
} from '@mui/material';

import { Delete } from '@mui/icons-material';

import ThumbnailExtendedProps from '../models/ThumbnailExtendProps';
import useWorkspaceStore from '../store/workspace/WorkspaceStore';
import { ThumbnailAction } from '../models/ThumbnailExtendProps';

import { styled } from '@mui/material/styles';

const StyledAvatar = styled(Avatar)`
    border-radius: 0;
`;

const ActionBox = styled(Box)`
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 1rem;
`;

const ImageText = styled(ListItemText)`
    width: 400px;
`

export const ImageListItem = (props: ThumbnailExtendedProps) => {
    const store = useWorkspaceStore();

    const selected = store.isSelected(props.id);

    const getSubtitle = () => {
        if (props.thumbnailLocation) {
            const fileNameContent = props.thumbnailLocation.split('/');

            if (fileNameContent.length) {
                return fileNameContent[fileNameContent.length - 1];
            }
        }

        return '';
    };

    const onSelectClicked = () => {
        if (selected) {
            store.removeImageFromSelected(props.id);
        } else {
            store.addImageToSelected(props.id);
            store.setSelecting(true);
        }
    };

    const onClickAway = (action: ThumbnailAction) => {
        if (action && action.onHideTooltip) {
            action.onHideTooltip();
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
                        { getSubtitle() }
                    </Typography>
                </ImageText>
            </ListItemButton>
            
            <ActionBox>
                {
                    props?.actions?.map((action, i) => {
                        console.log('action:', action);
                        if (action && action.text && action.show) {
                            if (action.tooltip) {
                                return <ClickAwayListener
                                    onClickAway={ () => onClickAway(action) }>
                                    <Tooltip
                                        title={ action.tooltip }
                                        open={ action.showTooltip }
                                        onClose={ action.onHideTooltip }
                                        placement="top"
                                        disableFocusListener
                                        disableHoverListener>
                                        <Button
                                            variant="outlined"
                                            color="primary"
                                            startIcon={ action?.icon }
                                            aria-label={ action.label }
                                            onClick={ (e) => action?.action(e) }
                                            key={ i }>
                                            { action?.text || '' }
                                        </Button>
                                    </Tooltip>
                                </ClickAwayListener>;
                            }

                            return <Button
                                variant="outlined"
                                color="primary"
                                startIcon={ action?.icon }
                                onClick={ (e) => action?.action(e) }
                                key={ i }>
                                { action?.text || '' }
                            </Button>;
                        }

                        return 'test';
                    })
                }
                <Button
                    variant="contained"
                    color="error"
                    startIcon={ <Delete /> }>
                    Delete
                </Button>
            </ActionBox>
        </ListItem>

        <Divider />
    </Fragment>;
}

export default ImageListItem;

