import React from 'react';
import Messenger from '../Messenger';
import MessageList from "../MessageList";
import {Route, BrowserRouter, Routes} from "react-router-dom";
import {RequireAuth} from "../Auth/RequireAuth";
import LoginUser from "../Login/LoginUser";
import {AuthProvider} from "../AuthProvider/AuthProvider";
import SignUpUser from "../SignUp/SignUpUser";

export default function App() {
    return (
  <AuthProvider>
      <div className="App">
          <BrowserRouter>
                  <Routes>
                      <Route path="" element={<RequireAuth><Messenger /></RequireAuth>} />
                      <Route path="/login" element={<LoginUser />} />
                      <Route path="/signup" element={<SignUpUser />} />
                      <Route path="/dia=:id" element={<RequireAuth><MessageList /></RequireAuth>} />
                  </Routes>
          </BrowserRouter>
      </div>
  </AuthProvider>
    );
}