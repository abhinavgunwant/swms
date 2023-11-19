import { CustomDialog, Error } from '../';
import { Dialog } from '../../models';

interface ErrorDialogProps extends Dialog { text: string }

export const ErrorDialog = (props: ErrorDialogProps) => 
    <CustomDialog
        open={ props.open }
        onClose={ props.onClose }
        title="Error"
        body={ <Error on={ true }>{ props.text }</Error> }
        actions={ [{
            buttonVariant: 'contained',
            text: 'OK',
            action: props.onClose,
        }] }/>;

export default ErrorDialog;

