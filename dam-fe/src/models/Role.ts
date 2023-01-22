import UserPermissions, { UserPermissionsImpl } from './UserPermissions';

export default interface Role {
    id: number,
    roleName: string,
    permissions: UserPermissions,
}

export class RoleImpl implements Role {
    id: number;
    roleName: string;
    permissions: UserPermissions;

    constructor (
        id: number = -1,
        name: string = '',
        permissions: UserPermissions = new UserPermissionsImpl()
    ) {
        this.id = id;
        this.roleName = name;
        this.permissions = permissions;
    }
}

