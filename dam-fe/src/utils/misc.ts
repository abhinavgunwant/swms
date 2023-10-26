import { userStore } from '../store/workspace/UserStore';
// import useWorkspaceStore from '../store/workspace/WorkspaceStore';

import { sessionFromToken } from './token';

export const isEmpty = (a: any) => {
    if (typeof a === 'undefined' || a === '' || !a) {
        return true;
    }

    return false;
}

export const generateId = (length: number = 8) => {
    const characters = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890-_';
    const len = characters.length;

    let result = '';
    let count = 0;

    while (count < length) {
        result += characters.charAt(Math.floor((Math.random() * 100) % len));

        ++count;
    }

    return result;
}

export const getLatestSessionToken = async () => {
    const response = await fetch('/api/admin/auth/refresh', {
        credentials: 'include',
    });

    try {
        if (response.status == 200) {
            return await response.text();
        }
    } catch (e) { console.log(e); }

    return '';
};

export const apiCall = async (
    url: string, options: RequestInit = {}, authorized: boolean = true
) => {
    if (authorized) {
        const now = new Date();

        let token;

        if (userStore.getState().session.expiry <= now) {
            token = await getLatestSessionToken();

            userStore.getState().setSession(sessionFromToken(token));
            userStore.getState().setSessionToken(token);
        } else {
            token = userStore.getState().sessionToken;
        }

        let headers: HeadersInit;

        if (options.headers) {
            headers = new Headers(options.headers);
        } else {
            headers = new Headers();
        }

        headers.set('Authorization', 'Bearer ' + token);

        options.headers = headers;
    }

    return fetch(url, options);
};

export default {};

