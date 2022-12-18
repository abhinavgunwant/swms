import { Fragment, useState, useTransition } from 'react';

//import { useNavigate } from 'react-router-dom';

import {
    Drawer, Typography, List, ListItem, ListItemButton
} from '@mui/material';

import menuData from './menuData';
import MenuItem from '../../models/MenuItem';

interface MenuProps {
    drawerOpen: boolean,
    onToggleDrawer: () => void,
}

const Menu = (props: MenuProps) => {
    //const navigate = useNavigate();

    const onClicked = (m: MenuItem) => {
        if (m.children.length > 0) {
            // TODO: Display child menu here...
            return;
        }

        window.location = m.navigateTo as (string | Location) & Location;
    };

    return <Fragment>
        <Drawer
            anchor="left"
            open={ props.drawerOpen }
            onClose={ props.onToggleDrawer }
            sx={{ width: 300 }}>
            <Typography
                variant="h4"
                sx={{
                    width: '100%',
                    textAlign: 'center',
                    marginTop: '2rem',
                }}>
                Dam
            </Typography>

            <List sx={{ width: 300 }}>
                {
                    menuData.map((m) => <ListItem>
                        <ListItemButton onClick={
                            () => onClicked(m)}
                            >
                            { m.title }
                        </ListItemButton>
                    </ListItem>)
                }
            </List>
        </Drawer>
    </Fragment>
};

export default Menu;

