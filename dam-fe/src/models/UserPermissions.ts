interface UserPermissions {
    createImage: boolean,
    readImage: boolean,
    modifyImage: boolean,
    deleteImage: boolean,
    readRenditions: boolean,
    createRenditions: boolean,
    modifyRenditions: boolean,
    deleteRenditions: boolean,
    readProject: boolean,
    createProject: boolean,
    modifyProject: boolean,
    deleteProject: boolean,
    readUser: boolean,
    createUser: boolean,
    modifyUser: boolean,
    deleteUser: boolean,
    publish: boolean,
    publishAll: boolean,
    accessAllProjects: boolean,
}

export default UserPermissions;
