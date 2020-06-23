import React, {useState} from "react";
import ReactDOM from "react-dom";
import {
    BrowserRouter as Router, Route
} from "react-router-dom";

import {Header, Categories} from '@components';

import './index.css';

const App = () => {
    const [routes] = useState([
        {
            path: '/categories',
            component: Categories,
        }
    ]);
    return (
        <Router>
            <Header/>
            {routes.map(({path, component}, key) => <Route path={path} component={component} key={key}/>)}
        </Router>
    );
};

ReactDOM.render(<App/>, document.querySelector("#root"));