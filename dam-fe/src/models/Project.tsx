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

export const DEFAULT_PROJECT: Project = {
    id: 0,
    name: '',
    slug: '',
    description: '',
    restrictUsers: false,
    createdBy: 0,
    modifiedBy: 0,
    createdOn: '',
    modifiedOn: '',
};

export default Project;

