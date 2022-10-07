export default interface UserState {
    // Session token
    sessionToken: string,

    // Session token expiry timestamp
    sessionTokenExpiry: number,

    setSession: (token: string, exp: number) => void;
}
