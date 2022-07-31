import { useState } from 'react';

import Typography from '@mui/material/Typography';
import Grid from '@mui/material/Grid';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';
import Box from '@mui/material/Box';
import UploadFileIcon from '@mui/icons-material/UploadFile';
import ContentCopyIcon from '@mui/icons-material/ContentCopy';

import { styled } from '@mui/material/styles';

import Breadcrumbs from "../../../components/Breadcrumbs";
import { IconButton } from '@mui/material';

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

const NewImage = () => {
    const [ slug, setSlug ] = useState();

    const onTitleChanged = (e: any) => {
        let slg = e.target.value;

        if (typeof slg === 'string' && slg.trim()) {
            slg = slg.trim().replaceAll(' ', '-');

            if (slg) {
                setSlug(slg.toLowerCase());
            }
        }
    }

    return <div className="page page--new-image">
        <Breadcrumbs links={[{ text: 'Workspace', to: '/workspace' }, 'New Image']} />

        <Typography variant="h5">
            New Image
        </Typography>

        <Typography variant="subtitle1">
            Enter below details to create new image.
        </Typography>

        <StyledGrid container>
            <Grid item xs={12} lg={6}>
                <StyledTextField label="Folder Path" defaultValue="/" required />
                <StyledTextField label="Image Title" onChange={ onTitleChanged } required />
                <StyledTextField label="Image Details" rows={3} multiline />
                
            </Grid>

            <Grid item xs={12} lg={6} style={{padding: '0.5rem 1rem'}}>
                <ImagePreview>
                    <Button startIcon={ <UploadFileIcon /> } color="secondary">
                        Upload Image
                    </Button>
                </ImagePreview>
            </Grid>

            <Grid container xs={6}>
                <Grid item xs={11}>
                    <StyledTextField
                        label="Slug"
                        value={ slug }
                        InputLabelProps={{ shrink: true }}
                        helperText={
                            'The slug should be a globally unique sequence of'
                            + ' characters. Must be upto 128 characters long.'
                        }
                        required />
                </Grid>
                <Grid item xs={1}>
                    <IconButton color="primary" style={{
                            marginTop: '1rem',
                            marginLeft: '1rem',
                        }}>
                        <ContentCopyIcon />
                    </IconButton>
                </Grid>
            </Grid>

            <Grid container xs={6}>
                <Grid item xs={11}>
                    <StyledTextField
                        label="Image UUID"
                        InputLabelProps={{ shrink: true }}
                        disabled />
                </Grid>
                <Grid item xs={1}>
                    <IconButton color="primary" style={{
                            marginTop: '1rem',
                            marginLeft: '1rem',
                        }}>
                        <ContentCopyIcon />
                    </IconButton>
                </Grid>
            </Grid>

        </StyledGrid>

        <Button variant="contained" style={{marginRight: '0.5rem'}} disabled>Save</Button>
        <Button variant="outlined">Cancel</Button>
    </div>
}

export default NewImage;
