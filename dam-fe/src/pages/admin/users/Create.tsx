import { useState, useEffect, useTransition, Fragment, ChangeEvent } from 'react';

import {
    Typography, Grid, TextField, Button, Box, CircularProgress, Accordion,
    AccordionSummary, AccordionDetails, InputLabel, Select, FormControl,
    OutlinedInput, SelectChangeEvent, Alert,
} from '@mui/material';

import useAPI from '../../../hooks/useAPI';

import {
    ExpandMore,
} from '@mui/icons-material';

import Breadcrumbs from '../../../components/Breadcrumbs';
import EmailTextField from '../../../components/EmailTextField';

import { isEmpty } from '../../../utils/misc';

import { styled } from '@mui/material/styles';

const StyledTextField = styled(TextField)`
    width: 100%;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
`;

const StyledGrid = styled(Grid)`
    margin-top: 1rem;
`;

const ChipWrapper = styled(Box)`
    display: flex;
    flex-wrap: wrap;
    gap: 0.5;
`;

const Create = () => {
    const [ name, setName ] = useState<string>('');
    const [ nameError, setNameError ] = useState<boolean>(false);
    const [ showNameHelper, setShowNameHelper ] = useState<boolean>(false);
    const [ login, setLogin ] = useState<string>('');
    const [ showLoginHelper, setShowLoginHelper ] = useState<boolean>(false);
    const [ loginError, setLoginError ] = useState<boolean>(false);
    const [ email, setEmail ] = useState<string>('');
    const [
        forceShowEmailError, setForceShowEmailError
    ] = useState<boolean>(false);
    const [ emailError, setEmailError ] = useState<boolean>(false);
    const [ password, setPassword ] = useState<string>('');
    const [
        showPasswordHelper, setShowPasswordHelper
    ] = useState<boolean>(false);
    const [ passwordError, setPasswordError ] = useState<boolean>(false);
    const [ confirmPassword, setConfirmPassword ] = useState<string>('');
    const [
        showConfirmPasswordHelper, setShowConfirmPasswordHelper
    ] = useState<boolean>(false);
    const [
        confirmPasswordError, setConfirmPasswordError
    ] = useState<boolean>(false);
    const [ saving, setSaving ] = useState<boolean>(false);
    const [ error, setError ] = useState<boolean>(false);
    const [ errorMessage, setErrorMessage ] = useState<string>('');
    const [ success, setSuccess ] = useState<boolean>(false);
    const [ successMessage, setSuccessMessage ] = useState<string>('');

    // eslint-disable-next-line @typescript-eslint/no-unused-vars 
    const [ _, startTransition ] = useTransition();

    const { createUser } = useAPI();

    const passwordPattern = /^(\w|[!@#\$%^&\*\(\)_])+$/g;

    const onNameChanged = (e: ChangeEvent<HTMLInputElement>) => {
        const newName = e.target.value;
        setName(newName);
        validateName(newName);
    }

    const onNameFocus = () => startTransition(() => setShowNameHelper(true));
    const onNameBlur = () => startTransition(() => setShowNameHelper(false));

    const onLoginChanged = (e: ChangeEvent<HTMLInputElement>) => {
        const newLogin = e.target.value;
        setLogin(newLogin);
        validateLogin(newLogin);
    }

    const onLoginFocus = () => startTransition(() => setShowLoginHelper(true));
    const onLoginBlur = () => startTransition(() => setShowLoginHelper(false));

    const onEmailChanged = (e: ChangeEvent<HTMLInputElement>) => {
        const newEmail = e.target.value;
        setEmail(newEmail);
    }

    const onEmailError = () => startTransition(() => setEmailError(true));
    const onEmailValid = () => startTransition(() => {
        setEmailError(false);
        setForceShowEmailError(false);
    });

    const onPasswordChanged = (e: ChangeEvent<HTMLInputElement>) => {
        const newPassword = e.target.value;
        setPassword(newPassword);
        validatePassword(newPassword);
    }

    const onPasswordFocus = () => startTransition(() => setShowPasswordHelper(true));
    const onPasswordBlur = () => startTransition(() => setShowPasswordHelper(false));

    const onConfirmPasswordChanged = (e: ChangeEvent<HTMLInputElement>) => {
        const newConfirmPassword = e.target.value;
        setConfirmPassword(newConfirmPassword);
        validateConfirmPassword(newConfirmPassword);
    }

    const onConfirmPasswordFocus = () =>
        startTransition(() => setShowConfirmPasswordHelper(true));
    const onConfirmPasswordBlur = () =>
        startTransition(() => setShowConfirmPasswordHelper(false));

    const validateName: (a?: string) => boolean = (newName:string = name) => {
        if (isEmpty(newName) || newName.length < 4 || newName.length > 64) {
            if (!nameError) {
                startTransition(() => setNameError(true));
            }

            return false;
        }

        if (nameError) {
            startTransition(() => setNameError(false));
        }

        return true;
    }

    const validateLogin: (a?: string) => boolean = (newLogin: string = login) => {
        if (isEmpty(newLogin) || newLogin.length < 4 || newLogin.length > 16) {
            if (!loginError) {
                startTransition(() => setLoginError(true));
            }

            return false;
        }

        if (loginError) {
            startTransition(() => setLoginError(false));
        }

        return true;
    }

    const validatePassword: (a?: string) => boolean = (
        newPassword: string = password
    ) => {
        if (
            isEmpty(newPassword)
            || newPassword.length < 8
            || newPassword.length > 32
            || !passwordPattern.test(newPassword)
            || !/.*\d.*/.test(newPassword)
            || !/.*[a-z].*/.test(newPassword)
            || !/.*[A-Z].*/.test(newPassword)
            || !/.*[!@#\$%^&\*\(\)_].*/.test(newPassword)) {
            if (!passwordError) {
                startTransition(() => setPasswordError(true));
            }

            return false;
        }

        if (passwordError) {
            startTransition(() => setPasswordError(false));
        }

        return true;
    }

    const validateConfirmPassword: (a?: string) => boolean = (
        newConfirmPassword: string = confirmPassword
    ) => {
        if (
            newConfirmPassword !== password
            || isEmpty(password)
            || isEmpty(newConfirmPassword)
        ) {
            if (!confirmPasswordError) {
                startTransition(() => setConfirmPasswordError(true));
            }

            return false;
        }

        if (confirmPasswordError) {
            startTransition(() => setConfirmPasswordError(false));
        }

        return true;
    }

    const onSuccessMessageClosed = () => {
        startTransition(() => {
            setSuccess(false);
            setSuccessMessage('');
        });
    }

    const onErrorMessageClosed = () => {
        startTransition(() => {
            setError(false);
            setErrorMessage('');
        });
    }

    /**
     * Does a final validation before calling the create user API.
     */
    const onSave = async () => {
        if (emailError) {
            return;
        }

        // Validate all fields once again!
        let valid: boolean = validateName();
        valid = validateLogin() || valid;
        valid = validatePassword() || valid;
        valid = validateConfirmPassword() || valid;

        if (!emailError && email === '') {
            valid = false;
            startTransition(() => setForceShowEmailError(true));
        }

        if (saving || !valid) {
            return;
        }

        startTransition(() => {
            setSaving(true);
            setSuccess(false);
            setSuccessMessage('');
            setError(false);
            setErrorMessage('');
        });

        console.log('creating user');

        const resp = await createUser({
            name, loginId: login, email, password
        });

        if (resp.success) {
            setTimeout(() => startTransition(() => {
                setSaving(false);
                setName('');
                setLogin('');
                setEmail('');
                setPassword('');
                setConfirmPassword('');
                setSuccess(true);
                setSuccessMessage(resp.message);
            }), 100);
        } else {
            setTimeout(() => startTransition(() => {
                setSaving(false);
                setError(true);
                setErrorMessage(resp.message);
            }), 100);
        }
    };

    return <div className="page page--create-users">
        <Breadcrumbs links={ [
            { text: 'Admin', to: '/admin' },
            { text: 'View Users', to: '/admin/users' },
            'Create User',
        ] } />

        <Typography variant="h5">
            New User
        </Typography>

        <Typography variant="subtitle1">
            Enter below details to create a new user.
        </Typography>

        <StyledGrid container>
            <Grid item xs={12} md={9} lg={8}>
                <StyledTextField
                    error={ nameError }
                    label="Name"
                    value={ name }
                    onChange={ onNameChanged }
                    onFocus={ onNameFocus }
                    onBlur={ onNameBlur }
                    helperText={
                        showNameHelper || nameError ?
                            'Full name; 4 - 64 characters' : ''
                    }
                    required />

                <StyledTextField
                    error={ loginError }
                    label="Username"
                    value={ login }
                    onChange={ onLoginChanged }
                    onFocus={ onLoginFocus }
                    onBlur={ onLoginBlur }
                    helperText={
                        showLoginHelper || loginError ?
                            '4 - 16 characters' : ''
                    }
                    required />

                <EmailTextField
                    value={ email }
                    forceShowError={ forceShowEmailError }
                    onChange={ onEmailChanged }
                    onError={ onEmailError }
                    onValid={ onEmailValid }
                    required />

                <Grid container>
                    <Grid item xs={12} md={6}>
                        <StyledTextField
                            error={ passwordError }
                            label="Password"
                            value={ password }
                            onChange={ onPasswordChanged }
                            onFocus={ onPasswordFocus }
                            onBlur={ onPasswordBlur }
                            helperText={
                                showPasswordHelper || passwordError ?
                                    '8 - 32 characters; Atleast 1 lowercase, 1 UPPERCASE alphabet, 1 number and 1 special character (!@#$%^&*())' : ''
                            }
                            required />
                    </Grid>

                    <Grid item xs={12} md={6}>
                        <StyledTextField
                            error={ confirmPasswordError }
                            label="Confirm Password"
                            value={ confirmPassword }
                            onChange={ onConfirmPasswordChanged }
                            onFocus={ onConfirmPasswordFocus }
                            onBlur={ onConfirmPasswordBlur }
                            helperText={
                                showConfirmPasswordHelper
                                || confirmPasswordError ?
                                    'Passwords should match' : ''
                            }
                            required />
                    </Grid>
                </Grid>

                {/* <FormControl>
                        <InputLabel id="role-chip-label">Roles</InputLabel>

                        <Select
                            labelId="role-chip-label"
                            id="role-chip"
                            input={
                                <OutlinedInput
                                    id="select-role-chip"
                                    label="User Roles"
                                />
                            }
                            renderValue={
                                (selected) => <ChipWrapper>
                                    { 
                                        selected.map((value) => <Chip
                                            key={ value }
                                            label={ value } />
                                        )
                                    }
                                </ChipWrapper>
                            }
                            multiple>
                        </Select>
                    </FormControl> */}
                {
                    error &&
                    <Alert severity='error' onClose={ onErrorMessageClosed }>
                        { errorMessage }
                    </Alert>
                }

                {
                    success &&
                    <Alert severity='success' onClose={ onSuccessMessageClosed }>
                        { successMessage }
                    </Alert>
                }
            </Grid>
        </StyledGrid>

        <Box sx={{ marginTop: '1rem' }}>
            <Button
                variant="contained"
                sx={{ marginRight: '0.5rem' }}
                disabled={
                    nameError || loginError || emailError || passwordError
                    || confirmPasswordError
                }
                onClick={ onSave }>
                {
                    saving ?
                        <Fragment>
                            <CircularProgress
                                size={ 16 }
                                color="secondary"
                                sx={{
                                    color: '#ffffff',
                                    marginRight: '1rem',
                                }} />
                            Saving
                        </Fragment>
                    :
                        'Save'
                }
            </Button>

            <Button variant="outlined">Cancel</Button>
        </Box>
    </div>;
};

export default Create;

