import React, { ChangeEvent, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import Grid from '@mui/material/Grid';
import Typography from '@mui/material/Typography';
import Card from '@mui/material/Card';
import CardContent from '@mui/material/CardContent';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';

import styled from '@emotion/styled';

const FullWidthColumn = styled.div`
    display: flex;
    flex-direction: column;
    width: 100%;
    margin-top: 1rem;
`

const Home = (): React.ReactElement => {
    const [ username, setUsername ] = useState('');
    const [ password, setPassword ] = useState('');

    const navigate = useNavigate();

    const onLogin = async () => {
        const response = await fetch('http://localhost:8080/api/admin/auth', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, password }),
        });

        if (response.status === 200) {
            const responseJson = await response.json();
    
            if (responseJson.success) {
                navigate('/workspace');
            }

            return;
        }
        
        console.log('Username and password combination is incorrect!');
    };

    const onUsernameChanged = (e: ChangeEvent<HTMLInputElement>) => {
        setUsername(e.target.value);
    }

    const onPasswordChanged = (e: ChangeEvent<HTMLInputElement>) => {
        setPassword(e.target.value);
    }

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

                    <FullWidthColumn>
                        <TextField
                            label="Username"
                            value={ username }
                            onChange={ onUsernameChanged }
                            required />
                        <TextField
                            required
                            label="Password"
                            type="password"
                            value={ password }
                            onChange={ onPasswordChanged }
                            sx={{ marginTop: '0.5rem' }} />
                    </FullWidthColumn>

                    <Button
                        type="submit"
                        variant="contained"
                        sx={{
                            textTransform: 'capitalize',
                            marginTop: '0.5rem',
                        }}
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
                </CardContent>
            </Card>
        </Grid>
    </div>
}

export default Home;
