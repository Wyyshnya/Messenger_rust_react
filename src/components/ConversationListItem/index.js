import React, {useEffect} from 'react';
import shave from 'shave';
import {NavLink} from "react-router-dom";

import './ConversationListItem.css';

export default function ConversationListItem(props) {
  useEffect(() => {
    shave('.conversation-snippet', 30);
  })

    const { photo, name, text, id } = props.data;
    let ssil = "dia=" + id;
    return (
      <NavLink to={ssil}>
          <div className="conversation-list-item">
        <img className="conversation-photo" src={photo} alt="conversation" />
        <div className="conversation-info">
          <h1 className="conversation-title">{name}</h1>
          <p className="conversation-snippet">{text}</p>
        </div>
        </div>
      </NavLink>
    );
}