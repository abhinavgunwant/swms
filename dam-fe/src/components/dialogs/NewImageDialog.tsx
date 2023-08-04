import { useTransition } from 'react';
import { useNavigate } from 'react-router-dom';

import {
    List, Dialog, DialogTitle, DialogContent, ListItem, ListItemText,
    ListItemIcon,
} from '@mui/material';

import { Folder, Image } from '@mui/icons-material';

interface NewImageDialogProps {
    open: boolean,
    onClose: () => void,
}

/**
 * Dialog that appears when user clicks "+ NEW" button on workspace.
 *
 * TODO: Update the component name to `NewItemDialog` or something similar.
 */
export const NewImageDialog = (props: NewImageDialogProps) => {
    /* eslint-disable @typescript-eslint/no-unused-vars */
    const [ _, startTransition ] = useTransition();
    const navigate = useNavigate();

    const onNewDialogClosed = () => {
        props.onClose();
    };

    const onNewImageClicked = () => 
        startTransition(() => navigate('/workspace/new-image'));

    const onNewFolderClicked = () =>
        startTransition(() => navigate('/workspace/new-folder'));

    return <Dialog onClose={ onNewDialogClosed } open={ props.open }>
        <DialogTitle>New</DialogTitle>

        <DialogContent>
            <List sx={{ width: 240 }}>
                <ListItem button onClick={ onNewImageClicked }>
                    <ListItemIcon><Image /></ListItemIcon>
                    <ListItemText>Image</ListItemText>
                </ListItem>

                <ListItem button onClick={ onNewFolderClicked }>
                    <ListItemIcon><Folder /></ListItemIcon>
                    <ListItemText>Folder</ListItemText>
                </ListItem>
            </List>
        </DialogContent>
    </Dialog>
};

export default NewImageDialog;

