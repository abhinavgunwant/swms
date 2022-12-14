export default interface Rendition {
    id: number,
    imageId: number,
    height: number,
    width: number,
    targetDevice: string,
    slug: string,
    isPublished: boolean,
    encoding: string,
    createdOn: string,
    createdBy: number,
    modifiedOn: string,
    modifiedBy: number,
}

