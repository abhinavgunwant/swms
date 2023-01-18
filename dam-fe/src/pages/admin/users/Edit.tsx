import {
    useState, useEffect, useTransition, ChangeEvent, ChangeEventHandler,
} from 'react';
import { useNavigate } from 'react-router-dom';

import {
    TextField, Button, Typography, Grid,
} from '@mui/material';

import { Breadcrumbs, Search, CustomFab, Loading } from '../../../components';

import useAdminStore from '../../../store/admin/AdminStore';
import UserListing from '../../../models/UserListing';
import useAPI from '../../../hooks/useAPI';

import { styled } from '@mui/material/styles';

const StyledTF = styled(TextField)`
    margin: 0.5rem 0;
    width: 100%;
`;

export const Edit = () => {
    const [ name, setName ] = useState<string>('');
    const [ email, setEmail ] = useState<string>('');
    const [ enableSave, setEnableSave ] = useState<boolean>(false);

    const [ _, startTransition ] = useTransition();

    const { editUser } = useAPI();

    const adminStore = useAdminStore();
    const navigate = useNavigate();

    const onNameChanged: ChangeEventHandler = (
        e: ChangeEvent<HTMLInputElement>
    ) => setName(e.target.value);

    const onEmailChanged: ChangeEventHandler = (
        e: ChangeEvent<HTMLInputElement>
    ) => setEmail(e.target.value);

    const onResetPassword = () => {
        // TODO: Open reset password dialog.
    };

    const onSave = () => {

        if (adminStore && adminStore.userToEdit) {
            const user: UserListing = {
                id: adminStore.userToEdit.id,
                loginId: adminStore.userToEdit.loginId,
                name,
                email,
            }

            editUser(user);

            console.log('User edited!');

            navigate('/admin/users');
        }
    }

    useEffect(() => {
        if (
            adminStore?.userToEdit?.name === name
            && adminStore?.userToEdit?.email === email
        ) {
            if (enableSave) {
                startTransition(() => setEnableSave(false));
            }
        } else {
            if (!enableSave) {
                startTransition(() => setEnableSave(true));
            }
        }
    }, [ name, email ]);

    useEffect(() => {
        if (!adminStore.userToEdit) {
            setTimeout(() => navigate('/admin/users'), 2400);

            return;
        }

        setName(adminStore.userToEdit.name);
        setEmail(adminStore.userToEdit.email);
    }, []);

    if (!adminStore.userToEdit) {
        return <div>Redirecting back to user listing page...</div>;
    }

    return <div className="page page--edit-user">
        <Breadcrumbs links={ [
            { text: 'Admin', to: '/admin' },
            { text: 'View Users', to: '/admin/users' },
            'Edit User: ' + adminStore.userToEdit.loginId,
        ] } />

        <Typography variant="h4">Edit User</Typography>

        <Grid container>
            <Grid item sm={ 12 } md={ 6 }>
                <StyledTF
                    label="Login ID"
                    value={ adminStore.userToEdit.loginId }
                    disabled={ true } />

                <StyledTF
                    label="Name"
                    value={ name }
                    onChange={ onNameChanged } />

                <StyledTF
                    label="Email"
                    value={ email }
                    onChange={ onEmailChanged } />

                <Button
                    onClick={ onResetPassword }
                    sx={{
                        margin: '0.5rem 0 1rem 0'
                    }}>
                    Reset Password
                </Button>
            </Grid>
        </Grid>

        <Button
            variant="contained"
            disabled={ !enableSave }
            onClick={ onSave }>
            Save
        </Button>

        <Button
            variant="outlined"
            sx={{ marginLeft: '0.5rem' }}>
            Cancel
        </Button>
    </div>;
}

export default Edit;

