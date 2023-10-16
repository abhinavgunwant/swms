import {
    Fragment, useEffect, useState, useTransition, ChangeEventHandler,
    ChangeEvent, useRef,
} from 'react';

import {
    CircularProgress, Dialog, DialogContent, DialogTitle, Typography, Alert,
    TextField, Button, AlertTitle, Box,
} from '@mui/material';

import useAPI from '../../hooks/useAPI';

import { Project } from '../../models';

import { styled } from '@mui/material/styles';
import { default as emoStyled } from '@emotion/styled';

interface DeleteProjectDialogProps {
    open: boolean,
    project: Project | null,
    onClose: () => void,
    onSuccess?: () => void,
    onFailure?: () => void,
}

const DeleteAlert = styled(Alert)`margin-bottom: 1rem`;
const DeleteTextField = styled(TextField)`
    margin-bottom: 1rem;
    width: 100%
`;

const DeleteText = styled(Typography)`
    display: flex;
    justify-content: center;
    align-items: center;

    width: 300px;
`;

const ActionBox = styled(Box)`
    display: flex;
    justify-content: flex-end;
    align-items: center;

    width: 100%;
`;

const Pre = emoStyled.pre`
    background-color: #eeeeee;
    color: #aa0000;
    padding: 1rem;
`;

/**
 * Implements the action when user clicks on the 'Delete' button on an
 * image thumbnail or on the image details view.
 */
export const DeleteProjectDialog = (props: DeleteProjectDialogProps) => {
    const [ error, setError ] = useState<boolean>(false);
    const [ errorMessage, setErrorMessage ] = useState<string>('');
    const [ deleting, setDeleting ] = useState<boolean>(false);
    const [ projectText, setProjectText ] = useState<string>('');

    const [ _, startTransition ] = useTransition();

    const textFieldRef = useRef<HTMLInputElement | null>(null);

    const { deleteProject } = useAPI();

    const onProjectTextChanged: ChangeEventHandler =
        (e: ChangeEvent<HTMLInputElement>) => {
            setProjectText(e.target.value);
        };

    const onDelete = async () => {
        if (props.project === null) { return; }

        startTransition(() => setDeleting(true));

        const resp = await deleteProject(props.project.id);

        if (resp === true) {
            if (props.onSuccess) {
                props.onSuccess();
            }

            onClose();
            return;
        }

        startTransition(() => {
            setDeleting(false);
            setError(true);
            setErrorMessage(resp);
        });
    };

    const onClose = () => {
        props.onClose();

        startTransition(() => {
            setError(false);
            setErrorMessage('');
            setDeleting(false);
            setProjectText('');
        });
    }

    useEffect(() => {
        setTimeout(() => {
            if (props.open && textFieldRef.current) {
                const input = textFieldRef.current.querySelector('input');

                if (input) {
                    input.focus();
                }
            }
        }, 100);
    }, [ props.open ]);

    return <Dialog
        open={ props.open && props.project !== null }
        onClose={ onClose }>

        <DialogTitle color="error">
            { deleting ? 'Deleting' : 'Delete Project: Confirm your action' }
        </DialogTitle>

        <DialogContent>
            {
                deleting ? <DeleteText><CircularProgress /></DeleteText>
                :
                error ?
                    <Alert severity="error">
                        {
                            errorMessage ? errorMessage
                            : <Fragment>
                                Some error occurred while deleting this project
                                <br />
                                Please try again later.
                            </Fragment>
                        }
                    </Alert>
                :
                    <Fragment>
                        <Typography>
                            <DeleteAlert severity="warning">
                                <AlertTitle>Warning</AlertTitle>
                                Deleting this project will delete all images,
                                renditions and files associated with this
                                project.
                            </DeleteAlert>

                            To continue deleting, type the following string
                            into the text box:
                            
                            <Pre>{ props.project && props.project.slug }</Pre>
                        </Typography>

                        <DeleteTextField
                            ref={ textFieldRef }
                            placeholder="Enter the project slug here"
                            variant="standard"
                            value={ projectText }
                            onChange={ onProjectTextChanged } />

                        <ActionBox>
                            <Button
                                variant="outlined"
                                sx={{ marginRight: '0.5rem' }}
                                onClick={ onClose }>
                                Cancel
                            </Button>

                            <Button
                                variant="contained"
                                color="error"
                                disabled={
                                    !(props.project
                                    && projectText === props.project.slug)
                                }
                                onClick={ onDelete }>
                                DELETE
                            </Button>
                        </ActionBox>
                    </Fragment>
            }
        </DialogContent>
    </Dialog>;
}

export default DeleteProjectDialog;

