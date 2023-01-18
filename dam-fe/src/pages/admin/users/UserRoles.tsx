import React, { useState, useEffect } from 'react';

import { Breadcrumbs, Search, CustomFab, Loading } from '../../../components';

import {
    IconButton, Table, TableHead, TableRow, TableCell, TableBody,
    TableContainer, TableSortLabel, Checkbox, TablePagination,
} from '@mui/material';

import { styled } from '@mui/material/styles';

const COLUMNS = [
    'Create Image', 'Read Image', 'Modify Image', 'Delete Image',
    'Read Renditions', 'Modify Renditions', 'Delete Renditions',
    'Read Project', 'Create Project', 'Modify Project', 'Delete Project',
    'Read User', 'Create User', 'Modify User', 'Delete User', 'Publish',
    'Publish All', 'Access All Projects',
];

const StyledSortLabel = styled(TableSortLabel)`
    width: max-content;
`;

const UserRoles = () => {
    const [ loading, setLoading ] = useState<boolean>(true);

    useEffect(() => {
        setTimeout(() => setLoading(false), 1000);
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
                                        { col }
                                    </StyledSortLabel>
                                </TableCell>)
                            }
                            </TableRow>
                        </TableHead>

                        <TableBody>
                        </TableBody>
                    </Table>
                </TableContainer>
        }
    </div>;
}

export default UserRoles;

