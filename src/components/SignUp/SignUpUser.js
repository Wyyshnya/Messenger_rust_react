import React from 'react';
import './SignUpUser.css';
import {useLocation, useNavigate, Navigate} from "react-router-dom";
import axios from "axios";
import {useAuth} from "../Auth/useAuth";

export default function SignUpUser() {
    const navigate = useNavigate();
    const location = useLocation();
    const {login} = useAuth();


    function handleSubmit(event) {
        event.preventDefault();
        const form = event.target;
        const user = form.username.value;

        login(user, () => navigate("/login", {replace: true}));
    }
    return (
        <div className="center">
            <h1>Sign Up</h1>
            <form onSubmit={handleSubmit}>
                <div className="txt_field">
                    <input type="text" name="username" required />
                        <span></span>
                        <label>Username</label>
                </div>
                {/*<div class="txt_field">*/}
                {/*<input type="text" name="email" required>*/}
                {/*<span></span>*/}
                {/*<label>Email</label>*/}
                {/*</div>*/}
                <div className="txt_field">
                    <input type="password" name="password" required />
                        <span></span>
                        <label>Password</label>
                </div>
                <input type="submit" value="Sign Up" />
                    <div className="signup_link">
                    </div>
            </form>
        </div>
    );
}