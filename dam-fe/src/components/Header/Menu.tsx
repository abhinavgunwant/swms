import { Fragment, useState, useEffect, useTransition, useRef } from 'react';
//import { useNavigate } from 'react-router-dom';

import {
    Drawer, Typography, List, ListItem, ListItemButton, ListItemText,
    ListItemIcon, Box
} from '@mui/material';

import { ChevronLeft, ChevronRight } from '@mui/icons-material';

import menuData from './menuData';
import MenuItem from '../../models/MenuItem';

import { styled } from '@mui/material/styles';

const MENU_PAGE_WIDTH = 300;

const MenuPage = styled(Box)`
    width: ${ MENU_PAGE_WIDTH }px;
    flex: 0 0 auto;
`;

const MenuPageRoot = styled(Box)`
    display: flex;
    justify-content: flex-start;
    align-items: flex-start;
    width: 300px;
    overflow-x: hidden;
    flex: 0 0 auto;
`;

const BackMenuButton = styled(ListItemButton)`
    background: #eeeeee;
`;

interface MenuProps {
    drawerOpen: boolean,
    onToggleDrawer: () => void,
}

const Menu = (props: MenuProps) => {
    // Count of extra pages
    const [ pages, setPages ] = useState<MenuItem[]>([]);
    const [ currentPageIndex, setCurrentPageIndex ] = useState<number>(-1);

    // eslint-disable-next-line @typescript-eslint/no-unused-vars 
    const [ _, startTransition ] = useTransition();

    const menuPageRootRef = useRef<HTMLDivElement>(null);

    const onClose = () => {
        props.onToggleDrawer();

        setTimeout(() => startTransition(() => {
            setPages([]);
            setCurrentPageIndex(0);
        }), 100);
    }

    const onBack = () => {
        if (currentPageIndex >= 0 && pages.length > 0) {
//            const pageLength = pages.length;
//            const pagesCopy = [ ...pages ];
//            pagesCopy.splice(pageLength-1, 1);

            startTransition(() => {
                //setPages(pagesCopy);
                setCurrentPageIndex(currentPageIndex - 1);
            });
        }
    }

    const onClicked = (m: MenuItem) => {
        if (m.children.length > 0) {
            // TODO: Display child menu here...
            startTransition(() => {
                setPages([...pages, m]);
                setCurrentPageIndex(currentPageIndex + 1);
            });

            return;
        }

        window.location = m.navigateTo as (string | Location) & Location;
    };

    useEffect(() => {
        if (
            menuPageRootRef
            && menuPageRootRef.current
            && currentPageIndex >= -1
        ) {
            menuPageRootRef.current.scroll({
                top: 0,
                left: MENU_PAGE_WIDTH * (currentPageIndex + 1),
                behavior: 'smooth',
            });

            if (pages.length > currentPageIndex + 1) {
                const pageLength = pages.length;
                const pagesCopy = [ ...pages ];

                pagesCopy.splice(pageLength - 1, 1);

                setTimeout(() =>
                    startTransition(() => setPages(pagesCopy)), 300
                );
            }
        }
    }, [ pages, currentPageIndex ]);

    return <Fragment>
        <Drawer
            anchor="left"
            open={ props.drawerOpen }
            onClose={ onClose }>

            <MenuPageRoot ref={ menuPageRootRef }>
                <MenuPage>
                    <Typography
                        variant="h4"
                        sx={{
                            width: '100%',
                            textAlign: 'center',
                            marginTop: '2rem',
                        }}>
                        SWMS
                    </Typography>

                    <List sx={{ width: 300 }}>
                        {
                            menuData.map((m, i) => <ListItem
                                key={ i }
                                disablePadding>
                                <ListItemButton onClick={() => onClicked(m)}>
                                    {
                                        m.icon &&
                                        <ListItemIcon>{ m.icon }</ListItemIcon>
                                    }

                                    <ListItemText primary={ m.title } />
                                    {
                                        m.children.length ?
                                            <ChevronRight /> : ''
                                    }
                                </ListItemButton>
                            </ListItem>)
                        }
                    </List>
                </MenuPage>

                {
                    pages.map((menuItem, i) => <MenuPage key={ i }>
                        <List disablePadding>
                            <ListItem disablePadding>
                                <BackMenuButton onClick={ onBack }>
                                    <ListItemIcon>
                                        <ChevronLeft />
                                        { menuItem.icon }
                                    </ListItemIcon>

                                    <ListItemText primary={ menuItem.title } />
                                </BackMenuButton>
                            </ListItem>
                            {
                                menuItem.children.map((menuChild, i) =>
                                    <ListItem key={ i } disablePadding>
                                        <ListItemButton onClick={
                                                () => onClicked(menuChild)
                                            }>
                                            {
                                                menuChild.icon &&
                                                <ListItemIcon>
                                                    { menuChild.icon }
                                                </ListItemIcon>
                                            }

                                            <ListItemText
                                                primary={ menuChild.title } />

                                            {
                                                menuChild.children.length ?
                                                    <ChevronRight /> : ''
                                            }
                                        </ListItemButton>
                                    </ListItem>
                                )
                            }
                        </List>
                    </MenuPage>)
                }
            </MenuPageRoot>
        </Drawer>
    </Fragment>
};

export default Menu;

