import {
    useState, useEffect, ChangeEventHandler, ChangeEvent, memo,
    Fragment, useTransition,
} from 'react';
import { useNavigate } from 'react-router-dom';

import {
    TextField, Checkbox, FormGroup, FormControlLabel, Grid, Typography, Button,
} from '@mui/material';

import { Breadcrumbs } from '../../../components';

import Role, { RoleImpl } from '../../../models/Role';
import { useAdminStore } from '../../../store';
import useAPI from '../../../hooks/useAPI';

import UserPermissions, { UserPermissionsImpl, UserPermissionsKeyToNameMapping }
from '../../../models/UserPermissions';

import { styled } from '@mui/material/styles';

const StyledTextField = styled(TextField)`
    width: 100%;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
`;

const PermissionsView = memo((
    props: {
        permissions?: UserPermissions,
        onPermissionsChanged: (newPerms: UserPermissions) => void
    }
) => {
    const onPermissionsChanged = (key: string, value: boolean) => {
        let newPerms: UserPermissions;

        if (props.permissions === undefined) {
            newPerms = new UserPermissionsImpl();
            (newPerms as any)[key] = value;
        } else {
            newPerms = { ...props.permissions, [key]: value };
        }

        props.onPermissionsChanged(newPerms);
    };

    if (props.permissions === undefined) {
        return null; 
    }

    return <Fragment>
    {
        Object.entries(UserPermissionsKeyToNameMapping)
        .map(([key, value]) => {
            if (typeof value === 'string') {
                return <Grid xs={6} md={4} lg={3} key={key} item>
                    <FormControlLabel
                        control={
                            <Checkbox checked={
                                (props.permissions as any)[key]
                            } />
                        }
                        onChange={
                            (_, checked: boolean) => {
                                onPermissionsChanged(key, checked)
                            }
                        }
                        label={ value } />
                </Grid>;
            }

            return '';
        })
    }
    </Fragment>;
});

interface NewEditRoleProps {
    mode: 'new' | 'edit',
}

/**
 * Common component for New Role and Edit Role pages.
 */
const NewEditRole = (props: NewEditRoleProps) => {
    const [ role, setRole ] = useState<Role>();
    const [ nameError, setNameError ] = useState<boolean>(false);

    const [ _, startTransition ] = useTransition();

    const navigate = useNavigate();
    const adminStore = useAdminStore();

    const { createEditRoles } = useAPI();

    const onNameChanged: ChangeEventHandler = (
        e: ChangeEvent<HTMLInputElement>
    ) => {
        if (role === undefined) {
            setRole(new RoleImpl(-1, e.target.value));
        } else {
            setRole(new RoleImpl(
                role.id,
                e.target.value,
                { ...role?.permissions }
            ));
        }

        setNameError(false);
        //setRoleName(e.target.value);
    };

    const onPermissionsChanged = (newPerms: UserPermissions) => {
        startTransition(() => setRole(
            new RoleImpl(role?.id, role?.roleName || '', newPerms
        )));
    };

    const onSave = async () => {
        console.log(role);

        if (role === undefined) {
            return;
        }

        if (!role?.roleName) {
            startTransition(() => setNameError(true));
            return;
        }

        console.log('mode=', props.mode);

        await createEditRoles(role, props.mode);

        navigate('/admin/roles');
    };

    const onCancel = () => {
        navigate('/admin/roles');
    };

    useEffect(() => {
        // When editing the role, the admin store must have the Role to be
        // edited, redirect back to "View Roles" page if this is not set.
        if (props.mode === 'edit') {
            if (adminStore.roleToEdit) {
                setRole(adminStore.roleToEdit);
               // setRoleName(adminStore.roleToEdit.roleName);
            } else {
                navigate('/admin/roles');
            }
        } else {
            setRole(new RoleImpl());
        }
    }, []);

    return <div className="page page--new-edit-roles">
        <Breadcrumbs links={ [
            { text: 'Admin', to: '/admin' },
            { text: 'User Roles', to: '/admin/roles' },
            props.mode === 'edit' ? 'Edit Role' : 'New Role',
        ] } />

        <Typography variant="h5">
            { props.mode === 'edit' ? 'Edit Role' : 'New Role' }
        </Typography>

        <FormGroup>
            <Grid container>
                <Grid container>
                    <Grid item xs={ 12 } lg={ 8 }>
                        <StyledTextField
                            error={ nameError }
                            label="Role Name"
                            value={ role?.roleName }
                            onChange={ onNameChanged }
                            helperText={
                                nameError ?
                                    '4 - 64 alpha-numeric characters' : ''
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

                <PermissionsView
                    permissions={ role?.permissions }
                    onPermissionsChanged={ onPermissionsChanged } />

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

