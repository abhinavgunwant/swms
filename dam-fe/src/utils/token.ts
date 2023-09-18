import { UserRole, Session } from '../models';

interface JsonPayload {
    sub: string,
    name: string,
    exp: number,
    iat: number,
    role: UserRole,
}

export const sessionFromToken: (t: string) => Session = (token: string) => {
    const payload = token.split('.')[1];
    const base64payload = payload.replace(/-/g, '+').replace(/_/g, '/');
    const jsonPayload = decodeURIComponent(
        window.atob(base64payload).split('').map((c) => {
            return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
        }).join('')
    );

    const p: JsonPayload = JSON.parse(jsonPayload);

    return {
        username: p.sub,
        name: p.name,
        expiry: new Date(p.exp * 1000),
        role: p.role,
    };
};

