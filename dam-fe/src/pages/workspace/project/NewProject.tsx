import { useEffect, useState, useCallback, ChangeEvent, useTransition } from 'react';
import { useNavigate } from 'react-router-dom';

import {
    Checkbox, Typography, Grid, TextField, Button, Box, FormGroup,
    FormControlLabel
} from '@mui/material';

import { throttle } from 'lodash';

import { Breadcrumbs, SelectUsers, Error } from "../../../components";

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
    const [ showUnknownError, setShowUnknownError ] = useState<boolean>(false);
    const [ showSlugError, setShowSlugError ] = useState<boolean>(false);
    const [ showSlugValidError, setShowSlugValidError ]
        = useState<boolean>(false);
    const [ showValidatingSlugMessage, setValidatingSlugMessage]
        = useState<boolean>(false);
    const [ showTitleError, setShowTitleError ] = useState<boolean>(false);
    const [ titleEdited, setTitleEdited ] = useState<boolean>(false);
    const [ slugEdited, setSlugEdited ] = useState<boolean>(false);

    const navigate = useNavigate();
    const { addProject, validateProjectSlug } = useAPI(navigate);

    const generateSlug = (slg: string) => {
        if (slg.trim()) {
            slg = slg.trim().replaceAll(' ', '-');

            if (slg) {
                startTransition(() => {
                    setSlug(slg.toLowerCase());

                    if (!slugEdited) {
                        setSlugEdited(true);
                    }

                    if (slg) {
                        setShowSlugError(false);
                        validateSlug(slg);
                    }
                });
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

        if (!titleEdited) {
            startTransition(() => setTitleEdited(true));
        }

        if (slg) {
            startTransition(() => setShowTitleError(false));
        }
    };

    const onTitleBlurred = () => {
        if (autoGenerateSlug) {
            generateSlug(title);
        }
    };

    const onRestrictAccessCheckboxChecked = (event: ChangeEvent<HTMLInputElement>) =>
        startTransition(() => setRestrictAccess(event.target.checked));

    const onAutogenerateCheckboxChecked = (event: ChangeEvent<HTMLInputElement>) => {
        startTransition(() => {
            generateSlug(title);
            setAutoGenerateSlug(event.target.checked);
        });
    }

    const onSlugChanged = (event: ChangeEvent<HTMLInputElement>) => {
        setSlug(event.target.value);
        validateSlug(event.target.value);
    }

    const onDescriptionChanged = (event: ChangeEvent<HTMLInputElement>) =>
        setDescription(event.target.value);

    const validateSlug = useCallback(throttle(
        async (s: string) => {
            console.log('Validating for: ', s);
            const response = await validateProjectSlug(s);

            if (response?.valid) {
                setShowSlugValidError(false);
            } else {
                setShowSlugValidError(true);
            }
        },
        1000,
        { trailing: true, leading: false }
    ), []);

    /**
     * Hits the add project API.
     *
     * TODO: Show a notification that a new project has been created.
     */
    const onSave = async () => {
        const today: string = (new Date()).toISOString();

        const response: boolean|string = await addProject({
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

        if (typeof response === 'boolean' && response === true) {
            navigate("/workspace");

            return;
        }
        
        startTransition(() => {
            setShowUnknownError(true);
        });
    }

    const getSlug = () => slug;

    const onCancel = () => {
        navigate("/workspace")
    }

    useEffect(() => {
        if (titleEdited && title === '') {
            startTransition(() => setShowTitleError(true));
        }

        if (slugEdited && slug === '') {
            startTransition(() => setShowSlugError(true));
        }
    }, [ title, slug ]);

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
                    color={
                        slugEdited && showSlugValidError ?
                            'warning' : undefined
                    }
                    focused={ slugEdited && showSlugValidError }
                    sx={slugEdited && showSlugValidError ?
                        {
                        "& .MuiInputBase-root.Mui-disabled": {
                            "& > fieldset": {
                                borderColor: "#ed6c02"
                            }
                        }
                    }: undefined}/>
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

            <Error on={ slugEdited && showSlugError }>
                Slug cannot be empty
            </Error>

            <Error on={ slugEdited && showSlugValidError }>
                Slug is invalid or already taken.
            </Error>

            <StyledTextField
                required
                label="Project Name"
                value={ title }
                onChange={ onTitleChanged }
                onBlur={ onTitleBlurred  } />

            <Error on={ titleEdited && showTitleError }>
                Project name cannot be empty
            </Error>

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

            <Error on={ showUnknownError }>
                Some unknown error occured, please try again.
            </Error>
        </StyledGrid>

        <Button
            variant="contained"
            style={{ marginRight: '0.5rem' }}
            disabled={
                showTitleError || showSlugValidError || showSlugError
            }
            onClick={ onSave }>
            Save
        </Button>

        <Button variant="outlined" onClick={ onCancel }>Cancel</Button>
    </div>
}

export default NewProject;

