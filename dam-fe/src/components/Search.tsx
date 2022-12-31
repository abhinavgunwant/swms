import { useState, useTransition } from 'react';

import {
    InputBase, IconButton
} from '@mui/material';

import { Search as SearchIcon } from '@mui/icons-material';

import styled from '@emotion/styled';

const SearchWrapper = styled.div`
    background: #eeeeee;
    padding: 0 0.25rem 0 1rem;
    border-radius: 4px;
`;

export const Search = () => {
    const [ expanded, setExpanded ] = useState<boolean>(false);

    // eslint-disable-next-line @typescript-eslint/no-unused-vars 
    const [ _, startTransition ] = useTransition();

    const expand = () => {
        startTransition(() => setExpanded(true));
    }

    const contract = () => {
        startTransition(() => setExpanded(false));
    }

    return <SearchWrapper onClick={ expand } onFocus={ expand } onBlur={ contract }>
        <InputBase
            placeholder="Search"
            sx={{
                width: expanded ? 240 : 100,
                transition: 'width 0.2s ease-in'
            }} />
        <IconButton><SearchIcon /></IconButton>
    </SearchWrapper>;
}

export default Search;

