import useUserStore from '../store/workspace/UserStore';
import useWorkspaceStore from '../store/workspace/WorkspaceStore';

const useAPI = () => {
    const userStore = useUserStore();
    const wsStore = useWorkspaceStore();

    return {
        /**
         * Gets the list of projects from dam api and assigns it to store.
         */
        getProjects: async () => {
            const response = await fetch('http://localhost:8080/api/admin/projects-for-user', {
                headers: {
                    //'Authorization': 'Bearer ' + userStore.sessionToken, // TODO: use this when jwt impl compeletes!
                    'Authorization': 'Bearer admin',
                }
            });

            if (response.status === 200) {
                const json = await response.json();
                wsStore.setProjectList(json.projects);
            }
        }
    }
};

export default useAPI;
