import { Fragment, useState, useTransition } from 'react';
import { useNavigate } from 'react-router-dom';

import {
    Fab, Dialog, DialogTitle, List, ListItem, ListItemIcon, ListItemText,
    DialogContent
} from '@mui/material';

import {
    Add as AddIcon, Image as ImageIcon, Folder as FolderIcon,
    SelectAll as SelectAllIcon, Deselect as DeselectIcon, Delete as DeleteIcon,
    DriveFileMove as DriveFileMoveIcon
} from '@mui/icons-material';

import useWorkspaceStore from '../../store/workspace/WorkspaceStore';

import styled from '@emotion/styled';
import { css } from '@emotion/react';
import { styled as muiStyled } from '@mui/material/styles';

const commonStyle = css`
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    align-items: flex-end;
`;

const WorkspaceFabWrapper = styled.div`
    position: fixed;
    bottom: 1rem;
    right: 1rem;
    display: flex;
    justify-content: flex-end;
    align-items: flex-end;
`;

const AddFabWrapper = styled.div`${ commonStyle }margin-left: 1rem;`;

const StyledFab = muiStyled(Fab)`margin-left: 1rem`;

const FabText = styled.div`
    margin-left: 0.5rem;
`;

const WorkspaceFab = (props: { inWorkspaceHome?: boolean } | undefined) => {
    const store = useWorkspaceStore();

    const navigate = useNavigate();

    const [ openNewDialog, setOpenNewDialog ] = useState(false);

    const [ _pending, startTransition ] = useTransition();

    const onDeselectAllClicked = () => {
        startTransition(() => {
            store.setSelecting(false);
            store.selectedImages.forEach(
                img => store.removeImageFromSelected(img)
            );
        })
    }

    const onSelectAllClicked = () => {
        startTransition(() => {
            store.setSelecting(true);
            store.imageList.forEach(
                img => store.addImageToSelected(img.id)
            );
        })
    }

    const onNewDialogClosed = () => setOpenNewDialog(false);

    const onNewClicked = () => {
        if (props?.inWorkspaceHome) {
            startTransition(() => navigate('/workspace/new-project'));
            return;
        }

        startTransition(() => setOpenNewDialog(!openNewDialog));
    }

    const onNewImageClicked = () => 
        startTransition(() => navigate('/workspace/new-image'));

    const onNewFolderClicked = () =>
        startTransition(() => navigate('/workspace/new-folder'));

    return <WorkspaceFabWrapper>
        {
            store.imageList.length !== store.selectedImages.size &&
            <StyledFab variant="extended" onClick={ onSelectAllClicked }>
                <SelectAllIcon />
                <FabText>Select All</FabText>
            </StyledFab>
        }

        {
            store.selecting ?
                <Fragment>
                    <StyledFab
                        variant="extended"
                        onClick={ onDeselectAllClicked }>
                        <DeselectIcon />
                        <FabText>Deselect All</FabText>
                    </StyledFab>
                    <StyledFab variant="extended">
                        <DriveFileMoveIcon />
                        <FabText>Move</FabText>
                    </StyledFab>
                    <StyledFab variant="extended" color="error">
                        <DeleteIcon />
                        <FabText>Delete</FabText>
                    </StyledFab>
                </Fragment>
            :
                <AddFabWrapper>
                    <Fab
                        color="secondary"
                        variant="extended"
                        onClick={ onNewClicked }>
                        <AddIcon />
                        <FabText>New</FabText>
                    </Fab>
                </AddFabWrapper>
        }

        <Dialog onClose={ onNewDialogClosed } open={ openNewDialog }>
            <DialogTitle>New</DialogTitle>

            <DialogContent>
                <List sx={{ width: 240 }}>
                    <ListItem button onClick={ onNewImageClicked }>
                        <ListItemIcon><ImageIcon /></ListItemIcon>
                        <ListItemText>Image</ListItemText>
                    </ListItem>

                    <ListItem button onClick={ onNewFolderClicked }>
                        <ListItemIcon><FolderIcon /></ListItemIcon>
                        <ListItemText>Folder</ListItemText>
                    </ListItem>
                </List>
            </DialogContent>
        </Dialog>
    </WorkspaceFabWrapper>;
}

export default WorkspaceFab;
