import { Session } from '../models';

export const sessionFromToken: (t: string) => Session = (token: string) => {
    const payload = token.split('.')[1];
    const base64payload = payload.replace(/-/g, '+').replace(/_/g, '/');
    const jsonPayload = decodeURIComponent(
        window.atob(base64payload).split('').map((c) => {
            return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
        }).join('')
    );

    const p: any = JSON.parse(jsonPayload);

    return {
        username: p.sub,
        name: p.name,
        expiry: new Date(p.exp * 1000),
        role: {
            id: p.role.id,
            ...p.role.permissions,
        },
    };
};

