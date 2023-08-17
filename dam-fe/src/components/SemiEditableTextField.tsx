import {
    useEffect, useMemo, useRef, useState, useTransition, Fragment, ChangeEvent,
    KeyboardEvent,
} from 'react';
import {
    TextField as MuiTextField, Typography, Grid, IconButton, OutlinedInput,
    InputAdornment, FormControl, InputLabel, CircularProgress, Box,
} from '@mui/material';

import { useTheme } from '@mui/material'

import { Edit, Delete, Check, Close, Visibility } from '@mui/icons-material';

import {
    Loading, Breadcrumbs, Error, ImagePreview,
} from '.';

import { generateId } from '../utils/misc';

interface SemiEditableTextFieldProps {
    label: string,
    value?: string,
    onSave?: (updatedVal: string) => void,
    updating?: boolean,
    onEdited?: (val: boolean) => void,
    onChange?: (e: ChangeEvent<HTMLInputElement>) => void,
}

export const SemiEditableTextField = (props: SemiEditableTextFieldProps) => {
    const [ edit, setEdit ] = useState<boolean>(false);
    const [ edited, setEdited ] = useState<boolean>(false);
    const [ showErrPopup, setShowErrPopup ] = useState<boolean>(false);
    const [ editedValue, setEditedValue ] = useState<string>('');
    const [ errPopupText, setErrPopupText ] = useState<string>('Error!');

    const [ _, startTransition ] = useTransition();

    const valueInputRef = useRef();
    const theme = useTheme();

    const id = useMemo(() => generateId(), []);

    const onEdit = () => startTransition(() => {
        setEdit(true);
        setShowErrPopup(false);
    });

    const onEditCancel = () => startTransition(() => {
        setEdit(false);
        setEdited(false);

        if (props.onEdited) {
            props.onEdited(false);
        }
    });

    const onEditSave = async () => {
        startTransition(() => setEdit(false));

        if (props.onSave) {
            props.onSave(editedValue);
        }

        if (props.onEdited) {
            props.onEdited(true);
        }
    };

    const onValueChanged = (e: ChangeEvent<HTMLInputElement>) => {
        if (edit) {
            if (!edited) {
                setEdited(true);
            }

            if (props.onChange) {
                props.onChange(e);
            } else {
                setEditedValue(e.target.value);
            }
        }
    };

    const onValueKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Enter') {
            onEditSave();
        }
    };

    useEffect(() => {
        if (valueInputRef  && valueInputRef.current) {
            const wrapper = valueInputRef.current as HTMLDivElement;
            const inputEl = wrapper.querySelector('input') as HTMLInputElement;

            if (inputEl) {
                inputEl.select();
            }
        }
    }, [ edit ]);

    return <FormControl sx={{ width: '100%', marginTop: '0.5rem' }}>
        <InputLabel htmlFor={ id }> { props.label } </InputLabel>

        <OutlinedInput
            id={ id }
            value={ edited ? editedValue : props.value }
            disabled={ !edit }
            label={ props.label }
            onChange={ onValueChanged }
            onKeyDown={ onValueKeyDown }
            ref={ valueInputRef }
            sx={{
                background: edited ?
                    'linear-gradient(to right, ' + theme.palette.primary.main + '00 75%, '
                    + theme.palette.primary.main + '55)' : 'none'
            }}
            endAdornment={
                <InputAdornment position="end">
                {
                    props.updating ?
                        <CircularProgress size={ 32 } />
                    :
                        edit ?
                        <Fragment>
                            <IconButton onClick={ onEditSave }>
                                <Check />
                            </IconButton>

                            <IconButton onClick={ onEditCancel }>
                                <Close />
                            </IconButton>
                        </Fragment>
                        :
                        <IconButton
                            onClick={ onEdit }
                            color={ edited? 'default' : 'secondary' }>
                            <Edit />
                        </IconButton>
                }
                </InputAdornment>
            } />

        <Error on={ showErrPopup }>{ errPopupText }</Error>
    </FormControl>;
};

export default SemiEditableTextField;

