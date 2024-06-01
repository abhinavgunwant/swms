import { NavigateFunction } from 'react-router-dom';

import { SessionState } from '../store/workspace/UserState';
import { userStore } from '../store/workspace/UserStore';

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

export const getLatestSessionToken = async (navigate?: NavigateFunction) => {
    try {
        const response = await fetch('/api/admin/auth/refresh', {
            credentials: 'include',
        });

        let respText = await response.text();

        if (response.status == 200) {
            return respText;
        }

        if (response.status >= 400) {
            if (userStore.getState().sessionState === SessionState.LoggedIn) {
                if (respText.toUpperCase() === 'YOU\'RE NOT SIGNED IN!') {
                    userStore.getState().setSessionState(
                        SessionState.SessionTimedout
                    );
                } else {
                    userStore.getState().setSessionState(
                        SessionState.SessionError
                    );
                }
            } else {
                userStore.getState().setSessionState(
                    SessionState.SessionError
                );
            }

            if (window.location.pathname !== '/' && navigate) {
                navigate('/');
                // window.location.pathname = '/';
            }
        }
    } catch (e) {
        userStore.getState().setSessionState(SessionState.SessionError);

        if (navigate) {
            navigate('/');
        }

        // window.location.pathname = '/';

        console.log(e);
    }

    return '';
};

/**
 * The standard function to call rest endpoints.
 *
 * @param url the rest api URL.
 * @param options an object of type `RequestInit`.
 * @param navigate an object of type `NavigateFunction` passed down from the
 *      component where this api call is made.
 * @param authorized whether the user is assumed to be authorized.
 */
export const apiCall = async (
    url: string,
    options: RequestInit = {},
    navigate?: NavigateFunction,
    authorized: boolean = true
) => {
    if (authorized) {
        const now = new Date();

        let token;

        if (userStore.getState().session.expiry <= now) {
            token = await getLatestSessionToken(navigate);

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

