import React, {useState} from 'react';
import Bar from "./components/AppBar";
import './App.css';

function App() {
  const [loggedIn, setLoggedIn] = useState(false);
  return (
    <>
      <Bar loggedIn={loggedIn} setLoggedIn={setLoggedIn}/>
    </>
  );
}

export default App;