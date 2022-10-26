interface Image {
    id: number,
    name: string,
    title: string,
    encoding: string,
    height: number,
    width: number,
    is_published: boolean,
    project_id: number,
    folder_id: number,
    metadata_id: number,
    slug: string,
    created_on: string,
    created_by: number,
    modified_on: string,
    modified_by: number,
}

export default Image;
