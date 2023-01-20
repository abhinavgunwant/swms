import React, { useState, useEffect, useTransition } from 'react';

import {
    IconButton, Table, TableHead, TableRow, TableCell, TableBody,
    TableContainer, TableSortLabel, Checkbox, TablePagination,
} from '@mui/material';

import { Breadcrumbs, Search, CustomFab, Loading } from '../../../components';

import useAPI from '../../../hooks/useAPI';
import Role from '../../../models/Role';
import UserPermissions from '../../../models/UserPermissions';

import { styled } from '@mui/material/styles';

const COLUMNS: { name: string, key: keyof UserPermissions }[]  = [
    { name: 'Create Image', key: 'createImage' },
    { name: 'Read Image', key: 'readImage' },
    { name: 'Modify Image', key: 'modifyImage' },
    { name: 'Delete Image', key: 'deleteImage' },
    { name: 'Read Renditions', key: 'readRenditions' },
    { name: 'Modify Renditions', key: 'modifyRenditions' },
    { name: 'Delete Renditions', key: 'deleteRenditions' },
    { name: 'Read Project', key: 'readProject' },
    { name: 'Create Project', key: 'createProject' },
    { name: 'Modify Project', key: 'modifyProject' },
    { name: 'Delete Project', key: 'deleteProject' },
    { name: 'Read User', key: 'readUser' },
    { name: 'Create User', key: 'createUser' },
    { name: 'Modify User', key: 'modifyUser' },
    { name: 'Delete User', key: 'deleteUser' },
    { name: 'Publish', key:'publish' },
    { name: 'Publish All', key: 'publishAll' },
    { name: 'Access All Projects', key: 'accessAllProjects' },
];

const StyledSortLabel = styled(TableSortLabel)`
    width: max-content;
`;

const UserRoles = () => {
    const [ loading, setLoading ] = useState<boolean>(true);
    const [ roles, setRoles ] = useState<Role[]>([]);

    const [ _, startTransition ] = useTransition();

    const { getRoles } = useAPI();

    useEffect(() => {
        // setTimeout(() => setLoading(false), 1000);
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
                                COLUMNS.map((col, i) => <TableCell key={ i }>
                                    <StyledSortLabel
                                        active={ false }
                                        direction="asc"
                                        onClick={ () => {} }>
                                        { col.name }
                                    </StyledSortLabel>
                                </TableCell>)
                            }
                            </TableRow>
                        </TableHead>

                        <TableBody>
                        {
                            roles.map((role, i) => <TableRow key={ i }>
                                <TableCell>{ role.roleName }</TableCell>

                                {
                                    COLUMNS.map((col, j) => <TableCell key={ j }>
                                        <Checkbox
                                            checked={
                                                role.permissions[col.key]
                                            }
                                            disabled/>
                                    </TableCell>)
                                }
                            </TableRow>)
                        }
                        </TableBody>
                    </Table>
                </TableContainer>
        }
    </div>;
}

export default UserRoles;

