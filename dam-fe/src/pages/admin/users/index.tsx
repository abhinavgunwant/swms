import { useState, useEffect, ChangeEvent, useTransition } from 'react';
import { useNavigate } from 'react-router-dom';

import {
    Box, Typography, FormControl, InputLabel, Select, MenuItem, List,
    ListItem, ListItemText, ListItemButton, IconButton, Table, TableHead,
    TableRow, TableCell, TableBody, TableContainer, TableSortLabel, Checkbox,
    TablePagination,
} from '@mui/material';

import { Edit, Delete, Add, LockReset } from '@mui/icons-material';

import Breadcrumbs from '../../../components/Breadcrumbs';
import Search from '../../../components/Search';
import CustomFab from '../../../components/CustomFab';
import CreateUserPage from './Create';

import UserListing from '../../../models/UserListing';

import styled from '@emotion/styled';
import { styled as matStyled } from '@mui/material/styles';

const TopRow = styled.div`
    display: flex;
    justify-content: space-between;
    align-items: center;
`;

const ContentBox = matStyled(Box)`
    margin: 1rem 0 4rem 0;
`;

const columns: string[] = [ 'Login ID', 'Name', 'Email' ];

const Users = () => {
    const [ searchTerm, setSearchTerm ] = useState<string>('');
    const [ pageSize, setPageSize ] = useState<number>(10);
    const [ page, setPage ] = useState<number>(0);
    const [ selectAll, setSelectAll ] = useState<boolean>(false);
    const [ selectionArr, setSelectionArr ] = useState<boolean[]>([]);
    const [ data, setData ] = useState<UserListing[]>([]);

    // eslint-disable-next-line @typescript-eslint/no-unused-vars 
    const [ _, startTransition ] = useTransition();

    const navigate = useNavigate();

    const onPageSizeChanged = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
        try {
            const num = parseInt(e.target.value);
            if (!isNaN(num)) {
                startTransition(() => setPageSize(num));
            }
        } catch (e) {
            console.log(e);
        }
    };

    const onPageChanged = (
        e: React.MouseEvent<HTMLButtonElement> | null,
        _page: number
    ) => startTransition(() => setPage(_page));

    const onSelectAll = (e: ChangeEvent<HTMLInputElement>) => startTransition(
        () => setSelectAll(e.target.checked)
    );

    const onRowSelected = (n: number) => {
        const arr = [ ...selectionArr ];

        if (typeof arr[n] !== 'undefined') {
            arr[n] = !arr[n];
        } else {
            arr[n] = true;

            while (typeof arr[--n] === 'undefined' && n > -1) {
                arr[n] = false;
                console.log('!!');
            }
        }

        startTransition(() => setSelectionArr(arr))
    };

    const allSelected = () => {
        if (selectionArr.length === 0) {
            return false;
        }

        if (data.length < pageSize && selectionArr.length < data.length) {
            return false;
        }

        if (data.length >= pageSize && selectionArr.length < pageSize) {
            return false;
        }

        return selectionArr.reduce((acc, curr) => acc && curr, true);
    }

    const someSelected = () =>
        selectionArr.reduce((acc, curr) => acc || curr, false);

    useEffect(() => {
        startTransition(() => setData([
            { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 1, loginId: 'abhii1', name: 'Abhinav Gunwant1', email: 'abhi@example.com' }, { id: 2, loginId: 'abhii2', name: 'Abhinav Gunwant2', email: 'abhi@example.com' }, { id: 3, loginId: 'abhii3', name: 'Abhinav Gunwant3', email: 'abhi@example.com' }, { id: 4, loginId: 'abhii4', name: 'Abhinav Gunwant4', email: 'abhi@example.com' },{ id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, { id: 0, loginId: 'abhii', name: 'Abhinav Gunwant', email: 'abhi@example.com' }, 
        ]));
    });

    useEffect(() => {
        console.log(selectAll, allSelected());
        if (selectAll) {
            if (data.length >= pageSize) {
                startTransition(() => setSelectionArr(
                    Array(pageSize).fill(true))
                );

                return;
            }

            startTransition(() => setSelectionArr(
                Array(data.length).fill(true))
            );

            return;
        }

        if (allSelected()) {
            startTransition(() => setSelectionArr(
                Array(selectionArr.length).fill(false))
            );
        }
    }, [ selectAll ]);

    useEffect(() => {
        if (selectAll && !allSelected()) {
            startTransition(() => setSelectAll(false));

            return;
        }

        if (!selectAll && allSelected()) {
            startTransition(() => setSelectAll(true));
        }
    }, [ selectionArr ]);

    return <div className="page page--users">
        <TopRow>
            <Breadcrumbs links={ [
                { text: 'Admin', to: '/admin' },
                'View Users',
            ] } />

            <Search />
        </TopRow>

        <ContentBox>
            <TableContainer>
                <Table stickyHeader aria-label="sticky table">
                    <TableHead>
                        <TableRow>
                            <TableCell padding="checkbox">
                                <Checkbox
                                    color="primary"
                                    checked={ selectAll }
                                    onChange={ onSelectAll } />
                            </TableCell>

                            {
                                columns.map((col, i) => <TableCell key={ i }>
                                    <TableSortLabel
                                        active={ false }
                                        direction="asc"
                                        onClick={ () => {} }>
                                        { col }
                                    </TableSortLabel>
                                </TableCell>)
                            }
                        </TableRow>
                    </TableHead>

                    <TableBody>
                        { data.slice(page * pageSize, page * pageSize + pageSize)
                            .map((row, i) => <TableRow key={ i }>
                            <TableCell padding="checkbox">
                                <Checkbox
                                    color="primary"
                                    checked={ selectionArr.length > i && selectionArr[i] }
                                    onChange={ () => onRowSelected(i) } />
                            </TableCell>

                            <TableCell>{ row.loginId }</TableCell>
                            <TableCell>{ row.name }</TableCell>
                            <TableCell>{ row.email }</TableCell>
                        </TableRow>) }
                    </TableBody>
                </Table>
            </TableContainer>

            <TablePagination
                rowsPerPageOptions={[ 10, 25, 50, 100 ]}
                component="div"
                count={ data.length }
                rowsPerPage={ pageSize }
                onRowsPerPageChange={ onPageSizeChanged }
                page={ page }
                onPageChange={ onPageChanged } />
        </ContentBox>

        <CustomFab fabs={[
            {
                text: "New User",
                preIcon: <Add />,
                color: "secondary",
                show: true,
                onClick: () => {
                    console.log('"New User" button clicked!');
                    navigate('/admin/users/create');
                },
            },
            {
                text: "Reset Password",
                preIcon: <LockReset />,
                show: someSelected(),
                onClick: () => {
                    console.log('"Reset Password" button clicked!');
                },
            },
            {
                text: "Delete User(s)",
                preIcon: <Delete />,
                color: "error",
                show: someSelected(),
                onClick: () => {
                    console.log('"Delete User(s)" button clicked!');
                },
            },
            ]}/>
    </div>;
}

export { CreateUserPage as Create };

export default Users;

