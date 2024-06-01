import { createStore, useStore } from 'zustand';
import { Session, defaultSession } from '../../models';
import { SessionState } from './UserState';

import UserState from './UserState';

export const userStore = createStore<UserState>((set) => ({
    // Session Token
    sessionToken: '',
    // Session
    session: defaultSession,
    sessionState: SessionState.LoggedOut,

    setSessionToken: (t: string) => set((state) =>
        ({ ...state, sessionToken: t })
    ),
    setSession: (s: Session) => set((state) => ({ ...state, session: s })),
    resetSession: () => set(() =>
        ({ sessionToken: '', session: defaultSession})
    ),
    setSessionState: (s: SessionState) => set((state) => ({
        ...state, sessionState: s
    })),
}));

const useUserStore = <T>(selector: (state: UserState) => T) =>
    useStore(userStore, selector);

export default useUserStore;

