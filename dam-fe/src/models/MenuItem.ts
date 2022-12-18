import { FC, ReactNode, ReactElement } from 'react';

export default interface MenuItem {
    title: string,
    navigateTo: string,
    icon?: ReactElement,
    children: MenuItem[],
}

