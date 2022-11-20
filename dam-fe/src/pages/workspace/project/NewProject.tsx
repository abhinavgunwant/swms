import { useState, ChangeEvent, useTransition } from 'react';

import {
    Checkbox, Typography, Grid, TextField, Button, Box
} from '@mui/material';

import Breadcrumbs from "../../../components/Breadcrumbs";

import { styled } from '@mui/material/styles';

const StyledTextField = styled(TextField)`
    width: 100%;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
`;

const StyledGrid = styled(Grid)`
    margin-top: 1rem;
`;

const CheckboxBox = styled(Box)`
    display: flex;
    justify-content: center;
    align-items: center;
    height: 4rem;
`;

const NewProject = () => {
    const [ _transition, startTransition ] = useTransition();
    const [ title, setTitle ] = useState<string>('');
    const [ slug, setSlug ] = useState<string>('');
    const [ autoGenerateSlug, setAutoGenerateSlug ] = useState<boolean>(true);
    const [ enableSave, setEnableSave ] = useState<boolean>(false);

    const generateSlug = (slg: string) => {
        if (slg.trim()) {
            slg = slg.trim().replaceAll(' ', '-');

            if (slg) {
                startTransition(() => setSlug(slg.toLowerCase()));
            }
        }
    }
    
    /**
     * Validates project details before creating the new project.
     * @returns true if project details are valid.
     * 
     * TODO: implement it!
     */
    const validate = () => {
        return true;
    }

    const onTitleChanged = (e: ChangeEvent<HTMLInputElement>) => {
        let slg = e.target.value || '';
        setTitle(slg);

        if (autoGenerateSlug) {
            generateSlug(slg);
        }

        startTransition(() => {
            if (validate()) {
                setEnableSave(true);
            }
        });
    };

    const onCheckboxChecked = (event: ChangeEvent<HTMLInputElement>) => {
        startTransition(() => {
            generateSlug(title);
            setAutoGenerateSlug(event.target.checked);
        });
    }

    const onSlugChanged = (event: ChangeEvent<HTMLInputElement>) => {
        startTransition(() => setSlug(event.target.value));
    }

    return <div className="page page--new-image">
        <Breadcrumbs links={[{ text: 'Workspace', to: '/workspace' }, 'New Folder']} />

        <Typography variant="h5">
            New Project
        </Typography>

        <Typography variant="subtitle1">
            Enter below details to create new folder.
        </Typography>

        <StyledGrid container>
            <Grid item xs={10}>
                <StyledTextField
                    required
                    disabled={ autoGenerateSlug }
                    value={ slug }
                    onChange={ onSlugChanged }
                    label="Slug"
                    defaultValue="/" />
            </Grid>
            <Grid item xs={2}>
                <CheckboxBox>
                    <Checkbox
                        onChange={ onCheckboxChecked }
                        checked={ autoGenerateSlug } />

                        Auto Generate Slug
                </CheckboxBox>
            </Grid>
            <StyledTextField
                required
                label="Project Title"
                value={ title }
                onChange={ onTitleChanged } />
            <StyledTextField multiline label="Project Details" rows={3} />
        </StyledGrid>

        <Button
            variant="contained"
            style={{ marginRight: '0.5rem' }}
            disabled={ !enableSave }>
            Save
        </Button>
        <Button variant="outlined">Cancel</Button>
    </div>
}

export default NewProject;
