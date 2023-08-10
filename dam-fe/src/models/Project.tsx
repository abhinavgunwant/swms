export interface Project {
    id: number,
    name: string,
    slug: string,
    description: string,
    restrictUsers: boolean,
    createdBy: number,
    modifiedBy: number,
    createdOn: string,
    modifiedOn: string,
}

export default Project;

