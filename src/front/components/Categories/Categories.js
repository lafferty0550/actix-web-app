import React, {Component} from 'react';

export default class extends Component {
    constructor(props) {
        super(props);

        this.state = {
            isLoaded: false,
        }
    }

    componentDidMount() {
        console.log(213123)
        fetch("http://localhost:3000/categories")
            .then(res => res.json())
            .then(
                (result) => {
                    this.setState({
                        isLoaded: false
                    });
                },
                (error) => {
                    this.setState({
                        isLoaded: true
                    });
                }
            )
    }

    render() {
        return (
            <>
                {this.state.isLoaded ? 'Loading...' : 'Categories'}
            </>
        )
    }
}