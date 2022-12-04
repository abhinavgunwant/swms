interface Image {
    id: number,
    name: string,
    title: string,
    encoding: string,
    height: number,
    width: number,
    isPublished: boolean,
    projectId: number,
    folderId: number,
    metadataId: number,
    slug: string,
    createdOn: string,
    createdBy: number,
    modifiedOn: string,
    modifiedBy: number,
}

export const default_image: (() => Image) = () => {
    return {
        id: 0,
        name: '',
        title: '',
        encoding: 'JPG',
        height: 0,
        width: 0,
        isPublished: false,
        projectId: 0,
        folderId: 0,
        metadataId: 0,
        slug: '',
        createdOn: '',
        createdBy: 0,
        modifiedOn: '',
        modifiedBy: 0,
    };
}

export default Image;

