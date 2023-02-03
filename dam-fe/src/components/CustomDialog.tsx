import { ReactNode } from 'react';

import styled from '@emotion/styled';

import {
    Button, Table, TableHead, TableRow, TableCell, TableBody,
    TableContainer, TableSortLabel, Checkbox, TablePagination, Dialog,
    DialogTitle, DialogContent,
} from '@mui/material';

/**
 * Interface representing the action buttons of the dialog.
 */
interface CustomDialogAction {
    /**
     * Text of the button
     */
    text: string,

    /**
     * Function to be executed when clicked.
     */
    action: () => void,

    /**
     * Which variant of the mui `<Button>` component to render.
     */
    buttonVariant?: 'text' | 'outlined' | 'contained',

    buttonColor?: 'primary' | 'secondary' | 'success' | 'error',
}

/**
 * Interface representing all the content in the confirm dialog.
 */
interface CustomDialogProps {
    /**
     * The text in the top title line.
     */
    title: string,

    /**
     * The "Body" part of the component.
     * Can contain any react component.
     */
    body?: ReactNode,

    actions?: CustomDialogAction[],

    /**
     * When to open the dialog.
     */
    open: boolean,

    /**
     * Function to be called when dialog is closed.
     */
    onClose: () => void,
}

const DialogActions = styled.div`
    display: flex;
    justify-content: flex-end;
    align-items: center;

    margin-top: 1rem;
    gap: 0.5rem;
`;

export const CustomDialog = (props: CustomDialogProps) => {
    if (!props.open) {
        return null;
    }

    return <Dialog open={ true } onClose={ props.onClose }>
        <DialogTitle>{ props.title }</DialogTitle>

        <DialogContent>
            { props.body }

            <DialogActions>
            {
                props?.actions?.map((action, i) => <Button
                    key={ i }
                    onClick={ action.action }
                    variant={ action.buttonVariant || 'outlined' }
                    color={ action.buttonColor || 'primary' }>
                        { action.text }
                    </Button>
                )
            }
            </DialogActions>
        </DialogContent>
    </Dialog>
};

export default CustomDialog;
