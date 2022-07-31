import React from "react";
import {
  BrowserRouter,
  Route,
  Routes,
  useLocation
} from "react-router-dom";
import Home from "../pages/Home";
import NewFolder from "../pages/workspace/new-folder/NewFolder";
import NewImage from "../pages/workspace/new-image/NewImage";
import Project from "../pages/workspace/project/Projects";
import Workspace from "../pages/workspace/Workspace";

import { getImagePathFromURL } from '../utils/PathUtils';

const Router = ():React.ReactElement => {
    const imagePath = getImagePathFromURL();
    return <BrowserRouter>
        <Routes>
            <Route path="/" element={ <Home /> } />
            <Route path="/workspace" element={ <Project /> } />
            <Route path="/workspace/tree/:projectSlug" element={ <Workspace /> } />
            <Route path={`/workspace/tree/:projectSlug/${ imagePath + (imagePath ? '/' : '') }:imageSlug`} element={ <Workspace /> } />
            <Route path="/workspace/new-image" element={ <NewImage /> } />
            <Route path="/workspace/new-folder" element={ <NewFolder /> } />
        </Routes>
    </BrowserRouter>;
}

export default Router;
