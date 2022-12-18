export default interface MenuItem {
    title: string,
    navigateTo: string,
    children: MenuItem[],
}

