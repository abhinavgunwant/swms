import useUserStore from '../store/workspace/UserStore';
import useWorkspaceStore from '../store/workspace/WorkspaceStore';
import Project from '../models/Project';
import SelectUserModel from '../models/SelectUserModel';

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
         * 
         * TODO: Add a project model
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
    };
};

export default useAPI;

