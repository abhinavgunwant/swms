import {
    Group, Settings, PersonSearch, GroupAdd, Workspaces, AdminPanelSettings
} from '@mui/icons-material';

import MenuItem from '../../models/MenuItem';

const menu: MenuItem[] = [
    {
        title: "Workspace",
        navigateTo: '/workspace',
        icon: <Workspaces />,
        children: [],
    },
    {
        title: "Users",
        navigateTo: '',
        icon: <Group />,
        children: [
            {
                title: 'View Users',
                navigateTo: '/admin/users',
                icon: <PersonSearch />,
                children: [],
            },
            {
                title: 'Create Users',
                navigateTo: '/admin/users/create',
                icon: <GroupAdd />,
                children: [],
            },
            {
                title: 'View User Roles',
                navigateTo: '/admin/roles',
                icon: <AdminPanelSettings />,
                children: [],
            },
        ]
    },
    {
        title: "Settings",
        navigateTo: '',
        icon: <Settings />,
        children: [],
    },
];

export default menu;

