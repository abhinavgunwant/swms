import { Fragment, useState, useRef, useTransition } from 'react';

import Fab from '@mui/material/Fab';
import AddIcon from '@mui/icons-material/Add';
import ImageIcon from '@mui/icons-material/Image';
import FolderIcon from '@mui/icons-material/Folder';
import SelectAllIcon from '@mui/icons-material/SelectAll';
import DeselectIcon from '@mui/icons-material/Deselect';
import DriveFileMoveIcon from '@mui/icons-material/DriveFileMove';
import DeleteIcon from '@mui/icons-material/Delete';
import { styled as muiStyled } from '@mui/material/styles';

import useWorkspaceStore from '../../store/workspace/WorkspaceStore';

import styled from '@emotion/styled';
import { css } from '@emotion/react';
import { useNavigate } from 'react-router-dom';

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

const AddFabMenu = styled.div`
    ${ commonStyle }
    background: linear-gradient(to bottom right, #00000033, #00000000);
    padding: 1rem 1rem 3.5rem 1rem;
    border-radius: 1.5rem;
    position: relative;
    bottom: -3rem;
    z-index: 1050;
`;

const ImageFab = muiStyled(Fab)`margin-bottom: 0.5rem`;

const StyledFab = muiStyled(Fab)`margin-left: 1rem`;

const FabText = styled.div`
    margin-left: 0.5rem;
`;

const WorkspaceFab = () => {
    const store = useWorkspaceStore();
    const navigate = useNavigate();

    const [ addExpanded, setAddExpanded ] = useState(false);

    const [ pending, startTransition ] = useTransition();

    const addFabRef: React.MutableRefObject<HTMLElement|undefined> = useRef();
    const addFabMenuRef: React.MutableRefObject<HTMLElement|undefined> = useRef();

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

    const onAddClicked = () => {
        startTransition(() => {
            setAddExpanded(!addExpanded);
        });

        if (addExpanded) {
            document.removeEventListener('click', onOutsideClicked);
        } else {
            document.addEventListener('click', onOutsideClicked);
        }
    }

    const onNewImageClicked = () => {
        startTransition(() => {
            navigate('/workspace/new-image');
        })
    }

    const onNewFolderClicked = () => {
        startTransition(() => {
            navigate('/workspace/new-folder');
        })
    }

    const onOutsideClicked = (e: any) => {
        if (
            !addFabRef.current?.contains(e?.target)
            && !addFabMenuRef.current?.contains(e?.target)
            ) {
            startTransition(() => {
                setAddExpanded(false);
            });
            document.removeEventListener('click', onOutsideClicked);
        }
    };
    
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
                    <StyledFab variant="extended" onClick={ onDeselectAllClicked }>
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
                    {
                        addExpanded &&
                        <AddFabMenu ref={ (r) => {
                            if (r) {
                                addFabMenuRef.current = r
                            }}}>
                            <ImageFab
                                color="secondary"
                                variant="extended"
                                onClick={ onNewImageClicked }>
                                <ImageIcon />
                                <FabText>Image</FabText>
                            </ImageFab>

                            <Fab
                                color="secondary"
                                variant="extended"
                                onClick={ onNewFolderClicked }>
                                <FolderIcon />
                                <FabText>Folder</FabText>
                            </Fab>
                        </AddFabMenu>
                    }
                    <Fab
                        color={ addExpanded ? 'default' : 'secondary'}
                        variant="extended"
                        onClick={onAddClicked}
                        ref={ (r) => {
                            if (r)  {
                                addFabRef.current = r;
                            }
                        }}>
                        <AddIcon />
                        <FabText>New</FabText>
                    </Fab>
                </AddFabWrapper>
        }
    </WorkspaceFabWrapper>
}

export default WorkspaceFab;
