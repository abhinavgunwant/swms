import UserListing from '../../models/UserListing';
import Role from '../../models/Role';

export default interface AdminState {
    userToEdit?: UserListing,
    roleToEdit?: Role,
    roles: Role[],

    setUserToEdit: (userToEdit: UserListing) => void,
    setRoleToEdit: (roleToEdit: Role) => void,
    setRoles: (roles: Role[]) => void,
}

