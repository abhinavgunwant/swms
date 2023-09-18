import UserRole from './UserRole';

export interface Session {
    username: string;
    name: string;
    expiry: Date;
    role: UserRole;
}

export const defaultSession: Session = {
    username: '',
    name: '',
    expiry: new Date(),
    role: {
        id: -1,
        roleName: '',
        createImage: false,
        readImage: false,
        modifyImage: false,
        deleteImage: false,
        readRenditions: false,
        createRenditions: false,
        modifyRenditions: false,
        deleteRenditions: false,
        readProject: false,
        createProject: false,
        modifyProject: false,
        deleteProject: false,
        readUser: false,
        createUser: false,
        modifyUser: false,
        deleteUser: false,
        publish: false,
        publishAll: false,
        accessAllProjects: false,
    }
}

export default Session;

