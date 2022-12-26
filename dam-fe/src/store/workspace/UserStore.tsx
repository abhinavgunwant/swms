import create from 'zustand';
import UserRole from '../../models/UserRole';

import UserState from './UserState';

const useUserStore = create<UserState>()((set, get) => ({
    // Session token
    sessionToken: '',

    // Expiry time
    sessionTokenExpiry: 0,

    role: {
        id: 0,
        roleName: '',
        createImage: false,
        readImage: false,
        modifyImage: false,
        deleteImage: false,
        readRenditions: false,
        createRenditions: false,
        modifyRenditions: false,
        deleteRenditions: false,
        readProject: false,
        createProject: false,
        modifyProject: false,
        deleteProject: false,
        readUser: false,
        createUser: false,
        modifyUser: false,
        deleteUser: false,
        publish: false,
        publishAll: false,
        accessAllProjects: false,
    },

    setSession: (token: string, exp: number) => set((state) => ({
        ...state,
        sessionToken: token,
        sessionTokenExpiry: exp,
    })),

    setRole: (role: UserRole) => set((state) => ({
        ...state, role 
    })),
}));

export default useUserStore;
