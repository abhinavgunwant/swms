// import { act } from 'react-dom/test-utils';
import { render, screen, waitFor } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';
import ImageDetails from './ImageDetails';

describe('Tests the "Image Details" page.', () => {
    it('Renders successfully', async () => {
        // TODO: mock useAPI and the api response for an image
        // TODO: mock the useParams hook of react-router-dom
        render(<BrowserRouter><ImageDetails /></BrowserRouter>);

        await waitFor(() => {
            expect(screen.getByText('Image Details')).toBeInTheDocument();
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
});

