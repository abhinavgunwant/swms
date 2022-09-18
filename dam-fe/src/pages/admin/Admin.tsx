import { useState } from "react";

import {
    Typography, Button, Dialog, DialogActions, DialogContent,
    DialogContentText, DialogTitle, TextField
} from "@mui/material";

import { styled } from "@mui/material/styles";

import BreadCrumbs from "../../components/Breadcrumbs";

const StyledDialogContentText = styled(DialogContentText)`
    margin-bottom: 1.5rem;
`;

const Admin =  () => {
    const [ newProjectDialogOpen, setNewProjectDialogOpen ] = useState(false);
    const [ newProjectName, setNewProjectName ] = useState('');
    const [ newProjectDesc, setNewProjectDesc ] = useState('');

    const onProjectCreateClicked = () => {
        console.log("Project Name: ", newProjectName,
            "\nProject Description: ", newProjectDesc);

        setNewProjectDialogOpen(false);
    }

    const onNewProjectClicked = () => {
        setNewProjectDialogOpen(true);
        setNewProjectName('');
        setNewProjectDesc('');
    }

    return <div className="page page--new-image">
        <BreadCrumbs links={ [{text: 'Admin', to: '/admin' }] } />

        <Typography variant="h4">
            Project Controls
        </Typography>

        <Button onClick={ onNewProjectClicked }>New Project</Button>
        <Button>List Projects</Button>

        {/* New Project Dialog */}
        <Dialog open={ newProjectDialogOpen }>
            <DialogTitle>New Project</DialogTitle>

            <DialogContent>
                <StyledDialogContentText>All fields are mandatory</StyledDialogContentText>
                <TextField
                    id="project-name"
                    label="Project Name"
                    type="text"
                    margin="normal"
                    value={ newProjectName }
                    onChange={ (e) => setNewProjectName(e.target.value) }
                    fullWidth
                    autoFocus />

                <TextField
                    id="project-desc"
                    label="Description"
                    margin="normal"
                    type="text"
                    value={ newProjectDesc }
                    onChange={ (e) => setNewProjectDesc(e.target.value) }
                    fullWidth
                    multiline
                    rows={ 3 } />
            </DialogContent>

            <DialogActions>
                <Button onClick={ () => setNewProjectDialogOpen(false) }>Cancel</Button>
                <Button onClick={ onProjectCreateClicked }>Create</Button>
            </DialogActions>
        </Dialog>
    </div>
}

export default Admin;

