import { useState, ChangeEvent, useTransition } from 'react';
import { useNavigate } from 'react-router-dom';

import {
    Checkbox, Typography, Grid, TextField, Button, Box, FormGroup,
    FormControlLabel, Alert
} from '@mui/material';

import Breadcrumbs from "../../../components/Breadcrumbs";
import SelectUsers from '../../../components/SelectUsers';

import useAPI from '../../../hooks/useAPI';

import { styled } from '@mui/material/styles';

const StyledTextField = styled(TextField)`
    width: 100%;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
`;

const StyledGrid = styled(Grid)`
    margin-top: 1rem;
    margin-bottom: 1rem;
`;

const AutoGenerateFormGroup = styled(Box)`
    display: flex;
    justify-content: center;
    align-items: center;
    height: 4rem;
`;

const NewProject = () => {
    const [ _transition, startTransition ] = useTransition();
    const [ title, setTitle ] = useState<string>('');
    const [ slug, setSlug ] = useState<string>('');
    const [ description, setDescription ] = useState<string>('');
    const [ autoGenerateSlug, setAutoGenerateSlug ] = useState<boolean>(true);
    const [ restrictAccess, setRestrictAccess ] = useState<boolean>(false);
    const [ enableSave, setEnableSave ] = useState<boolean>(false);
    const [ showError, setShowError ] = useState<boolean>(false);
    const [ error, setError ] = useState<string>('');

    const { addProject } = useAPI();
    const navigate = useNavigate();

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

    const onRestrictAccessCheckboxChecked = (event: ChangeEvent<HTMLInputElement>) =>
        startTransition(() => setRestrictAccess(event.target.checked));

    const onAutogenerateCheckboxChecked = (event: ChangeEvent<HTMLInputElement>) => {
        startTransition(() => {
            generateSlug(title);
            setAutoGenerateSlug(event.target.checked);
        });
    }

    const onSlugChanged = (event: ChangeEvent<HTMLInputElement>) =>
        setSlug(event.target.value);

    const onDescriptionChanged = (event: ChangeEvent<HTMLInputElement>) =>
        setDescription(event.target.value);

    /**
     * Hits the add project API.
     */
    const onSave = async () => {
        const today: string = (new Date()).toISOString();

        const errors: boolean|string = await addProject({
            id: 0,
            name: title,
            slug,
            description,
            restrictUsers: restrictAccess,
            createdBy: 0,
            modifiedBy: 0,
            createdOn: today,
            modifiedOn: today,
        });

        if (errors === true) {
            navigate("/workspace")
        }

        startTransition(() => {
            if (typeof errors === 'string') {
                setError(errors);
            } else {
                setError('Some Error Occured!');
            }

            setShowError(true);
        });
    }

    const onCancel = () => {
        navigate("/workspace")
    }

    return <div className="page page--new-image">
        <Breadcrumbs
            links={[
                { text: 'Workspace', to: '/workspace' },
                'New Folder'
            ]} />

        <Typography variant="h5">New Project</Typography>

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
                    label="Project Slug"
                    defaultValue="/" />
            </Grid>

            <Grid item xs={2}>
                <AutoGenerateFormGroup>
                    <FormControlLabel
                        control={
                            <Checkbox
                                onChange={ onAutogenerateCheckboxChecked }
                                checked={ autoGenerateSlug } />
                        }
                        label="Auto Generate Slug" />
                </AutoGenerateFormGroup>
            </Grid>

            <StyledTextField
                required
                label="Project Name"
                value={ title }
                onChange={ onTitleChanged } />

            <StyledTextField
                multiline
                label="Project Description"
                rows={3}
                value={ description }
                onChange={ onDescriptionChanged } />

            <Grid item xs={12}>
                <FormGroup>
                    <FormControlLabel
                        control={
                            <Checkbox
                                onChange={ onRestrictAccessCheckboxChecked }
                                checked={ restrictAccess } />
                        }
                        label="Restrict Access" />
                </FormGroup>
            </Grid>

            {
                restrictAccess &&
                <Grid item xs={12} md={6}>
                    <SelectUsers
                        placeholder="Type names to add to the access list for this project"
                        title="Restrict access for:" />
                </Grid>
            }

            { showError && <Alert severity="error">{ error }</Alert> }
        </StyledGrid>

        <Button
            variant="contained"
            style={{ marginRight: '0.5rem' }}
            disabled={ !enableSave }
            onClick={ onSave }>
            Save
        </Button>

        <Button variant="outlined" onClick={ onCancel }>Cancel</Button>
    </div>
}

export default NewProject;
