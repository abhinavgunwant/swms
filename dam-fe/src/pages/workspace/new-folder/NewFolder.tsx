import Typography from '@mui/material/Typography';
import Grid from '@mui/material/Grid';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';
import Box from '@mui/material/Box';
import UploadFileIcon from '@mui/icons-material/UploadFile';

import { styled } from '@mui/material/styles';

import Breadcrumbs from "../../../components/Breadcrumbs";

const StyledTextField = styled(TextField)`
    width: 100%;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
`;

const StyledGrid = styled(Grid)`
    margin-top: 1rem;
`;

const ImagePreview = styled(Box)`
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: #dddddd;
    width: 100%;
    min-height: 240px;
    border-radius: 1rem;
`

const NewFolder = () => {
    return <div className="page page--new-image">
        <Breadcrumbs links={[{ text: 'Workspace', to: '/workspace' }, 'New Image']} />

        <Typography variant="h5">
            New Folder
        </Typography>

        <Typography variant="subtitle1">
            Enter below details to create new folder.
        </Typography>

        <StyledGrid container>
            <Grid item xs={12}>
                <StyledTextField required label="Upload Path" defaultValue="/" />
                <StyledTextField required label="Folder Title" />
                <StyledTextField multiline label="Folder Details" rows={3} />
            </Grid>
        </StyledGrid>

        <Button variant="contained" style={{marginRight: '0.5rem'}} disabled>Save</Button>
        <Button variant="outlined">Cancel</Button>
    </div>
}

export default NewFolder;
