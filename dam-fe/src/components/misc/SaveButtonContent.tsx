import { CircularProgress } from '@mui/material';
import { Fragment } from 'react';

interface SaveButtonContentProps { saving: boolean }

export const SaveButtonContent = (props: SaveButtonContentProps) => {
    if (props.saving) {
        return <Fragment>
            <CircularProgress
                size={ 16 }
                color="secondary"
                sx={{ color: '#ffffff', marginRight: '1rem' }} />
            Saving
        </Fragment>;
    }

    return <Fragment>Save</Fragment>;
}


export default SaveButtonContent;

