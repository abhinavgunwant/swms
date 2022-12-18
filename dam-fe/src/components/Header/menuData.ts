import MenuItem from '../../models/MenuItem';

const menu: MenuItem[] = [
    {
        title: "Users",
        navigateTo: '',
        children: [
            {
                title: 'View Users',
                navigateTo: '/users',
                children: [],
            },
            {
                title: 'Create Users',
                navigateTo: '/users/create',
                children: [],
            },
        ]
    },
    {
        title: "Settings",
        navigateTo: '',
        children: [],
    },
];

export default menu;

