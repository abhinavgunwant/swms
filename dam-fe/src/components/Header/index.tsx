import { useState, useTransition } from 'react'

import {
    AppBar, Toolbar, IconButton, Typography, Button,
} from '@mui/material';

import MenuIcon from '@mui/icons-material/Menu';

import useUserStore from '../../store/workspace/UserStore';
import UserState from  '../../store/workspace/UserState';

import { styled } from '@mui/material/styles';

import Menu from './Menu';

const CustomToolbar = styled(Toolbar)`
    justify-content: space-between;
`;

const userSelector = (state: UserState) => ({
    session: state.session,
    sessionToken: state.sessionToken,
    setSessionToken: state.setSessionToken,
    setSession: state.setSession,
    resetSession: state.resetSession,
});

const Header = (): React.ReactElement => {
    const [ drawerOpen, setDrawerOpen ] = useState<boolean>(false);

    // eslint-disable-next-line @typescript-eslint/no-unused-vars 
    const [ _, startTransition ] = useTransition();

    const userStore = useUserStore(userSelector);

    const onToggleDrawer = () => {
        startTransition(() => setDrawerOpen(!drawerOpen));
    }

    const onLogout = async () => {
        const resp = await fetch('/api/admin/auth/logout');

        if (resp.status === 200) {
            userStore.resetSession();
            window.location.pathname = '/';
        }
    };

    return <header className="header">
        <AppBar position="fixed">
            <CustomToolbar>
            <IconButton
                size="large"
                edge="start"
                color="inherit"
                aria-label="menu"
                sx={{ mr: 2 }}
                onClick={ onToggleDrawer }>
                    <MenuIcon />
            </IconButton>

            <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                SWMS: Simple Web Multimedia Server
            </Typography>
            
            {
                userStore.session.username &&
                <Typography component="div">
                    { userStore.session.username }
                    <Button
                        sx={{
                            color: '#ffffff',
                            textTransform: 'capitalize',
                            '&:hover': {
                                textDecoration: 'underline',
                            }
                        }}
                        variant="text"
                        onClick={ onLogout }>
                            (Logout)
                    </Button>
                </Typography>
            }

            </CustomToolbar>
        </AppBar>

        <Menu drawerOpen={ drawerOpen } onToggleDrawer={ onToggleDrawer } />
    </header>
}

export default Header;
