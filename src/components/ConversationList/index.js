import React, {useState, useEffect} from 'react';
import ConversationSearch from '../ConversationSearch';
import ConversationListItem from '../ConversationListItem';
import Toolbar from '../Toolbar';
import ToolbarButton from '../ToolbarButton';
import axios from 'axios';
import './ConversationList.css';
import {useAuth} from "../Auth/useAuth";
import {useNavigate} from "react-router-dom";


export default function ConversationList(props) {
    const navigate = useNavigate();
    const {user, logout} = useAuth();
    if (window.performance) {
        if (performance.navigation.TYPE_RELOAD) {
            sessionStorage.setItem("token", user);
        }}
  const [conversations, setConversations] = useState([]);
  useEffect(() => {
    getConversations()
  },[])

    const handleLogout = () => {
      logout(() => navigate("/login", {replace: true}));
    }
 const getConversations = () => {
     // 'https://randomuser.me/api/?results=20'  http://127.0.0.1:8080/api/conversations/1
     // var xhr = new XMLHttpRequest(); // lower than axios
     //
     // xhr.open('POST', 'http://localhost:8080/api/conversations/1', false);
     // xhr.send();
     //
     // if (xhr.status != 200) {
     //     // обработать ошибку
     //     alert('Ошибка ' + xhr.status + ': ' + xhr.statusText);
     // } else {
     //     // вывести результат
     //     alert(xhr.responseText);
     // }
     let url = `http://localhost:8080/api/conversations/` + user;
    axios.post(url).then(response => {
        let newConversations = Object.values(response.data)[0].map((result, index) => {
            let last_msg = Object.values(response.data)[1][index];
            if (last_msg === "None") {
                last_msg = '!Zero messages!';
            }
          return {
            photo: "https://randomuser.me/api/portraits/men/97.jpg",
            name: result.title,
            text: last_msg,
            id: result.id,
          };
        });
        setConversations([...conversations, ...newConversations])
    });
  }

    return (
      <div className="conversation-list">
        <Toolbar
          title="Aci"
          leftItems={[
              <button className="buttons" onClick={handleLogout}><ToolbarButton key="cog" icon="ion-ios-cog" /></button>
          ]}
          rightItems={[
            <ToolbarButton key="add" icon="ion-ios-add-circle-outline" />
          ]}
        />
        <ConversationSearch />
        {
          conversations.map(conversation =>
            <ConversationListItem
              key={conversation.name}
              data={conversation}
            />
          )
        }
      </div>
    );
}