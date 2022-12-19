import {
    Group, Settings, PersonSearch, GroupAdd
} from '@mui/icons-material';

import MenuItem from '../../models/MenuItem';

const menu: MenuItem[] = [
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

