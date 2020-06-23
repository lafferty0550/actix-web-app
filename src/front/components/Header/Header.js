import React, {useState} from 'react';
import {FontAwesomeIcon} from '@fortawesome/react-fontawesome';
import {faHome, faSignInAlt} from '@fortawesome/free-solid-svg-icons';
import {Link} from "react-router-dom";

import './Header.css';

export default () => {
    const [items] = useState([
        {
            title: 'Categories',
            uri: '/categories'
        },
        {
            title: 'Products',
            uri: '/products'
        },
        {
            title: 'Users',
            uri: '/users'
        }
    ]);
    return (
        <div className='header'>
            <Link to='/'><FontAwesomeIcon className='header__icon' icon={faHome} size='3x'/></Link>
            {items.map(({title, uri}, key) => (
                <Link to={uri} key={key}>
                    {title}
                </Link>
            ))}
            <Link to='/login'><FontAwesomeIcon className='header__icon' icon={faSignInAlt} size='3x'/></Link>
        </div>
    )
}