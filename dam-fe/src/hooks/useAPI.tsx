import useUserStore from '../store/workspace/UserStore';
import useWorkspaceStore from '../store/workspace/WorkspaceStore';
import Project from '../models/Project';
import SelectUserModel from '../models/SelectUserModel';
import UploadImage from '../models/UploadImage';
import Rendition from '../models/Rendition';
import CreateUserPayload from '../models/CreateUserPayload';
import UserListing from '../models/UserListing';
import Role from '../models/Role';

const HOST = 'http://localhost:8080';
const DEFAULT_ERROR_MESSAGE = 'Some unknown error occurred, please try again later';

// const DEFAULT_ERROR_RESPONSE = {
//     success: false,
//     message: 'Some unknown error occurred'
// };

const useAPI = () => {
    const userStore = useUserStore();
    const wsStore = useWorkspaceStore();

    return {
        /**
         * Gets a list of all the users at once.
         */
        getUsers: async () => {
            const response = await fetch('http://localhost:8080/api/admin/users', {
                headers: {
                    //'Authorization': 'Bearer ' + userStore.sessionToken, // TODO: use this when jwt impl compeletes!
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                },
            });

            try {
                const json = await response.json();
                console.log('user list response: ', json);

                if (json) {
                    if (!json.message && !json.success) {
                        json.message = DEFAULT_ERROR_MESSAGE;
                    }

                    return json;
                }
            } catch (e) {
                console.log(e);
            }

            return {
                success: false,
                message: DEFAULT_ERROR_MESSAGE,
            };
        },

        createUser: async (user: CreateUserPayload) => {
            const response = await fetch('http://localhost:8080/api/admin/user', {
                headers: {
                    //'Authorization': 'Bearer ' + userStore.sessionToken, // TODO: use this when jwt impl compeletes!
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                    'Content-Type': 'application/json',
                },
                method: 'POST',
                body: JSON.stringify(user)
            });

            try {
                const json = await response.json();
                console.log('user creation response: ', json);

                if (json) {
                    if (!json.message) {
                        if (json.success) {
                            json.message = 'User Created!';
                        } else {
                            json.message = DEFAULT_ERROR_MESSAGE;
                        }
                    }

                    return json;
                }
            } catch (e) {
                console.log(e);
            }

            return {
                success: false,
                message: DEFAULT_ERROR_MESSAGE,
            };
        },

        /**
         * Edits user by replacing user attributes with ones in `user`.
         */
        editUser: async (user: UserListing) => {
            const response = await fetch('http://localhost:8080/api/admin/user', {
                headers: {
                    //'Authorization': 'Bearer ' + userStore.sessionToken, // TODO: use this when jwt impl compeletes!
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                    'Content-Type': 'application/json',
                },
                method: 'PUT',
                body: JSON.stringify(user)
            });

            if (response.status === 200) {
                return true;
            }

            return false;
        },

        /**
         * Gets all the roles in the system.
         */
        getRoles: async () => {
            const response = await fetch('http://localhost:8080/api/admin/roles', {
                headers: {
                    //'Authorization': 'Bearer ' + userStore.sessionToken, // TODO: use this when jwt impl compeletes!
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                },
            });

            if (response.status === 200) {
                try {
                    const roles: Role[] = await response.json();

                    return { success: true, roles };
                } catch (e) {
                    console.log(e);
                }
            }

            return { success: false, roles: [] };
        },

        createEditRoles: async (role: Role, mode: 'new' | 'edit' = 'new') => {
            const response = await fetch('http://localhost:8080/api/admin/role', {
                headers: {
                    //'Authorization': 'Bearer ' + userStore.sessionToken, // TODO: use this when jwt impl compeletes!
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                    'Content-Type': 'application/json',
                },
                method: mode === 'new' ? 'POST' : 'PUT',
                body: JSON.stringify(role),
            });

            if (response.status === 200) {
                return true;
            }

            return false;
        },

        deleteRole: async (role: Role) => {
            const response = await fetch('http://localhost:8080/api/admin/role', {
                headers: {
                    //'Authorization': 'Bearer ' + userStore.sessionToken, // TODO: use this when jwt impl compeletes!
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                    'Content-Type': 'application/json',
                },
                method: 'DELETE',
                body: JSON.stringify(role),
            });

            if (response.status === 200) {
                return true;
            }

            return false;
        },

        /**
         * Gets the list of projects from dam api and sets it in store.
         */
        getProjects: async () => {
            const response = await fetch('http://localhost:8080/api/admin/projects-for-user', {
                headers: {
                    //'Authorization': 'Bearer ' + userStore.sessionToken, // TODO: use this when jwt impl compeletes!
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                }
            });

            if (response.status === 200) {
                const json = await response.json();
                wsStore.setProjectList(json.projects);
            }
        },

        /**
         * Gets the list of images from dam api and sets it in store.
         */
        getImages: async (slug:string, type:string='PROJECT') => {
            const response = await fetch(
                `${HOST}/api/admin/get-children?type=${type}&slug=${slug}`, {
                headers: {
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                }
            });

            if (response.status === 200) {
                const json = await response.json();
                wsStore.setImageList(json.images);
            }
        },

        /**
         * Adds a new project.
         */
        addProject: async (project: Project) => {
            const response = await fetch(
                `${HOST}/api/admin/project`, {
                method: 'POST',
                body: JSON.stringify(project),
                headers: {
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                    'Content-Type': 'application/json',
                }
            });

            if (response.status === 200) {
                const json = await response.json();
                wsStore.setImageList(json.images);

                return true;
            }

            return await response.text();
        },

        uploadImage: async (uploadImg: UploadImage, payload: File) => {
            const formData = new FormData();

            formData.set('payload', payload);

            const response = await fetch(
                `${HOST}/api/image`, {
                method: 'POST',
                headers: {
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                },
                body: formData,
            });

            if (response.status === 200) {
                const uploadResp = await response.json();

                console.log('initial response: ', uploadResp);

                uploadImg.uploadId = uploadResp.uploadId;

                // TODO: Second request with the data and the above ID
                const resp2 = await fetch(
                    `${ HOST }/api/admin/image-save`, {
                        method: 'POST',
                        headers: {
                            'Authorization': 'Bearer '
                                + userStore.sessionToken,
                            'Content-Type': 'application/json',
                        },
                        body: JSON.stringify(uploadImg),
                    },
                )

                const resp2Json = await resp2.json();

                console.log('response after upload: ', resp2Json);

                if (resp2Json) {
                    return resp2Json;
                }

                return {
                    success: false,
                    message: 'Some unknown error occurred'
                };
            }

            try {
                return await response.json();
            } catch (_e) {
                return {
                    success: false,
                    message: 'Some unknown error occurred'
                };
            }
        },

        /**
         * API to create renditions.
         *
         * TODO: Implement error handling.
         */
        addRenditions: async (renditions: Rendition[], eager: boolean) => {
            const response = await fetch(
                `${HOST}/api/admin/renditions`, {
                method: 'POST',
                headers: {
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ renditions, eager }),
            });

            if (response.status === 200) {
                // return await response.json();
                return { success: true, message: 'Success!' };
            }

            return { success: false, message: 'Some Unknown Error Occured' };
        },

        /**
         * Fetcher function used with the typeahead component.
         */
        userTypeahead: async (queryText: string) => {
            const response = await fetch(
                `${HOST}/api/admin/search/user?name=${ queryText }`, {
                method: 'GET',
                headers: {
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                    'Content-Type': 'application/json',
                }
            });

            if (response.status === 200) {
                const list: SelectUserModel[] = await response.json();

                return list;
            }

            return [];
        },

        /**
         * Validates slug for new project.
         */
        validateProjectSlug: async (slug: string) => {
            const response = await fetch (
                `${ HOST }/api/admin/project/validate-slug?slug=${ slug }`, {
                method: 'GET',
                headers: {
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                    'Content-Type': 'application/json',
                }
            });

            if (response.status === 200) {
                const valid = await response.json();

                return { valid, error: false };
            }

            return { error: true };
        }
    };
};

export default useAPI;

