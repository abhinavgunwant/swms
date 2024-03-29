import { useEffect, useState, useTransition } from 'react';

import { useNavigate } from 'react-router-dom';

import { CircularProgress, DialogTitle, Typography } from '@mui/material';

import CustomDialog from '../CustomDialog';

import useAPI from '../../hooks/useAPI';

import { styled } from '@mui/material/styles';

interface DeleteItemDialogProps {
    open: boolean,
    onClose: (success: boolean) => void,
    onSuccess?: () => void,
    onFailure?: () => void,
    imageIDs?: Array<number>,
    folderIDs?: Array<number>,
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
export const DeleteItemDialog = (props: DeleteItemDialogProps) => {
    const [ error, setError ] = useState<boolean>(false);
    const [ deleting, setDeleting ] = useState<boolean>(false);

    const [ _, startTransition ] = useTransition();

    const navigate = useNavigate();
    const { deleteImages, deleteFolders } = useAPI();

    let validImgArray: boolean = Array.isArray(props.imageIDs)
        && props.imageIDs.length > 0;

    let validFolArray: boolean = Array.isArray(props.folderIDs)
        && props.folderIDs.length > 0;

    const multiItems = (validImgArray && validFolArray)
        || (validImgArray && props.imageIDs && props.imageIDs.length > 1)
        || (validFolArray && props.folderIDs && props.folderIDs.length > 1);

    const onYes: () => void = async () => {
        startTransition(() => setDeleting(true));

        if (!(validImgArray || validFolArray)) {
            startTransition(() => setError(true));
            return;
        }

        if (
            validImgArray && validFolArray && props.imageIDs
            && props.folderIDs
        ) {
            startTransition(() => setDeleting(true));

            console.log('Deleting images and folders');

            Promise.all([
                deleteImages(props.imageIDs),
                deleteFolders(props.folderIDs)
            ]).then(([ respImg, respFol ]) => {
                if (respImg.success && respFol.success) {
                    props.onClose(true);

                    if (props.navigateToAfterSuccess) {
                        navigate(props.navigateToAfterSuccess);
                    }

                    startTransition(() => {
                        setError(false);
                        setDeleting(false);
                    });
                }
            });

            return;
        }

        if (validImgArray && props.imageIDs) {
            startTransition(() => setDeleting(true));
            console.log('Deleting images');

            const resp = await deleteImages(props.imageIDs);

            if (resp.success) {
                props.onClose(true);

                if (props.navigateToAfterSuccess) {
                    navigate(props.navigateToAfterSuccess);
                }

                startTransition(() => {
                    setError(false);
                    setDeleting(false);
                });
            }

            return;
        }

        // For `validFolArray`

        startTransition(() => setDeleting(true));
        console.log('Deleting folders');

        if (props.folderIDs) {
            const resp = await deleteFolders(props.folderIDs);

            if (resp.success) {
                props.onClose(true);

                if (props.navigateToAfterSuccess) {
                    navigate(props.navigateToAfterSuccess);
                }

                startTransition(() => {
                    setError(false);
                    setDeleting(false);
                });
            }
        }
    };

    const onNo = () => props.onClose(false);

    useEffect(() => {
        if (props.open && error) {
            setError(false);
        }

        if (!props.open) {
            startTransition(() => setDeleting(false));
        }
    }, [ props.open ]);

    return <CustomDialog
        open={ props.open && (validImgArray || validFolArray) }
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
                        Some error occurred while deleting item{
                        multiItems && 's' }.
                        Item{ multiItems && 's' } might not have been deleted.
                        <br />
                        Please try again later.
                    </Typography>
                :
                    <Typography>
                        Deleting this will delete all
                        { validFolArray && ' images, ' } renditions and
                        files associated with
                        { multiItems ? ' selected' : ' this' } item{
                        multiItems && 's' }.
                        <br />
                        Do you want to continue?
                    </Typography>
        }
        actions={
            deleting ? [] :
                error ?
                    [{
                        text: 'Close',
                        action: () => { props.onClose(true); },
                        buttonColor: 'error'
                    }]
                :
                    [{
                        text: 'Yes, delete ' + (multiItems ? 'them!' : 'it!'),
                        action: onYes,
                        buttonColor: 'error',
                        buttonVariant: 'contained',
                    }, {
                        text: 'No', action: onNo
                    }]
        }
        onClose={ () => { props.onClose(false) } }
    />
}

export default DeleteItemDialog;

