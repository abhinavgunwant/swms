import { Link } from 'react-router-dom';

//import MuiLink from '@mui/material/Link';
import MuiBreadcrumbs from '@mui/material/Breadcrumbs';
import BreadcrumbProps from '../models/props/BreadcrumbProps';

import { styled } from '@mui/material/styles';

// To make the workspace and new-image/new-folder breadcrumb height same...
// i.e. to prevent any significant layout shift...
const StyledBreadcrumbs = styled(MuiBreadcrumbs)`height: 2.25rem;`;

export const Breadcrumbs = ({ links }: BreadcrumbProps) => {

    return <StyledBreadcrumbs>
    {
        links.map((link, i) => {
            if (typeof link === 'string') {
                return <span key={i}>{ link }</span>;
            }

            if (link.to === undefined) {
                return <span key={i}>{ link.text }</span>;
            }

            return <Link
                to={ link.to }
                key={ i }
                style={{ color: '#1976d2' }}
                onClick={ () => {
                    if (typeof link.onClick === 'function') {
                        link.onClick();
                    }
                }}>
                { link.text }
            </Link>
        })
    }
    </StyledBreadcrumbs>;
}

export default Breadcrumbs;

