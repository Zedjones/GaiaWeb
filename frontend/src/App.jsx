import React, {useState} from 'react';
import Bar from "./components/AppBar";
import LoginAlert from "./components/LoginAlert";
import './App.css';
import { Typography } from "@material-ui/core";

function App() {
  const [loggedIn, setLoggedIn] = useState(false);

  return (
    <>
      <Bar />
      <Typography>
        {loggedIn.toString()}
      </Typography>
      <LoginAlert severity={"error"} message={"Test"}/>
    </>
  );
}

export default App;