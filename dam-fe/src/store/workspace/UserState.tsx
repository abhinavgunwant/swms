import { Session } from '../../models';

export default interface UserState {
    sessionToken: string,
    // Session
    session: Session,

    setSessionToken: (t: string) => void;
    setSession: (s: Session) => void;
    resetSession: () => void;
}

