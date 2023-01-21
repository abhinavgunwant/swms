import { useState, useEffect, ReactNode } from 'react';
import { useNavigate } from 'react-router-dom';

import {
    TextField, Checkbox, FormGroup, FormControlLabel, Grid, Typography, Button,
} from '@mui/material';

import { Breadcrumbs, Search, CustomFab, Loading } from '../../../components';

import Role from '../../../models/Role';

import {
    UserPermissionsKeyToNameMapping
} from '../../../models/UserPermissions';

import { styled } from '@mui/material/styles';

const StyledTextField = styled(TextField)`
    width: 100%;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
`;

const StyledGridItem = (props: { children: ReactNode}) => <Grid
    item xs={ 6 } md={ 4 } lg={ 3 }>

    { props.children }
</Grid>;

interface NewEditRoleProps {
    role: 'new' | 'edit',
}

/**
 * Common component for New Role and Edit Role pages.
 */
const NewEditRole = (props: NewEditRoleProps) => {
    const [ role, setRole ] = useState<Role>();
    const [ nameError, setNameError ] = useState<boolean>(false);
    const [ roleName, setRoleName ] = useState<string>('');

    const navigate = useNavigate();

    const onNameChanged = () => {};
    const onNameFocus = () => {};
    const onNameBlur = () => {};

    const onSave = () => {
        navigate('/admin/roles');
    };

    const onCancel = () => {
        navigate('/admin/roles');
    };

    useEffect(() => {
        // When editing the role, the admin store must have the Role to be
        // edited, redirect back to "View Roles" page if this is not set.
        if (props.role === 'edit') {
            // TODO: Implement this!
        }
    }, []);

    return <div className="page page--new-edit-roles">
        <Breadcrumbs links={ [
            { text: 'Admin', to: '/admin' },
            { text: 'User Roles', to: '/admin/roles' },
            'New Role',
        ] } />

        <Typography variant="h5">New Role</Typography>

        <FormGroup>
            <Grid container>
                <Grid container>
                    <Grid item xs={ 12 } lg={ 8 }>
                        <StyledTextField
                            error={ nameError }
                            label="Role Name"
                            value={ roleName }
                            onChange={ onNameChanged }
                            onFocus={ onNameFocus }
                            onBlur={ onNameBlur }
                            helperText={
                                nameError ?
                                    'Full name; 4 - 64 characters' : ''
                            }
                            required />
                    </Grid>
                </Grid>

                <Grid container>
                    <Grid item xs={ 12 } sx={{ marginTop: '1rem' }}>
                        <Typography variant="h6">Permissions</Typography>
                    </Grid>

                    <Grid item xs={ 12 }>
                        <Typography>
                            Choose the permissions that should apply to users
                            with this role
                        </Typography>
                    </Grid>
                </Grid>

                {
                    Object.entries(UserPermissionsKeyToNameMapping)
                    .map(([key, value]) => {
                        if (typeof value === 'string') {
                            return <StyledGridItem key={ key }>
                                <FormControlLabel
                                    control={ <Checkbox /> }
                                    label={ value } />
                            </StyledGridItem>;
                        }

                        return '';
                    })
                }

                <Grid container>
                    <Grid item xs={ 12 } sx={{ marginTop: '1rem' }}>
                        <Button
                            variant="contained"
                            sx={{ marginRight: '0.5rem' }}
                            onClick={ onSave }>
                            Save
                        </Button>

                        <Button variant="outlined" onClick={ onCancel }>
                            Cancel
                        </Button>
                    </Grid>
                </Grid>
            </Grid>
        </FormGroup>
    </div>;
}

export default NewEditRole;

