import React, {useState} from 'react';
import Bar from "./components/AppBar";
import { Fade, CircularProgress, Typography } from "@material-ui/core";
import './App.css';

function App() {
  const [loggedIn, setLoggedIn] = useState(false);
  const [loading, setLoading] = useState(true);
  return (
    <>
      <Bar loggedIn={loggedIn} setLoggedIn={setLoggedIn} setLoading={setLoading} loading={loading}/>
      {
        loading ? 
        <Fade
          in={loading}
          style={{
            transitionDelay: '0ms',
          }}
          unmountOnExit
        >
          <CircularProgress />
        </Fade>
        :
        <Typography>
          {loggedIn ? "Oh we logged" : "Oop no log"}
        </Typography>
      }
    </>
  );
}

export default App;