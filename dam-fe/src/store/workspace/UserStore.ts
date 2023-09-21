import { createStore, useStore } from 'zustand';
import { Session, defaultSession } from '../../models';

import UserState from './UserState';

//const useUserStore = create<UserState>()((set) => ({
export const userStore = createStore<UserState>((set) => ({
    // Session Token
    sessionToken: '',
    // Session
    session: defaultSession,

    setSessionToken: (t: string) => set((state) =>
        ({ ...state, sessionToken: t })
    ),
    setSession: (s: Session) => set((state) => ({ ...state, session: s })),
    resetSession: () => set(() =>
        ({ sessionToken: '', session: defaultSession})
    ),
}));

const useUserStore = <T>(selector: (state: UserState) => T) =>
    useStore(userStore, selector);

export default useUserStore;

