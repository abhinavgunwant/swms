import {
    useState, useEffect, ChangeEvent, Fragment, useTransition,
} from 'react';

import { useNavigate } from 'react-router-dom';

import {
    Typography, Grid, TextField, Button, CircularProgress, IconButton, Tooltip,
    Alert,
} from '@mui/material';

import { default as emoStyled } from '@emotion/styled';

import { Edit, Undo }from '@mui/icons-material/';

import Breadcrumbs from "../../../components/Breadcrumbs";

import useWorkspaceStore from '../../../store/workspace/WorkspaceStore';

import useAPI from '../../../hooks/useAPI';

import generateSlug, { validSlug } from '../../../utils/validations';

import { styled } from '@mui/material/styles';


const StyledTextField = styled(TextField)`
    width: 100%;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
`;

const StyledGrid = styled(Grid)`
    margin-top: 1rem;
`;

const StyledDiv = emoStyled.div`
    margin: 0.5rem;
    color: #666666;
`;

const CenterGrid = styled(Grid)`
    display: flex;
    justify-content: center;
    align-items: center;
`;

const NewFolder = () => {
    const [ saving, setSaving ] = useState<boolean>(false);
    const [ editSlug, setEditSlug ] = useState<boolean>(false);
    const [ error, setError ] = useState<boolean>(false);

    const [ slug, setSlug ] = useState<string>('');
    const [ folderTitle, setFolderTitle ] = useState<string>('');
    const [ folderDetails, setFolderDetails ] = useState<string>('');
    const [ editedSlug, setEditedSlug ] = useState<string>('');
    const [ errorMessage, setErrorMessage ] = useState<string>('');

    const [ _, startTransition ] = useTransition();

    const store = useWorkspaceStore();

    const navigate = useNavigate();

    const { addFolder } = useAPI(navigate);

    const onSlugChanged = (e: ChangeEvent<HTMLInputElement>) => {
        if (editSlug && validSlug(e.target.value)) {
            setEditedSlug(e.target.value);
        }
    };

    const onFolderTitleChanged = (e: ChangeEvent<HTMLInputElement>) => {
        setFolderTitle(e.target.value);
    };

    const onFolderDetailsChanged = (e: ChangeEvent<HTMLInputElement>) => {
        setFolderDetails(e.target.value);
    };

    const onEditSlug = () => startTransition(() => {
        if (editSlug) {
            setEditSlug(false);
        } else {
            setEditedSlug(slug);
            setEditSlug(true);
        }
    });

    const onSave = () => {
        if (saving) {
            return;
        }

        startTransition(() => {
            setSaving(true);
            setError(false);
        });

        const now = (new Date()).toISOString();

        addFolder({
            id: 0,
            slug: editSlug ? editedSlug : slug,
            title: folderTitle,
            projectId: store.currentProject.id,
            description: folderDetails,
            parentFolderId: store.currentFolder.id,
            createdBy: 0,
            modifiedBy: 0,
            createdOn: now,
            modifiedOn: now,
        }).then((response) => {
            if (response.success) {
                navigate('/workspace/tree' + store.currentPath);
            } else {
                startTransition(() => {
                    setSaving(false);
                    setError(true);
                    setErrorMessage(
                        'Some error occured, please try again later.'
                    );
                });
            }
        });
    };

    const onCancel = () => {
        navigate('/workspace/tree' + store.currentPath);
    };

    useEffect(() => {
        startTransition(() => setSlug(generateSlug(folderTitle)));
    }, [ folderTitle ]);

    return <div className="page page--new-folder">
        <Breadcrumbs links={[{ text: 'Workspace', to: '/workspace' }, 'New Folder']} />

        <Typography variant="h5">
            New Folder
        </Typography>

        <Typography variant="subtitle1">
            Enter below details to create new folder.
        </Typography>

        <StyledGrid container>
            <Grid item xs={12} lg={6}>
                <StyledTextField
                    label="Folder Title"
                    value={ folderTitle }
                    onChange={ onFolderTitleChanged }
                    required  />

                <Grid container>
                    <Grid item xs={11}>
                        <StyledTextField
                            label="Slug"
                            value={ editSlug ? editedSlug : slug }
                            disabled={ !editSlug }
                            onChange={ onSlugChanged }
                            required/>
                    </Grid>

                    <CenterGrid item xs={1}>
                        <Tooltip title={
                            (editSlug ?
                                'Undo edit folder slug'
                                :
                                'Edit folder slug')
                            }>
                            <IconButton
                                color="secondary"
                                onClick={ onEditSlug }>
                                { editSlug ? <Undo /> : <Edit /> }
                            </IconButton>
                        </Tooltip>
                    </CenterGrid>
                </Grid>

                <StyledDiv>
                    <span><b>Path:</b></span>
                    <span>
                        {
                            store.currentPath
                            + (slug ? '/' + slug : '')
                        }
                    </span>
                </StyledDiv>

                <StyledTextField
                    label="Folder Details"
                    value={ folderDetails }
                    onChange={ onFolderDetailsChanged }
                    rows={3}
                    multiline />
            </Grid>
        </StyledGrid>

        {
            error && <Alert severity="error" sx={{ marginBottom: '0.5rem' }}>
                { errorMessage }
            </Alert>
        }

        <Button
            variant="contained"
            style={{ marginRight: '0.5rem' }}
            onClick={ onSave }
            disabled={ folderTitle == '' || slug == '' }>
            {
                saving ?
                    <Fragment>
                        <CircularProgress
                            size={ 16 }
                            color="secondary"
                            sx={{
                                color: '#ffffff',
                                marginRight: '1rem',
                            }} />
                            Saving
                    </Fragment>
                    :
                    'Save'
            }
        </Button>
        <Button variant="outlined" onClick={ onCancel }>Cancel</Button>
    </div>
}

export default NewFolder;

