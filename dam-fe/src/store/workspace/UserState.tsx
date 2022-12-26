import UserRole from "../../models/UserRole";

export default interface UserState {
    // Session token
    sessionToken: string,

    // Session token expiry timestamp
    sessionTokenExpiry: number,

    role: UserRole,

    setSession: (token: string, exp: number) => void;
    setRole: (role: UserRole) => void;
}

