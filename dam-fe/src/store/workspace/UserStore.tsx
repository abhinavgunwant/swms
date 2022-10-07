import create from 'zustand';

import UserState from './UserState';

const useUserStore = create<UserState>()((set, get) => ({
    // Session token
    sessionToken: '',

    // Expiry time
    sessionTokenExpiry: 0,

    setSession: (token: string, exp: number) => set((state) => ({
        ...state,
        sessionToken: token,
        sessionTokenExpiry: exp,
    }))
}));

export default useUserStore;
