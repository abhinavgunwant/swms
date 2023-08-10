export interface UserPermissions {
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

export const UserPermissionsKeyToNameMapping: { [key in keyof UserPermissions]: string } = {
    createImage: 'Create Image',
    readImage: 'Read Image',
    modifyImage: 'Modify Image',
    deleteImage: 'Delete Image',
    readRenditions: 'Read Renditions',
    createRenditions: 'Create Renditions',       
    modifyRenditions: 'Modify Renditions', 
    deleteRenditions: 'Delete Renditions', 
    readProject: 'Read Project',
    createProject: 'Create Project',
    modifyProject: 'Modify Project',
    deleteProject: 'Delete Project',
    readUser: 'Read User',
    createUser: 'Create User',
    modifyUser: 'Modify User',
    deleteUser: 'Delete User',
    publish: 'Publish',
    publishAll: 'Publish All',
    accessAllProjects: 'Access All Projects',
};

export class UserPermissionsImpl implements UserPermissions {
    createImage: boolean;
    readImage: boolean;
    modifyImage: boolean;
    deleteImage: boolean;
    readRenditions: boolean;
    createRenditions: boolean;
    modifyRenditions: boolean;
    deleteRenditions: boolean;
    readProject: boolean;
    createProject: boolean;
    modifyProject: boolean;
    deleteProject: boolean;
    readUser: boolean;
    createUser: boolean;
    modifyUser: boolean;
    deleteUser: boolean;
    publish: boolean;
    publishAll: boolean;
    accessAllProjects: boolean;

    constructor () {
        this.createImage = false;
        this.readImage = false;
        this.modifyImage = false;
        this.deleteImage = false;
        this.readRenditions = false;
        this.createRenditions = false;
        this.modifyRenditions = false;
        this.deleteRenditions = false;
        this.readProject = false;
        this.createProject = false;
        this.modifyProject = false;
        this.deleteProject = false;
        this.readUser = false;
        this.createUser = false;
        this.modifyUser = false;
        this.deleteUser = false;
        this.publish = false;
        this.publishAll = false;
        this.accessAllProjects = false;
    }
}

export default UserPermissions;

