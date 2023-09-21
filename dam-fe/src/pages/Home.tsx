import React, {
    ChangeEvent, Fragment, useEffect, useState, useTransition,
} from 'react';
import { useNavigate } from 'react-router-dom';

import {
    Alert, Grid, Typography, Card, CardContent, TextField, Button,
    CircularProgress,
} from '@mui/material/';

import UserState from '../store/workspace/UserState';
import useUserStore from '../store/workspace/UserStore';
import { sessionFromToken } from '../utils/token';

import useAPI from '../hooks/useAPI';

import { getLatestSessionToken } from '../utils/misc';

import { styled as materialStyled } from '@mui/material/styles';
import styled from '@emotion/styled';

const LoginAlert = materialStyled(Alert)`
    margin: 1rem 0 0.5rem 0;
`;

const FullWidthColumn = styled.div`
    display: flex;
    flex-direction: column;
    width: 100%;
    margin-top: 1rem;
`;

const FlexCentered = styled.div`
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 1rem;
`;

const ProcessingAnimation = () => <FlexCentered>
    <CircularProgress />
</FlexCentered>;

const userSelector = (state: UserState) => ({
    session: state.session,
    sessionToken: state.sessionToken,
    setSessionToken: state.setSessionToken,
    setSession: state.setSession,
});

const Home = (): React.ReactElement => {
    const userStore = useUserStore(userSelector);

    const [ username, setUsername ] = useState<string>('');
    const [ password, setPassword ] = useState<string>('');

    const [ error, setError ] = useState<string>('Unknown error');
    const [ showError, setShowError ] = useState<boolean>(false);
    const [ processing, setProcessing ] = useState<boolean>(false);
    const [ checkingSession, setCheckingSession ] = useState<boolean>(true);

    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const [ _, startTransition ] = useTransition();

    const { login } = useAPI();

    const navigate = useNavigate();

    const onLogin = async (e: React.SyntheticEvent) => {
        if (e) {
            e.preventDefault();
        }

        if (checkingSession) {
            return;
        }

        if (username === '' || password === '') {
            startTransition(() => {
                setProcessing(false);
                setError('Username and Password should not be empty.');
                setShowError(true);
            });

            return;
        }

        startTransition(() => {
            setShowError(false);
            setProcessing(true);
        });

        const loginResponse = await login(username, password);

        if (loginResponse.success && loginResponse.s) {
            const session = sessionFromToken(loginResponse.s);

            userStore.setSessionToken(loginResponse.s);
            userStore.setSession(session);

            navigate('/workspace');
            return;
        }

        if (loginResponse.status === 404) {
            startTransition(() => {
                setError('The username and password combination is invalid. Retry with correct credentials.');
                setShowError(true);
                setProcessing(false);
            });
            
            return;
        }

        startTransition(() => {
            setError(loginResponse.message);
            setShowError(true);
            setProcessing(false);
        });
    };

    const onUsernameChanged = (e: ChangeEvent<HTMLInputElement>) => {
        setUsername(e.target.value);
        setShowError(false);
    }

    const onPasswordChanged = (e: ChangeEvent<HTMLInputElement>) => {
        setPassword(e.target.value);
        setShowError(false);
    }

    useEffect(() => {
        if (userStore.sessionToken) {
            navigate('/workspace');
        } else {
            async function f() {
                const token = await getLatestSessionToken();

                if (token) {
                    userStore.setSession(sessionFromToken(token));
                    userStore.setSessionToken(token);

                    navigate('/workspace');
                } else {
                    startTransition(() => setCheckingSession(false));
                }
            }

            f();
        }
    }, []);

    return <div className="homepage">
        <Grid
            container
            direction="column"
            alignItems="center"
            justifyContent="center"
            style={{ minHeight: 'calc(100vh - 64px)' }}
            >
            <Card sx={{ width: '480px'}}>
                <CardContent>
                    <Typography variant="h4">
                        Login
                    </Typography>

                    <Typography>
                        In order to use DAM, you must login.
                    </Typography>

                    <form onSubmit={ onLogin }>
                        <FullWidthColumn>
                            <TextField
                                label="Username"
                                value={ username }
                                onChange={ onUsernameChanged }
                                error={ showError }
                                required />
                            <TextField
                                label="Password"
                                type="password"
                                value={ password }
                                onChange={ onPasswordChanged }
                                error={ showError }
                                sx={{ marginTop: '0.5rem' }}
                                required />
                        </FullWidthColumn>

                        {
                            showError &&
                            <LoginAlert severity="error">
                                { error }
                            </LoginAlert>
                        }

                        {
                            processing ?
                                <ProcessingAnimation />
                            :
                                <Fragment>
                                    <Button
                                        type="submit"
                                        variant="contained"
                                        sx={{
                                            textTransform: 'capitalize',
                                            marginTop: '0.5rem',
                                        }}
                                        disabled={
                                            showError || checkingSession
                                        }
                                        onClick={ onLogin }>
                                        Login
                                    </Button>
                                    <Button
                                        variant="text"
                                        sx={{
                                            textTransform: 'capitalize',
                                            marginTop: '0.5rem',
                                            marginLeft: '0.5rem',
                                        }}>
                                        Forgot password?
                                    </Button>
                                </Fragment>
                        }
                    </form>
                </CardContent>
            </Card>
        </Grid>
    </div>
}

export default Home;

