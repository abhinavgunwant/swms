import UserListing from '../../models/UserListing';
import Role from '../../models/Role';

export default interface AdminState {
    userToEdit?: UserListing,
    roleToEdit?: Role,

    setUserToEdit: (userToEdit: UserListing) => void,
    setRoleToEdit: (roleToEdit: Role) => void,
}

