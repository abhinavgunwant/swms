import { Typography } from '@mui/material';

import { CustomDialog } from '../';
import { Dialog } from '../../models';

interface WIPDialogProps extends Dialog { optText?: string }

/**
 * "This feature is under development" dialog.
 */
export const WIPDialog = (props: WIPDialogProps) =>
<CustomDialog
    open={ props.open }
    onClose={ props.onClose }
    title="This feature is under development"
    body={ props.optText ? <Typography>{ props.optText }</Typography> : '' }
    actions={ [{
        buttonVariant: 'contained',
        text: 'OK',
        action: props.onClose,
    }] }/>;

export default WIPDialog;

