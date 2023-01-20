import UserPermissions from './UserPermissions';

export default interface Role {
    id: number,
    roleName: string,
    permissions: UserPermissions,
}

