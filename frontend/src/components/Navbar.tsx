import React from 'react';
import { Link } from 'react-router-dom';

const Navbar: React.FC = () => {
    return (
        <nav className="bg-gray-800 p-4">
            <div className="container mx-auto flex justify-between">
                <div className="text-white">
                    <Link to="/" className="mr-4">Home</Link>
                    <Link to="/profile">Profile</Link>
                </div>
                <div>
                    <Link to="/login" className="text-white mr-4">Login</Link>
                    <Link to="/register" className="text-white">Register</Link>
                </div>
            </div>
        </nav>
    );
};

export default Navbar; 