import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
// I am assuming the api client will be generated here
// import { DefaultApi, Configuration } from '../api';
// import { User } from '../api';

const Profile: React.FC = () => {
    // const [user, setUser] = useState<User | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        const fetchProfile = async () => {
            try {
                const token = localStorage.getItem('token');
                if (!token) {
                    navigate('/login');
                    return;
                }
                // const config = new Configuration({ accessToken: token });
                // const api = new DefaultApi(config);
                // const response = await api.getProfile();
                // setUser(response.data);
            } catch (error) {
                console.error('Failed to fetch profile', error);
                navigate('/login');
            }
        };

        fetchProfile();
    }, [navigate]);

    const handleLogout = () => {
        localStorage.removeItem('token');
        navigate('/login');
    };

    // if (!user) {
    //     return <div>Loading...</div>;
    // }

    return (
        <div className="flex items-center justify-center h-screen">
            <div className="w-full max-w-md p-8 space-y-6 bg-white rounded-lg shadow-md">
                <h1 className="text-2xl font-bold text-center">Profile</h1>
                {/* <p><strong>First Name:</strong> {user.firstname}</p>
                <p><strong>Last Name:</strong> {user.lastname}</p>
                <p><strong>Email:</strong> {user.email}</p>
                <p><strong>Role:</strong> {user.role}</p> */}
                <button
                    onClick={handleLogout}
                    className="w-full px-4 py-2 font-bold text-white bg-red-600 rounded-md hover:bg-red-700"
                >
                    Logout
                </button>
            </div>
        </div>
    );
};

export default Profile; 