import { useEffect, useState, useRef, useTransition, startTransition } from 'react';

import {
    Card, TextField, Box, List, ListItem, ListItemText, IconButton, Typography
} from '@mui/material';

import Remove from '@mui/icons-material/Remove';

import Typeahead from './Typeahead';

import SelectUserModel from '../models/SelectUserModel';
import useAPI from '../hooks/useAPI';

import { styled } from '@mui/material/styles';

const ContentBox = styled(Box)`
    min-height: 200px;
    max-height: 300px;
    overflow-y: auto;
    background: #eeeeee;

    &::-webkit-scrollbar {
        width: 10px;

        &-track {
            background: #efefef;
        }

        &-thumb {
            background: #cccccc;
            border-radius: 5px
        }
    }
`;


const NothingBox = styled(Box)`
    display: flex;
    justify-content: center;
    align-items: center;
    width: calc(100% - 4rem);
    height: 200px;
    text-align: center;
    padding: 2rem;
`;

const NameList = styled(List)`
    padding: 0;
`;

const NameItem = styled(ListItem)`
    background: #ffffff;
    border-bottom: 1px solid #dddddd;
`;

interface SelectedUserProps {
    placeholder?: string,
    title?: string,
}

const SelectUsers = (props: SelectedUserProps) => {
    const [ userList, setUserList ] = useState<SelectUserModel[]>([]);

    const { userTypeahead } = useAPI();

    const removeUser = (user: SelectUserModel) => () => {
        let arr = [...userList];
        let itemRemoved: boolean = false;

        for (let i=0; i<arr.length; ++i) {
            if (arr[i].id === user.id) {
                arr.splice(i, 1);
                itemRemoved = true;

                break;
            }
        }

        if (itemRemoved) {
            startTransition(() => setUserList(arr));
        }
    };

    const addUser = (user: SelectUserModel) => {
        let add = true;

        for (let i=0; i<userList.length; ++i) {
            if (userList[i].id === user.id) {
                add = false;
                break;
            }
        }

        if (add) {
            startTransition(() => setUserList([...userList, user]));
        }
    }

    return <Card>
        {
            props?.title &&
            <Typography
                variant="subtitle2"
                sx={{
                    marginLeft: '1rem',
                    marginTop: '0.25rem',
                }}>
                { props.title }
            </Typography>
        }
        <Typeahead
            placeholder={ props?.placeholder || "Type names to add users"}
            fetcherFunction={ userTypeahead }
            onItemSelected={ addUser } />

        <ContentBox>
            {
                userList.length === 0 &&
                <NothingBox>
                    Nothing selected! Start typing names
                    in the search bar above to add users.
                </NothingBox>
            }
            <NameList>
                {
                    userList.map(user => <NameItem
                        secondaryAction={
                            <IconButton onClick={ removeUser(user) }>
                                <Remove color='action' />
                            </IconButton>
                        }
                        key={ user.id }>
                        <ListItemText>{ user.name }</ListItemText>
                    </NameItem>)
                }
            </NameList>
        </ContentBox>
    </Card>
}

export default SelectUsers;

