// import Project from '../models/Project';
// import useWorkspaceStore from '../store/workspace/WorkspaceStore';
import useUserStore from '../store/workspace/UserStore';

const getProjects = (token: string) => async () => {
    const response = await fetch('http://localhost:8080/api/admin/projects-for-user', {
        headers: {
            'Authorization': 'Bearer ' + token,
        }
    });

    if (response.status === 200) {
        const json = await response.json();
        console.log('received json: ', json);
    }

    return [];
}

const useAPI = () => {
    // const store = useWorkspaceStore();
    const userStore = useUserStore();

    return {
        getProjects: getProjects(userStore.sessionToken),
    }
};

export default useAPI;
