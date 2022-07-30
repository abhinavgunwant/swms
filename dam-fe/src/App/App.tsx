import React, { Fragment } from 'react';

import Header from '../components/Header';
import Router from '../components/Router';
import styled from '@emotion/styled';

const Page = styled.div`
    padding: 1rem;
    margin-top:4rem;
`;

const App = (): React.ReactElement => <Fragment>
    <Header />
    
    <Page>
        <Router />
    </Page>
</Fragment>;

export default App;
