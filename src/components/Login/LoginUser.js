import React from 'react';
import './LoginUser.css';
import {useLocation, useNavigate, Link} from "react-router-dom";
import axios from "axios";
import {useAuth} from "../Auth/useAuth";

export default function LoginUser() {
    const navigate = useNavigate();
    const location = useLocation();
    const {login} = useAuth();

    function handleSubmit(event) {
       event.preventDefault();
       const form = event.target;
       const user = form.username.value;
       const password = form.password.value;

       axios.post("http://localhost:8080/api/sign_in", {username: user, passwd: password}).then(response => {
           login(response.data.token, () => navigate("/", {replace: true}));
       }).catch(error => {
           alert(error.response.data);
           }
       )
    }
    return (
    <div class="center">
        <h1>Login</h1>
        <form onSubmit={handleSubmit}>
            <div class="txt_field">
                <input type="text" name="username" required />
                    <span></span>
                    <label>Username</label>
            </div>
            <div class="txt_field">
                <input type="password" name="password" required />
                    <span></span>
                    <label>Password</label>
            </div>
            <div class="pass">Forgot Password?</div>
            <input type="submit" value="Login" />
                <div class="signup_link">
                    Not a member? <Link to='/signup'>Sign up</Link>
                </div>
        </form>
    </div>
    );
}