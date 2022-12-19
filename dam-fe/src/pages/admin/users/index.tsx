import Breadcrumbs from '../../../components/Breadcrumbs';
import Search from '../../../components/Search';

import styled from '@emotion/styled';

const TopRow = styled.div`
    display: flex;
    justify-content: space-between;
    align-items: center;
`;

const Users = () => {
    return <div className="page page--users">
        <TopRow>
            <Breadcrumbs links={ [
                { text: 'Admin', to: '/admin' },
                'View Users',
            ] } />

            <Search />
        </TopRow>
    </div>
}

export default Users;

