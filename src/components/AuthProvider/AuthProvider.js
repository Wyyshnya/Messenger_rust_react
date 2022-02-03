import React from 'react';
import {createContext, useState} from "react";

export const AuthContext = createContext(null);

export const AuthProvider = ({children}) => {
    const [user, setUser] = useState(sessionStorage.getItem("token"));
    if (user)
    {
        sessionStorage.removeItem("token");
    }
    const login = (newUser,  cb) => {
        setUser(newUser);
        cb();
    }
    const logout = (cb) => {
      setUser(null);
      cb();
    }

    const value = {user, login, logout};
    return <AuthContext.Provider value={value}>
        {children}
    </AuthContext.Provider>
}