import create from 'zustand';
import { Session, defaultSession } from '../../models';

import UserState from './UserState';

const useUserStore = create<UserState>()((set) => ({
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

export default useUserStore;

