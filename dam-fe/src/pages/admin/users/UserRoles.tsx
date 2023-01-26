import { useState, useEffect, useTransition } from 'react';
import { useNavigate } from 'react-router-dom';

import {
    IconButton, Table, TableHead, TableRow, TableCell, TableBody,
    TableContainer, TableSortLabel, Checkbox, TablePagination,
} from '@mui/material';

import { Edit as EditIcon, Delete, Add, LockReset } from '@mui/icons-material';

import { Breadcrumbs, Search, CustomFab, Loading, ConfirmDialog } from '../../../components';

import useAPI from '../../../hooks/useAPI';
import Role from '../../../models/Role';
import { useAdminStore } from '../../../store';
import
    UserPermissions, { UserPermissionsKeyToNameMapping }
from '../../../models/UserPermissions';

import { styled } from '@mui/material/styles';

const StyledSortLabel = styled(TableSortLabel)`
    width: max-content;
`;

const UserRoles = () => {
    const [ loading, setLoading ] = useState<boolean>(true);
    const [ showDeleteConfirmDialog, setShowDeleteConfirmDialog ]
            = useState<boolean>(true);
    const [ roles, setRoles ] = useState<Role[]>([]);
    const [ roleToDelete, setRoleToDelete ] = useState<Role>();

    const [ _, startTransition ] = useTransition();

    const { getRoles } = useAPI();
    const adminStore = useAdminStore();

    const navigate = useNavigate();

    const onEdit = (role: Role) => {
        adminStore.setRoleToEdit(role);
        navigate('/admin/roles/edit');
    };

    /**
     * When user clicks on the delete button on a role in the view roels page.
     */
    const onDelete = (role: Role) => startTransition(() => {
        setShowDeleteConfirmDialog(true);
        setRoleToDelete(role);
    });

    const onDeleteDialogClosed = () => {
    };

    useEffect(() => {
        const func = async () => {
            const rolesResp = await getRoles();

            if (
                rolesResp.success && Array.isArray(rolesResp.roles)
                && rolesResp.roles.length
            ) {
                startTransition(() => {
                    setRoles(rolesResp.roles);
                    setLoading(false);
                });
            }
        };
        func();
    }, []);

    return <div className="page page--user-roles">
        <Breadcrumbs links={ [
            { text: 'Admin', to: '/admin' },
            'User Roles',
        ] } />

        {
            loading ? <Loading />
                :
                
                <TableContainer>
                    <Table stickyHeader aria-label="sticky table">
                        <TableHead>
                            <TableRow>
                                <TableCell>
                                    <StyledSortLabel>Role Name</StyledSortLabel>
                                </TableCell>
                                {
                                    Object.entries(
                                        UserPermissionsKeyToNameMapping
                                    ).map(([key, value]) => <TableCell key={ key }>
                                        <StyledSortLabel
                                            active={ false }
                                            direction="asc"
                                            onClick={ () => {} }>
                                            { value }
                                        </StyledSortLabel>
                                    </TableCell>)
                                }
                                <TableCell>Actions</TableCell>
                            </TableRow>
                        </TableHead>

                        <TableBody>
                        {
                            roles.map((role, i) => <TableRow key={ i }>
                                <TableCell>{ role.roleName }</TableCell>

                                {
                                    Object.entries(role.permissions)
                                    .map(([key, value]) => <TableCell
                                            key={ key }>
                                        <Checkbox
                                            checked={ value }
                                            disabled/>
                                    </TableCell>)
                                }

                                <TableCell sx={{ display: 'flex' }}>
                                    <IconButton onClick={ () => onEdit(role) }>
                                        <EditIcon />
                                    </IconButton>
                                    <IconButton
                                        color="warning"
                                        onClick={ () => onDelete(role) }>
                                        <Delete />
                                    </IconButton>
                                </TableCell>
                            </TableRow>)
                        }
                        </TableBody>
                    </Table>
                </TableContainer>
        }

        <CustomFab fabs={[{
            text: "New Role",
            preIcon: <Add />,
            color: "secondary",
            show: true,
            onClick: () => {
                console.log('"New Role" button clicked!');
                navigate('/admin/roles/create');
            },
            }]}/>

        <ConfirmDialog
            title="Are you sure?"
            body={ <span>
                <p>
                Deleting this role will revoke all permissions from users that
                have this role.
                </p>
            </span>}
            open={ showDeleteConfirmDialog }
            onClose={ onDeleteDialogClosed }
            actions={[
                {
                    text: 'Cancel',
                    action: () => {},
                    buttonVariant: 'outlined',
                    buttonColor: 'error',
                },
                {
                    text: 'Delete',
                    action: () => {},
                    buttonVariant: 'contained',
                    buttonColor: 'error',
                }
            ]} />
    </div>;
}

export default UserRoles;

