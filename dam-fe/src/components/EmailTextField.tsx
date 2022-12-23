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
    onChange?: ChangeEventHandler<HTMLInputElement>,
}

const EMAIL_PATTERN_ERROR = 'Not a valid email!';

const EmailTextField = (props: EmailTextFieldProps) => {
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
        }
    }, [ props.value ]);

    return <StyledTextField
        label={ props.label ? props.label : 'Email' }
        value={ props.value ? props.value : '' }
        onChange={ onChange }
        required={ props.required ? props.required : false }
        helperText={ error ? EMAIL_PATTERN_ERROR : ''} />;
}

export default EmailTextField;

