import { useEffect, useState, useRef, useTransition, startTransition } from 'react';

import {
    Card, TextField, Box, List, ListItem, ListItemText, IconButton
} from '@mui/material';

import Remove from '@mui/icons-material/Remove';

import Typeahead from './Typeahead';

import SelectUserModel from '../models/SelectUserModel';

import { styled } from '@mui/material/styles';

const ContentBox = styled(Box)`
    min-height: 200px;
    max-height: 300px;
    overflow-y: auto;

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

const SelectUsers = () => {
    const [ userList, setUserList ] = useState<SelectUserModel[]>([
        {
            name: 'Abhinav Gunwant',
            id: 1,
        },
        {
            name: 'Someone Else',
            id: 2,
        },
        {
            name: 'Some one',
            id: 3,
        },
        {
            name: 'Person #2',
            id: 4,
        },
        {
            name: 'Person #3',
            id: 5,
        },
        {
            name: 'Person #4',
            id: 6,
        },
        {
            name: 'Person #5',
            id: 7,
        },
    ]);

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

    return <Card>
        <Typeahead placeholder="Type names to add users" />
        <ContentBox>
            <List>
                {
                    userList.map(user => <ListItem
                        secondaryAction={
                            <IconButton onClick={ removeUser(user) }>
                                <Remove color='action' />
                            </IconButton>
                        }
                        key={ user.id }>
                        <ListItemText>{ user.name } </ListItemText>
                    </ListItem>)
                }
            </List>
        </ContentBox>
    </Card>
}

export default SelectUsers;
