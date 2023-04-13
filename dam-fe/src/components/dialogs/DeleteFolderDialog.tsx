import { useEffect, useState, useTransition } from 'react';

import { useNavigate } from 'react-router-dom';

import { CircularProgress, DialogTitle, Typography } from '@mui/material';

import CustomDialog from '../CustomDialog';

import useAPI from '../../hooks/useAPI';

import { styled } from '@mui/material/styles';

interface DeleteImageDialogProps {
    open: boolean,
    onClose: () => void,
    folderId: number,
    navigateToAfterSuccess?: string,
}

const DeleteText = styled(Typography)`
    display: flex;
    justify-content: center;
    align-items: center;

    width: 300px;
`;

/**
 * Implements the action when user clicks on the 'Delete' button on an
 * image thumbnail or on the image details view.
 */
export const DeleteFolderDialog = (props: DeleteImageDialogProps) => {
    const [ error, setError ] = useState<boolean>(false);
    const [ deleting, setDeleting ] = useState<boolean>(false);

    const [ _, startTransition ] = useTransition();

    const navigate = useNavigate();
    const { deleteFolder } = useAPI();

    const onYes = async () => {
        startTransition(() => setDeleting(true));
        const resp = await deleteFolder(props.folderId);

        console.log(resp);

        if (resp.success) {
            startTransition(() => setDeleting(true));

            if (props.navigateToAfterSuccess) {
                navigate(props.navigateToAfterSuccess);
            }

            props.onClose();
            return;
        }

        startTransition(() => {
            setError(true);
            setDeleting(false);
        });
    };

    const onNo = () => props.onClose();

    useEffect(() => {
        if (props.open && error) {
            setError(false);
        }
    }, [ props.open ]);

    return <CustomDialog
        open={ props.open }
        title={
            error ? 
                <DialogTitle color="error">Error while deleting</DialogTitle>
                : 'Confirm Delete'
        }
        body={
            deleting ?
                <DeleteText>
                    <span style={{ paddingRight: '1rem' }}>
                        <CircularProgress />
                    </span>
                    Deleting
                </DeleteText>
            :
                error ?
                    <Typography color="error">
                        Some error occurred while deleting image...
                        Please try again later.
                    </Typography>
                :
                    <Typography>
                        Deleting this folder will delete all it&apos;s content
                        including all images, their renditions and files
                        associated with them.
                        <br /><br />
                        Do you want to continue?
                    </Typography>
        }
        actions={
            deleting ? [] :
                error ?
                    [{
                        text: 'Close',
                        action: props.onClose,
                        buttonColor: 'error'
                    }]
                :
                    [{
                        text: 'Yes, delete it!',
                        action: onYes,
                        buttonColor: 'error',
                        buttonVariant: 'contained',
                    },
                    {
                        text: 'No',
                        action: onNo,
                    }]
        }
        onClose={ props.onClose }
    />
}

export default DeleteFolderDialog;

