export const isEmpty = (a: any) => {
    if (typeof a === 'undefined' || a === '' || !a) {
        return true;
    }

    return false;
}

export default {};

