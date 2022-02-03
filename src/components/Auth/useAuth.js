import {useContext} from 'react';
import {AuthContext} from "../AuthProvider/AuthProvider";

export function useAuth() {
    return useContext(AuthContext);
}