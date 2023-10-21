export interface Folder {
    id: number,
    slug: string,
    title: string,
    projectId: number,
    description: string,
    parentFolderId: number,
    createdBy: number,
    modifiedBy: number,
    createdOn: string,
    modifiedOn: string,
}

export const DEFAULT_FOLDER: Folder = {
    id: 0,
    slug: '',
    title: '',
    description: '',
    projectId: 0,
    parentFolderId: 0,
    createdBy: 0,
    modifiedBy: 0,
    createdOn: '',
    modifiedOn: '',
};

export default Folder;

