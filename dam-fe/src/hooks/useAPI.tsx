import useWorkspaceStore from '../store/workspace/WorkspaceStore';

import {
    Project, SelectUserModel, UploadImage, Rendition, CreateUserPayload,
    UserListing, Role, Folder, Image,
} from '../models';

import { DEFAULT_PROJECT } from '../models/Project';

import { apiCall } from '../utils/misc';
import { NavigateFunction } from 'react-router-dom';

//const HOST = 'http://localhost:8080';
const PATH_PRE = `/api/admin`;

const APPLICATION_JSON = { 'Content-Type': 'application/json' };
const DEFAULT_ERROR_MESSAGE = 'Some unknown error occurred, please try again later';

export type SuccessType = {
    success: boolean,
    message: string,
    other?: object | undefined,
};

type SuccessAddRenditionType = Promise< SuccessType & {
    renditionMessages?: { id: number, message:string }[]
}>;

type SuccessRenditionType = Promise<SuccessType & { renditions: Rendition[] }>;

const success = (
    success: boolean, message: string, other: object | undefined = undefined
): SuccessType => {
    if (other === undefined) {
        return { success, message }
    }

    return { success, message, ...other };
};

type GetFolderType = Promise<{
    success: boolean, message?: string, folder?: Folder
}>;

const useAPI = (navigate?: NavigateFunction) => {
    const wsStore = useWorkspaceStore();

    return {
        login: async (username: string, password: string) => {
            const response = await apiCall(`${ PATH_PRE }/auth/login`, {
                method: 'POST',
                headers: APPLICATION_JSON,
                body: JSON.stringify({ username, password }),
                credentials: 'include',
            }, navigate, false);

            try {
                if (response.status === 200) {
                    const responseJson = await response.json();
            
                    if (responseJson.success) {
                        return { ...responseJson, status: response.status };
                    }
                }
            } catch (e) { console.log(e); }

            return success(false, 'Some error occured!');
        },

        /**
         * Gets a list of all the users at once.
         */
        getUsers: async () => {
            const response = await apiCall(`${ PATH_PRE }/users`, {}, navigate);

            try {
                const json = await response.json();
                // console.log('user list response: ', json);

                if (json) {
                    if (!json.message && !json.success) {
                        json.message = DEFAULT_ERROR_MESSAGE;
                    }

                    return json;
                }
            } catch (e) {
                console.log(e);
            }

            return success(false, DEFAULT_ERROR_MESSAGE);
        },

        createUser: async (user: CreateUserPayload) => {
            const response = await apiCall(`${ PATH_PRE }/user`, {
                headers: APPLICATION_JSON,
                method: 'POST',
                body: JSON.stringify(user),
            }, navigate);

            try {
                const json = await response.json();
                // console.log('user creation response: ', json);

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

            return success(false, DEFAULT_ERROR_MESSAGE);
        },

        /**
         * Edits user by replacing user attributes with ones in `user`.
         */
        editUser: async (user: UserListing) => {
            const response = await apiCall(`${ PATH_PRE }/user`, {
                headers: APPLICATION_JSON,
                method: 'PUT',
                body: JSON.stringify(user),
            }, navigate);

            if (response.status === 200) {
                return true;
            }

            return false;
        },

        /**
         * Gets all the roles in the system.
         */
        getRoles: async () => {
            const response = await apiCall(`${ PATH_PRE }/roles`, {}, navigate);

            if (response.status === 200) {
                try {
                    const roles: Role[] = await response.json();

                    return { success: true, roles };
                } catch (e) { console.log(e); }
            }

            return { success: false, roles: [] };
        },

        createEditRoles: async (role: Role, mode: 'new' | 'edit' = 'new') => {
            const response = await apiCall(`${ PATH_PRE }/role`, {
                headers: APPLICATION_JSON,
                method: mode === 'new' ? 'POST' : 'PUT',
                body: JSON.stringify(role),
            }, navigate);

            if (response.status === 200) {
                return true;
            }

            return false;
        },

        deleteRole: async (role: Role) => {
            const response = await apiCall(`${ PATH_PRE }/role`, {
                headers: APPLICATION_JSON,
                method: 'DELETE',
                body: JSON.stringify(role),
            }, navigate);

            if (response.status === 200) {
                return true;
            }

            return false;
        },

        getProject: async (projectId: number) => {
            const response = await apiCall(
                `${ PATH_PRE }/project/${ projectId }`, {}, navigate
            );

            if (response.status === 200) {
                return await response.json();
            }

            return DEFAULT_PROJECT;
        },

        /**
         * Gets the list of projects from dam api and sets it in store.
         */
        getProjects: async () => {
            const response = await apiCall(`${ PATH_PRE }/projects-for-user`, {}, navigate);

            if (response.status === 200) {
                const json = await response.json();
                wsStore.setProjectList(json.projects);
            }
        },

        /**
         * Gets the list of images from dam api and sets it in store.
         */
        getImages: async (slug:string, type:string='PROJECT') => {
            const response = await apiCall(
                `${ PATH_PRE }/get-children?type=${type}&path=${slug}`,
                {},
                navigate
            );

            if (response.status === 200) {
                const json = await response.json();
                wsStore.setImageList(json.images);
            }
        },

        /**
         * Gets the list of all children from dam api and sets it in store.
         */
        getChildren: async (slug:string, type:('folder' | 'project') ='project') => {
            // console.log('getChildren: slug: ', slug, 'type: ', type);

            const response = await apiCall(
                `${ PATH_PRE }/get-children?type=${type}&path=${slug}`,
                {},
                navigate
            );

            if (response.status === 200) {
                const json = await response.json();

                if (json.images) {
                    wsStore.setImageList(json.images);
                } else {
                    wsStore.setImageList([]);
                }

                if (json.folders) {
                    wsStore.setFolderList(json.folders);
                } else {
                    wsStore.setFolderList([]);
                }
            }

            if (response.status === 404) {
                wsStore.setImageList([]);
                wsStore.setFolderList([]);
            }

            if (response.status >= 500) {
                wsStore.setError(
                    "Some error occured, please ask admin to check logs."
                );
            }
        },

        /**
         * Gets a single image.
         */
        getImage: async (imageId: number) => {
            const response = await apiCall(`${ PATH_PRE }/image/${ imageId }`, {}, navigate);

            if (response.status === 200) {
                const json = await response.json();
                return json;
            }

            return false;
        },

        /**
         * Gets original blob and saves it as an object URL.
         */
        getImageBlobUrl: async (imageId: number): Promise<string> => {
            const response = await apiCall(
                `${ PATH_PRE }/image-file/${ imageId }`, {}, navigate
            );

            if (response.status === 200) {
                return URL.createObjectURL(await response.blob());
            }

            return '';
        },

        updateImage: async (image: Image) => {
            const response = await apiCall( `${ PATH_PRE }/image`, {
                headers: APPLICATION_JSON,
                method: 'PUT',
                body: JSON.stringify(image),
            }, navigate);

            if (response.status === 200) {
                const respJson = await response.json();

                return respJson;
            }

            try {
                let resp = await response.json();

                return resp;
            } catch (e) { console.log(e); }

            return success(false, 'Some error Occured! Please try again in some time!');
        },

        /**
         * Adds a new project.
         */
        addProject: async (project: Project) => {
            const response = await apiCall(
                `${ PATH_PRE }/project`, {
                method: 'POST',
                body: JSON.stringify(project),
                headers: APPLICATION_JSON,
            }, navigate);

            if (response.status === 200) {
                const json = await response.json();
                wsStore.setImageList(json.images);

                return true;
            }

            return await response.text();
        },

        /**
         * Deletes a project with id `pid`.
         *
         * @param pid project id
         */
        deleteProject: async (pid: number) => {
            const response = await apiCall(
                `${ PATH_PRE }/project/${ pid }`,
                { method: 'DELETE' },
                navigate
            );

            if (response.status === 200) {
                return true;
            }

            return await response.text();
        },

        uploadImage: async (uploadImg: UploadImage, payload: File) => {
            const formData = new FormData();

            formData.set('payload', payload);

            const response = await apiCall(`/api/image`, {
                method: 'POST',
                body: formData,
            }, navigate);

            if (response.status === 200) {
                const uploadResp = await response.json();

                // console.log('initial response: ', uploadResp);

                uploadImg.uploadId = uploadResp.uploadId;

                // TODO: Second request with the data and the above ID
                const resp2 = await apiCall(`${ PATH_PRE }/image-save`, {
                        method: 'POST',
                        headers: APPLICATION_JSON,
                        body: JSON.stringify(uploadImg),
                    },
                    navigate
                )

                const resp2Json = await resp2.json();

                // console.log('response after upload: ', resp2Json);

                if (resp2Json) {
                    return resp2Json;
                }

                return success(false, 'Some unknown error occurred');
            }

            try {
                return await response.json();
            } catch (_e) {
                return success(false, 'Some unknown error occurred');
            }
        },

        deleteImages: async (imageIDs: Array<number>) => {
            if (imageIDs.length === 0) {
                return success(false, 'No image selected!');
            }

            const response = await apiCall(
                `${ PATH_PRE }/image?id=${ imageIDs.join(',') }`, {
                method: 'DELETE',
                headers: APPLICATION_JSON,
            }, navigate);

            if (response.status === 200) {
                // return await response.json();
                return success(true, await response.text() || 'Success!');
            }

            return success(false, 'Some Unknown Error Occured');
        },

        /**
         * API to create renditions.
         */
        addRenditions: async (
            renditions: Rendition[], eager: boolean
        ): SuccessAddRenditionType  => {
            const response = await apiCall(`${ PATH_PRE }/renditions`, {
                method: 'POST',
                headers: APPLICATION_JSON,
                body: JSON.stringify({ renditions, eager }),
            }, navigate);

            if (response.status === 200) {
                return success(true, 'Success!');
            }

            const rJson = await response.json();

            // console.log('rJson', rJson);

            let renditionMessages: { id: number, message:string }[] = [];

            if (rJson.unsuccessful_renditions) {
                renditionMessages = rJson.unsuccessful_renditions;
            }

            return {
                success: false,
                message: 'Some Unknown Error Occured',
                renditionMessages,
            };
        },

        getRenditions: async (imageId: number): SuccessRenditionType => {
            const response = await apiCall(
                `${ PATH_PRE }/renditions?image-id=${ imageId }`,
                {},
                navigate
            );

            if (response.status === 200) {
                const { renditions }: ({ renditions: Rendition[]}) = await response.json();

                return {
                    success: true,
                    message: 'Success!',
                    renditions: renditions,
                };
            }

            return {
                success: false,
                message: 'Some Unknown Error Occured',
                renditions: [],
            };
        },

        deleteRendition: async(renditionId: number) => {
            const response = await apiCall(
                `${ PATH_PRE }/rendition/${ renditionId }`, {
                headers: APPLICATION_JSON,
                method: 'DELETE',
            }, navigate);

            try {
                return await response.json();
            } catch (e) {
                return success(false, 'Some error occured!');
            }
        },

        addFolder: async (folder: Folder) => {
            try {
                // console.log(folder);
                const response = await apiCall(`${ PATH_PRE }/folder/`, {
                    method: 'POST',
                    body: JSON.stringify(folder),
                    headers: APPLICATION_JSON,
                }, navigate);

                if (response.status === 200) {
                    return { success: true };
                }

                const message = await response.text();

                return success(false, message);
            } catch (e) {
                return { success: false };
            }
        },

        getFolder: async (folderId: number): GetFolderType => {
            const response = await apiCall(`${PATH_PRE}/folder/${folderId}/`, {
                headers: APPLICATION_JSON,
            }, navigate);

            if (response.status === 200) {
                const folder: Folder = await response.json();
                return { success: true, folder };
            }

            return success(false, 'NOT_FOUND');
        },

        deleteFolders: async (folderId: Array<number>) => {
            if (folderId.length === 0) {
                return success(false, 'Invalid Image!');
            }

            const response = await apiCall(
                `${ PATH_PRE }/folder?id=${ folderId }`, {
                method: 'DELETE',
                headers: APPLICATION_JSON,
            }, navigate);

            if (response.status === 200) {
                return success(true, await response.text() || 'Success!');
            }

            let message = 'Some Unknown Error Occured';

            try {
                message = await response.json();
            } catch (e) { console.log(e); }

            return success(false, message);
        },

        /**
         * Fetcher function used with the typeahead component.
         */
        userTypeahead: async (queryText: string) => {
            const response = await apiCall(
                `${ PATH_PRE }/search/user?name=${ queryText }`, {
                headers: APPLICATION_JSON,
            }, navigate);

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
            const response = await apiCall(
                `${ PATH_PRE }/project/validate-slug?slug=${ slug }`, {
                headers: APPLICATION_JSON,
            }, navigate);

            if (response.status === 200) {
                const valid = await response.json();

                return { valid, error: false };
            }

            return { error: true };
        }
    };
};

export default useAPI;

