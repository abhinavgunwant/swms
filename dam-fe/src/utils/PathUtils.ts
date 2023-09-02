const pathRegEx = /^\/workspace\/tree\/[a-zA-Z0-9\-]+\/([a-zA-Z0-9\-\/]+)\/[a-zA-Z0-9\-]+(\.\w+)?$/;

export const getImagePathFromURL = ():string => {
    const pathname = window.location.pathname || '';

    if (pathRegEx.test(pathname)){
        const matches = pathRegEx.exec(pathname);

        if (matches) {
            return matches[1] || '';
        }
    }

    return '';
}

export const generateThumbnailURL = (path: string, imgSlug: string):string =>
    'http://localhost:8080/api/image/' + path + '/' + imgSlug
    + '/ui-thumb-default.jpg';

