import { useState, useTransition } from 'react'

import {
    AppBar, Toolbar, IconButton, Typography, Drawer, List, ListItem,
    ListItemButton
} from '@mui/material';

import MenuIcon from '@mui/icons-material/Menu';

import Menu from './Menu';

const Header = (): React.ReactElement => {
    const [ drawerOpen, setDrawerOpen ] = useState<boolean>(false);

    // eslint-disable-next-line @typescript-eslint/no-unused-vars 
    const [ _, startTransition ] = useTransition();

    const onToggleDrawer = () => {
        startTransition(() => setDrawerOpen(!drawerOpen));
    }

    return <header className="header">
        <AppBar position="fixed">
            <Toolbar>
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
                Dam
            </Typography>
            
            </Toolbar>
        </AppBar>

        <Menu drawerOpen={ drawerOpen } onToggleDrawer={ onToggleDrawer } />
    </header>
}

export default Header;
