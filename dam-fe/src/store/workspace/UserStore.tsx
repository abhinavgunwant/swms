import create from 'zustand';
import UserPermissions from '../../models/UserPermissions';

import UserState from './UserState';

const useUserStore = create<UserState>()((set, get) => ({
    // Session token
    sessionToken: '',

    // Expiry time
    sessionTokenExpiry: 0,

    permissions: {
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

    setPermissions: (permissions: UserPermissions) => set((state) => ({
        ...state, permissions
    })),
}));

export default useUserStore;
