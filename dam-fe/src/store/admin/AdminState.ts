import UserListing from '../../models/UserListing';

export default interface AdminState {
    userToEdit?: UserListing,

    setUserToEdit: (userToEdit: UserListing) => void,
}

