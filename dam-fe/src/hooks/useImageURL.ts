import useWorkspaceStore from '../store/workspace/WorkspaceStore';

import { Image, Project, DEFAULT_PROJECT } from '../models';
import useAPI from './useAPI';

/**
 * Gets the URL of the rendition (or the default rendition of the image).
 *
 * @param projectId the project's id
 * @param imageId the image's id (the image that you want to get the url for)
 * @param slug the slug of the rendition 
 */
const useImageURL = () => {
    const wsStore = useWorkspaceStore();
    const { getImage, getProject } = useAPI();

    return async (projectId: number, imageId: number, slug: string) => {
        let projectFetched: boolean = false;
        let image: Image | null = null;
        let project: Project = DEFAULT_PROJECT;

        if (!projectId && imageId) {
            image = await getImage(imageId);
        }

        if (wsStore.currentProject.id === 0 && image) {
            project = await getProject(image.projectId);

            if (project.id !== 0) {
                wsStore.setCurrentProject(project);
                projectFetched = true;
            }
        } else {
            project = wsStore.currentProject;
        }

        let relPath = wsStore.currentPath.replace('workspace/tree/', '');

        // in case the page is refreshed, relPath will be empty string
        if (projectFetched && project.id !== 0 || !relPath) {
            relPath = project.slug + '/' + relPath;
        }

        const url = (
            `/api/image/${ relPath }/${ slug }`
        ).replaceAll('//', '/');

        return url;
    }
}

export default useImageURL;

