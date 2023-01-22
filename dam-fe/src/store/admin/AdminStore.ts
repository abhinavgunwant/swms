import create from 'zustand';

import AdminState from './AdminState';
import UserListing from '../../models/UserListing';
import Role from '../../models/Role';

export const useAdminStore = create<AdminState>()((set, get) => ({
    setUserToEdit: (userToEdit: UserListing) => set((state) => ({
        ...state,
        userToEdit
    })),

    setRoleToEdit: (roleToEdit: Role) => set((state) => ({
        ...state,
        roleToEdit,
    })),
}));

export default useAdminStore;

