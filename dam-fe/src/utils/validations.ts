const slugPattern = /[^a-z0-9.\-_\(\)\s]/g;
const slugPatternWoWS = /^[a-z0-9.\-_\(\)]+$/;
/**
 * Takes raw string as input and returns a slug.
 */
export const generateSlug = (str: string) => {
    let s = str.trim().toLowerCase().replace(slugPattern, '');
    return s.replaceAll(/\s/g, '-');
};

export const validSlug = (str: string) =>
    str === '' || slugPatternWoWS.test(str);

export default generateSlug;

