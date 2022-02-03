import React from 'react';
import './Compose.css';
import axios from "axios";
import {useAuth} from "../Auth/useAuth";

export default function Compose(props) {
    const {user} = useAuth();
    const url = "http://localhost:8080/api/message_post/" + props.id_chat;
    function handleKeyPress(event) {
        if(event.key === 'Enter'){
            const msg = event.target.value.trim();
            if(msg !== ""){
                axios.post(url, {message: msg, id_user: user});
                event.target.value="";
                props.item();
            }
        }
    }
    return (
      <div className="compose">
        <input
          type="text"
          id="one"
          onKeyPress={handleKeyPress}
          className="compose-input"
          placeholder="Write a message..."
        />

        {
          props.rightItems
        }
      </div>
    );
}