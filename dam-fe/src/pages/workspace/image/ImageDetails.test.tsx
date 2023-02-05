import { act } from 'react-dom/test-utils';
import { render, screen } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';
import ImageDetails from './ImageDetails';
import create, { storeResetFns } from '../../../store/workspace/WorkspaceStore.test';

describe('Tests the "Image Details" page.', () => {
    beforeEach(() => {
        act(() => storeResetFns.forEach((resetFn) => resetFn()));
    });

    it('Renders successfully', () => {
        // TODO: mock useAPI and the api response for an image
        // TODO: mock the useParams hook of react-router-dom
        render(<BrowserRouter><ImageDetails /></BrowserRouter>);

        expect(screen.getByText('Image Title')).toBeInTheDocument();
        expect(screen.getByText('Image File Name')).toBeInTheDocument();
        expect(screen.getByText('Image Encoding')).toBeInTheDocument();
        expect(screen.getByText('Height')).toBeInTheDocument();
        expect(screen.getByText('Width')).toBeInTheDocument();
        expect(screen.getByText('Created On')).toBeInTheDocument();
        expect(screen.getByText('Modified On')).toBeInTheDocument();
        expect(screen.getByText('Renditions')).toBeInTheDocument();
    });
});

