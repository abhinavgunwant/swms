interface Folder {
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

export default Folder;

