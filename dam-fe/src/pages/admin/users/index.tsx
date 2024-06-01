import {
    useState, useEffect, useTransition, ChangeEvent, Fragment, useMemo,
} from 'react';
import { useNavigate } from 'react-router-dom';

import {
    Box, IconButton, Table, TableHead,
    TableRow, TableCell, TableBody, TableContainer, TableSortLabel, Checkbox,
    TablePagination,
} from '@mui/material';

import { Edit as EditIcon, Delete, Add, LockReset } from '@mui/icons-material';

import { Breadcrumbs, Search, CustomFab, Loading } from '../../../components';
import CreateUserPage from './Create';
import UserRoles from './UserRoles';
import NewEditRole from './NewEditRole';
import Edit from './Edit';

import UserListing from '../../../models/UserListing';
import useAPI from '../../../hooks/useAPI';
import useAdminStore from '../../../store/admin/AdminStore';

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

const columns: string[] = [ 'Login ID', 'Name', 'Email', 'Actions' ];

const Users = () => {
    const [ pageSize, setPageSize ] = useState<number>(10);
    const [ page, setPage ] = useState<number>(0);
    const [ selectAll, setSelectAll ] = useState<boolean>(false);
    const [ selectionArr, setSelectionArr ] = useState<boolean[]>([]);
    const [ data, setData ] = useState<UserListing[]>([]);
    const [ loading, setLoading ] = useState<boolean>(true);

    // eslint-disable-next-line @typescript-eslint/no-unused-vars 
    const [ _, startTransition ] = useTransition();

    const { getUsers } = useAPI();

    const adminStore = useAdminStore();

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
        _e: React.MouseEvent<HTMLButtonElement> | null,
        _page: number
    ) => startTransition(() => setPage(_page));

    const onSelectAll = (e: ChangeEvent<HTMLInputElement>) =>
        setSelectAll(e.target.checked);

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

    const onEdit = (userListingObj: UserListing) => {
        adminStore.setUserToEdit(userListingObj);

        navigate('/admin/users/edit');
    }

    const allSelected = useMemo(() => {
        const n = selectionArr.length;

        if (n === 0) {
            return false;
        }

        if (data.length < pageSize && selectionArr.length < data.length) {
            return false;
        }

        if (data.length >= pageSize && selectionArr.length < pageSize) {
            return false;
        }

        let count = 0;

        for (let i=0; i<n && selectionArr[i]; ++i, ++count) {
            // no code required here...
        }

        return count === n;
    }, [ selectionArr, pageSize, data ]);

    const someSelected = useMemo(() => {
        for (let i=0; i<selectionArr.length; ++i) {
            if (selectionArr[i]) {
                return true;
            }
        }
    }, [ selectionArr ]);

    const multipleSelected = () => {
        if (selectionArr.length) {
            let count = 0;

            for (let i=0; i<selectionArr.length && count < 2; ++i) {
                if (selectionArr[i]) {
                    ++count;
                }
            }

            return count > 1;
        }

        return false;
    }

    useEffect(() => {
        const makeReq = async () => {
            const resp = await getUsers();

            if (resp.success) {
                startTransition(() => {
                    setData(resp.users);
                    setLoading(false);
                });
            }

            startTransition(() => setLoading(false));
        };

        makeReq();
    }, []);

    useEffect(() => {
        if (selectAll) {
            if (data.length >= pageSize) {
                setSelectionArr(Array(pageSize).fill(true));

                return;
            }

            setSelectionArr(Array(data.length).fill(true));

            return;
        }

        if (allSelected) {
            setSelectionArr(Array(selectionArr.length).fill(false));
        }
    }, [ selectAll ]);

    useEffect(() => {
        if (selectAll && !allSelected) {
            setSelectAll(false)

            return;
        }

        if (!selectAll && allSelected) {
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

        {
            loading ? <Loading />
            :
            <Fragment>
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
                                    columns.map((col, i) => <TableCell
                                            key={ i }>
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
                            { data.slice(
                                page * pageSize, page * pageSize + pageSize
                                ).map((row, i) => <TableRow key={ i }>
                                <TableCell padding="checkbox">
                                    <Checkbox
                                        color="primary"
                                        checked={
                                            selectionArr.length > i
                                            && selectionArr[i]
                                        }
                                        onChange={ () => onRowSelected(i) } />
                                </TableCell>

                                <TableCell>{ row.loginId }</TableCell>
                                <TableCell>{ row.name }</TableCell>
                                <TableCell>{ row.email }</TableCell>
                                <TableCell sx={{ display: 'flex' }}>
                                    <IconButton onClick={ () => onEdit(row) }>
                                        <EditIcon />
                                    </IconButton>
                                    <IconButton><LockReset /></IconButton>
                                    <IconButton
                                        color="warning">
                                        <Delete />
                                    </IconButton>
                                </TableCell>
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
                    show: someSelected && !multipleSelected(),
                    onClick: () => {
                        console.log('"Reset Password" button clicked!');
                    },
                },
                {
                    text: "Edit User",
                    preIcon: <EditIcon />,
                    show: someSelected && !multipleSelected(),
                    onClick: () => {
                        console.log('"Edit User" button clicked!');
                    },
                },
                {
                    text: "Delete User(s)",
                    preIcon: <Delete />,
                    color: "error",
                    show: someSelected,
                    onClick: () => {
                        console.log('"Delete User(s)" button clicked!');
                    },
                },
                ]}/>
            </Fragment>
        }
    </div>;
}

export { CreateUserPage as Create, Edit, UserRoles, NewEditRole };

export default Users;

