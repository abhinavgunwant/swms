import create from 'zustand';

import AdminState from './AdminState';
import UserListing from '../../models/UserListing';

const useAdminStore = create<AdminState>()((set, get) => ({
    setUserToEdit: (userToEdit: UserListing) => set((state) => ({
        ...state,
        userToEdit
    }))
}));

export default useAdminStore;

