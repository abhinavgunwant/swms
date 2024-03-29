import React from "react";
import {
  BrowserRouter,
  Route,
  RouteProps,
  Routes,
} from "react-router-dom";
import Home from "../pages/Home";
import NewFolder from "../pages/workspace/new-folder/NewFolder";
import NewImage from "../pages/workspace/new-image/NewImage";
import ImageDetails from "../pages/workspace/image/ImageDetails";
import FolderDetails from "../pages/workspace/folder/FolderDetails";
import Project from "../pages/workspace/project/Projects";
import NewProject from "../pages/workspace/project/NewProject";
import Workspace from "../pages/workspace/Workspace";
import Admin from "../pages/admin/Admin";
import Users, { Create, Edit, UserRoles, NewEditRole } from "../pages/admin/users";

//import { getImagePathFromURL } from '../utils/PathUtils';
import SelectUsers from "./SelectUsers";

const Router = ():React.ReactElement<RouteProps> => {
    //const imagePath = getImagePathFromURL();
    return <BrowserRouter>
        <Routes>
            <Route path="/" element={ <Home /> } />
            <Route path="/workspace" element={ <Project /> } />
            <Route path="/admin" element={ <Admin /> } />
            <Route path="/admin/users" element={ <Users /> } />
            <Route path="/admin/users/create" element={ <Create /> } />
            <Route path="/admin/users/edit" element={ <Edit /> } />
            <Route path="/admin/roles" element={ <UserRoles /> } />
            <Route path="/admin/roles/create" element={ <NewEditRole mode="new" /> } />
            <Route path="/admin/roles/edit" element={ <NewEditRole mode="edit" /> } />
            <Route path="/workspace/tree/*"
                Component={ () => <Workspace key={Math.random()} /> }
            />
        {/*<Route path="/workspace/tree/:projectSlug/:imageSlug" element={ <Workspace key='2' /> } />*/}
            <Route path="/workspace/image/:imageId" element={ <ImageDetails /> } />
            <Route path="/workspace/folder/:folderId" element={ <FolderDetails /> } />
            <Route path="/workspace/new-image" element={ <NewImage /> } />
            <Route path="/workspace/new-folder" element={ <NewFolder /> } />
            <Route path="/workspace/new-project" element={ <NewProject /> } />
            <Route path="/test" element={ <SelectUsers /> } />
            <Route path="*" element={ <h1>404: Page not found</h1> } />
        </Routes>
    </BrowserRouter>;
}

export default Router;

