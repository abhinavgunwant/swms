import {
    useEffect, useMemo, useRef, useState, useTransition, Fragment, ChangeEvent,
    KeyboardEvent,
} from 'react';
import {
    IconButton, OutlinedInput, InputAdornment, FormControl, InputLabel,
    CircularProgress,
} from '@mui/material';

import { useTheme } from '@mui/material'

import { Edit, Check, Close } from '@mui/icons-material';

import { Error } from '.';

import { generateId } from '../utils/misc';

interface SemiEditableTextFieldProps {
    label: string,
    value?: string,
    sx?: any,
    onSave?: (updatedVal: string) => void,
    updating?: boolean,
    showErrPopup?: boolean,
    errPopupText?: string,
    onEdited?: (val: boolean) => void,
}

export const SemiEditableTextField = (props: SemiEditableTextFieldProps) => {
    const [ edit, setEdit ] = useState<boolean>(false);
    const [ edited, setEdited ] = useState<boolean>(false);
    const [ editedValue, setEditedValue ] = useState<string>('');

    const [ _, startTransition ] = useTransition();

    const valueInputRef = useRef();
    const theme = useTheme();

    const id = useMemo(() => generateId(), []);

    const onEdit = () => startTransition(() =>  setEdit(true));

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

            setEditedValue(e.target.value);
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

    useEffect(() => {
        if (props.value === editedValue) {
            startTransition(() => setEdited(false));
        }
    }, [ props.value ]);

    return <FormControl sx={ props.sx || { width: '100%', marginTop: '0.5rem' }}>
        <InputLabel htmlFor={ id }> { props.label } </InputLabel>

        <OutlinedInput
            id={ id }
            data-testid="setf-input"
            value={ edited ? editedValue : props.value }
            disabled={ !edit }
            label={ props.label }
            onChange={ onValueChanged }
            onKeyDown={ onValueKeyDown }
            ref={ valueInputRef }
            sx={{
                background: edited ?
                    'linear-gradient(to right, ' + theme.palette.primary.main
                    + '00 75%, ' + theme.palette.primary.main + '55)' : 'none'
            }}
            endAdornment={
                <InputAdornment position="end">
                {
                    props.updating ?
                        <CircularProgress size={ 32 } />
                    :
                        edit ?
                        <Fragment>
                            <IconButton
                                data-testid="setf-save-button"
                                onClick={ onEditSave }>
                                <Check />
                            </IconButton>

                            <IconButton
                                data-testid="setf-cancel-button"
                                onClick={ onEditCancel }>
                                <Close />
                            </IconButton>
                        </Fragment>
                        :
                        <IconButton
                            data-testid="setf-edit-button"
                            onClick={ onEdit }
                            color={ edited? 'default' : 'secondary' }>
                            <Edit />
                        </IconButton>
                }
                </InputAdornment>
            } />

        <Error on={ props.showErrPopup || false }>
            { props.errPopupText }
        </Error>
    </FormControl>;
};

export default SemiEditableTextField;

