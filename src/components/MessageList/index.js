import React, {useEffect, useState} from 'react';
import Compose from '../Compose';
import Toolbar from '../Toolbar';
import ToolbarButton from '../ToolbarButton';
import Message from '../Message';
import moment from 'moment';

import './MessageList.css';
import {useParams} from "react-router-dom";
import axios from "axios";
import {useAuth} from "../Auth/useAuth";



export default function MessageList(props) {
  const [messages, setMessages] = useState([])
    const params = useParams();
  const {user} = useAuth();
  const [MY_USER_ID, setUserId] = useState();
  axios.post("http://localhost:8080/api/decode_jwt", {"token": user}).then(response => {
     setUserId(response.data.user_id);
  });
    if (window.performance) {
        if (performance.navigation.TYPE_RELOAD) {
            sessionStorage.setItem("token", user);
        }}
  const update = () => {
    getMessages();
  }
  useEffect(() => {
      getMessages();
      const intervalId = setInterval(() => {
          getMessages();
      }, 5000);

      return () => clearInterval(intervalId);
  }, []);

  let url = `http://localhost:8080/api/messages/` + params.id;
  const getMessages = () => {
      axios.post(url).then(response => {
          let tempMessages = Object.values(response.data).map(result => {
              return {
                  id: result.id,
                  author: result.sender_id,
                  message: result.content.content,
                  timestamp: result.date_send
              };
          });
          setMessages([...tempMessages])
      })
  }

  const renderMessages = () => {
    let i = 0;
    let messageCount = messages.length;
    let tempMessages = [];
    while (i < messageCount) {
      let previous = messages[i - 1];
      let current = messages[i];
      let next = messages[i + 1];
      let isMine = current.author === MY_USER_ID;
      let currentMoment = moment(current.timestamp);
      let prevBySameAuthor = false;
      let nextBySameAuthor = false;
      let startsSequence = true;
      let endsSequence = true;
      let showTimestamp = true;

      if (previous) {
        let previousMoment = moment(previous.timestamp);
        let previousDuration = moment.duration(currentMoment.diff(previousMoment));
        prevBySameAuthor = previous.author === current.author;
        
        if (prevBySameAuthor && previousDuration.as('hours') < 1) {
          startsSequence = false;
        }

        if (previousDuration.as('hours') < 1) {
          showTimestamp = false;
        }
      }

      if (next) {
        let nextMoment = moment(next.timestamp);
        let nextDuration = moment.duration(nextMoment.diff(currentMoment));
        nextBySameAuthor = next.author === current.author;

        if (nextBySameAuthor && nextDuration.as('hours') < 1) {
          endsSequence = false;
        }
      }

      tempMessages.push(
        <Message
          key={i}
          isMine={isMine}
          startsSequence={startsSequence}
          endsSequence={endsSequence}
          showTimestamp={showTimestamp}
          data={current}
        />
      );

      // Proceed to the next message.
      i += 1;
    }

    return tempMessages;
  }

    return(
      <div className="message-list">
        <Toolbar
          title={params.id}
          rightItems={[
            <ToolbarButton key="info" icon="ion-ios-information-circle-outline" />,
            <ToolbarButton key="video" icon="ion-ios-videocam" />,
            <ToolbarButton key="phone" icon="ion-ios-call" />
          ]}
        />

        <div className="message-list-container">{renderMessages()}</div>

        <Compose item={update} id_chat={params.id} rightItems={[
          <ToolbarButton key="photo" icon="ion-ios-camera" />,
          <ToolbarButton key="image" icon="ion-ios-image" />,
          <ToolbarButton key="audio" icon="ion-ios-mic" />,
          <ToolbarButton key="money" icon="ion-ios-card" />,
          <ToolbarButton key="games" icon="ion-logo-game-controller-b" />,
          <ToolbarButton key="emoji" icon="ion-ios-happy" />
        ]}/>
      </div>
    );
}