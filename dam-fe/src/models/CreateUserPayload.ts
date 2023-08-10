export interface CreateUserPayload {
    name: string,
    loginId: string,
    email: string,
    password: string,
    userRole: number,
}

export default CreateUserPayload;

