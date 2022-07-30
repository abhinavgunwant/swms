import React from "react";
import {
  BrowserRouter,
  Route,
  Routes,
  Link
} from "react-router-dom";
import Home from "../pages/Home";
import NewFolder from "../pages/workspace/new-folder/NewFolder";
import NewImage from "../pages/workspace/new-image/NewImage";
import Workspace from "../pages/workspace/Workspace";

const Router = ():React.ReactElement => {
    return <BrowserRouter>
        <Routes>
            <Route path="/" element={ <Home /> } />
            <Route path="/workspace" element={ <Workspace /> } />
            <Route path="/workspace/new-image" element={ <NewImage /> } />
            <Route path="/workspace/new-folder" element={ <NewFolder /> } />
        </Routes>
    </BrowserRouter>;
}

export default Router;
