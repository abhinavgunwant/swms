import { ReactElement } from 'react';

export interface MenuItem {
    title: string,
    navigateTo: string,
    icon?: ReactElement,
    children: MenuItem[],
}

export default MenuItem;

