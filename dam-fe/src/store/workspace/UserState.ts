import { Session } from '../../models';

export enum SessionState {
    LoggedOut,
    LoggedIn,
    SessionTimedout,
    SessionError,
}

export default interface UserState {
    sessionToken: string,
    session: Session,
    sessionState: SessionState,

    setSessionToken: (t: string) => void;
    setSession: (s: Session) => void;
    resetSession: () => void;
    setSessionState: (s: SessionState) => void;
}

