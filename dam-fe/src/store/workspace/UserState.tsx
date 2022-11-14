import UserPermissions from "../../models/UserPermissions";

export default interface UserState {
    // Session token
    sessionToken: string,

    // Session token expiry timestamp
    sessionTokenExpiry: number,

    permissions: UserPermissions,

    setSession: (token: string, exp: number) => void;
    setPermissions: (permissions: UserPermissions) => void;
}
