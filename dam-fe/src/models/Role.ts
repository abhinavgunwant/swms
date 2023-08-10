import UserPermissions, { UserPermissionsImpl } from './UserPermissions';

export interface Role {
    id: number,
    roleName: string,
    permissions: UserPermissions,
}

export class RoleImpl implements Role {
    id: number;
    roleName: string;
    permissions: UserPermissions;

    constructor (
        id: number = 0,
        name: string = '',
        permissions: UserPermissions = new UserPermissionsImpl()
    ) {
        this.id = id;
        this.roleName = name;
        this.permissions = permissions;
    }
}

export default Role;

