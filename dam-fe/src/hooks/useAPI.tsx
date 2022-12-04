import useUserStore from '../store/workspace/UserStore';
import useWorkspaceStore from '../store/workspace/WorkspaceStore';
import Project from '../models/Project';
import SelectUserModel from '../models/SelectUserModel';
import Image from '../models/Image';

const HOST = 'http://localhost:8080'

const useAPI = () => {
    const userStore = useUserStore();
    const wsStore = useWorkspaceStore();

    return {
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

        addImage: async (image: Image, payload: File) => {
            const formData = new FormData();

            formData.set('name', image.name);
            formData.set('title', image.title);
            formData.set('project_id', image.projectId.toString());
            formData.set('folder_id', image.folderId.toString());
            formData.set('payload', payload);

            const response = await fetch(
                `${HOST}/api/image`, {
                method: 'POST',
                headers: {
                    'Authorization': 'Bearer ' + userStore.sessionToken,
                    //'Content-Type': 'multipart/form-data',
                },
                body: formData,
            });

            if (response.status === 200) {
                return { success: true, message: '' };
            }

            return { success: false, message: await response.text() };
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

