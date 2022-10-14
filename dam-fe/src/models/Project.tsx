export default interface Project {
    id: number,
    name: string,
    slug: string,
    description: string,
    restrict_user: boolean,
    created_by: number,
    modified_by: number,
    created_on: number,
    modified_on: number,
}
