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

export default {};

