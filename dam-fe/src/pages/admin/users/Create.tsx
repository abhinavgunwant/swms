import { useState, useTransition, Fragment, ChangeEvent } from 'react';

import {
    Typography, Grid, TextField, Button, Box, CircularProgress,
} from '@mui/material';

import Breadcrumbs from '../../../components/Breadcrumbs';
import EmailTextField from '../../../components/EmailTextField';

import { styled } from '@mui/material/styles';

const StyledTextField = styled(TextField)`
    width: 100%;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
`;

const StyledGrid = styled(Grid)`
    margin-top: 1rem;
`;

const Create = () => {
    const [ name, setName ] = useState<string>('');
    const [ login, setLogin ] = useState<string>('');
    const [ email, setEmail ] = useState<string>('');
    const [ password, setPassword ] = useState<string>('');
    const [ confirmPassword, setConfirmPassword ] = useState<string>('');
    const [ saving, setSaving ] = useState<boolean>(false);

    // eslint-disable-next-line @typescript-eslint/no-unused-vars 
    const [ _, startTransition ] = useTransition();

    const onNameChanged = (e: ChangeEvent<HTMLInputElement>) =>
        setName(e.target.value);

    const onLoginChanged = (e: ChangeEvent<HTMLInputElement>) =>
        setLogin(e.target.value);

    const onEmailChanged = (e: ChangeEvent<HTMLInputElement>) =>
        setEmail(e.target.value);

    const onPasswordChanged = (e: ChangeEvent<HTMLInputElement>) =>
        setPassword(e.target.value);

    const onConfirmPasswordChanged = (e: ChangeEvent<HTMLInputElement>) =>
        setConfirmPassword(e.target.value);

    const onSave = () => {
        console.log('Save Clicked!');
        if (saving) {
            return;
        }

        startTransition(() => setSaving(true));
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
            <Grid item xs={12} lg={6}>
                <StyledTextField
                    label="Name"
                    value={ name }
                    onChange={ onNameChanged }
                    required />

                <StyledTextField
                    label="Username"
                    value={ login }
                    onChange={ onLoginChanged }
                    required />

                <EmailTextField
                    value={ email }
                    onChange={ onEmailChanged }
                    required />

                <StyledGrid container>
                    <Grid item xs={12} md={6}>
                        <StyledTextField
                            label="Password"
                            value={ password }
                            onChange={ onPasswordChanged }
                            required />
                    </Grid>

                    <Grid item xs={12} md={6}>
                        <StyledTextField
                            label="Confirm Password"
                            value={ confirmPassword }
                            onChange={ onConfirmPasswordChanged }
                            required />
                    </Grid>
                </StyledGrid>
            </Grid>
        </StyledGrid>

        <Box sx={{ marginTop: '1rem' }}>
            <Button
                variant="contained"
                sx={{ marginRight: '0.5rem' }}
                disabled={ false }
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

