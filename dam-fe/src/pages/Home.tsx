import React from 'react';
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
    const navigate = useNavigate();

    const onLogin = () => {
        navigate('/workspace');
    };

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
                        <TextField required label="Username"/>
                        <TextField
                            required
                            label="Password"
                            type="password"
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
