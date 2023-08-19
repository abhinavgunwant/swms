import { expect, test } from 'vitest';
import { render, screen } from '../utils/test-utils';

import App from './App';

it('Renders app', () => {
    render(<App />);

    expect(
        screen.getByText("In order to use DAM, you must login.")
    ).toBeInTheDocument();
});

