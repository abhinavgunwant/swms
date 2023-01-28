import {
    useState, useEffect, useTransition, ChangeEventHandler, ChangeEvent
} from 'react';

import TextField from '@mui/material/TextField';

import { styled } from '@mui/material/styles';

const StyledTextField = styled(TextField)`
    width: 100%;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
`;

interface EmailTextFieldProps {
    label?: string,
    value?: string,
    required?: boolean,
    forceShowError?: boolean,
    onChange?: ChangeEventHandler<HTMLInputElement>,
    onError?: Function,
    onValid?: Function,
}

const EMAIL_PATTERN_ERROR = 'Must be a valid email';

export const EmailTextField = (props: EmailTextFieldProps) => {
    const [ error, setError ] = useState<boolean>(false);

    // eslint-disable-next-line @typescript-eslint/no-unused-vars 
    const [ _, startTransition ] = useTransition();

    const emailPattern = /^[\w\-\.]+@([\w\-]+\.)+[\w\-]{2,4}$/ig;

    const onChange = (e: ChangeEvent<HTMLInputElement>) => {
        if (props.onChange) {
            props.onChange(e);
        }
    }

    useEffect(() => {
        if (!props.value || typeof props.value === 'undefined') {
            return;
        }

        const valid = emailPattern.test(props.value);

        if (!valid !== error) {
            startTransition(() => setError(!valid));

            if (!valid) {
                if (typeof props.onError !== 'undefined') {
                    props.onError();
                }
            } else {
                if (typeof props.onValid !== 'undefined') {
                    props.onValid();
                }
            }
        }
    }, [ props.value ]);

    return <StyledTextField
        error={ error || props.forceShowError }
        label={ props.label ? props.label : 'Email' }
        value={ props.value ? props.value : '' }
        onChange={ onChange }
        required={ props.required ? props.required : false }
        helperText={
            error || props.forceShowError ? EMAIL_PATTERN_ERROR : ''
        } />;
}

export default EmailTextField;

