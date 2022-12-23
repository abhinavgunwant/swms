import React from "react";
import {
  BrowserRouter,
  Route,
  Routes,
} from "react-router-dom";
import Home from "../pages/Home";
import NewFolder from "../pages/workspace/new-folder/NewFolder";
import NewImage from "../pages/workspace/new-image/NewImage";
import Project from "../pages/workspace/project/Projects";
import NewProject from "../pages/workspace/project/NewProject";
import Workspace from "../pages/workspace/Workspace";
import Admin from "../pages/admin/Admin";
import Users, { Create } from "../pages/admin/users";

import { getImagePathFromURL } from '../utils/PathUtils';
import SelectUsers from "./SelectUsers";

const Router = ():React.ReactElement => {
    const imagePath = getImagePathFromURL();
    return <BrowserRouter>
        <Routes>
            <Route path="/" element={ <Home /> } />
            <Route path="/workspace" element={ <Project /> } />
            <Route path="/admin" element={ <Admin /> } />
            <Route path="/admin/users" element={ <Users /> } />
            <Route path="/admin/users/create" element={ <Create /> } />
            <Route path="/workspace/tree/:projectSlug" element={ <Workspace /> } />
            <Route path={`/workspace/tree/:projectSlug/${ imagePath + (imagePath ? '/' : '') }:imageSlug`} element={ <Workspace /> } />
            <Route path="/workspace/new-image" element={ <NewImage /> } />
            <Route path="/workspace/new-folder" element={ <NewFolder /> } />
            <Route path="/workspace/new-project" element={ <NewProject /> } />
            <Route path="/test" element={ <SelectUsers /> } />
        </Routes>
    </BrowserRouter>;
}

export default Router;

