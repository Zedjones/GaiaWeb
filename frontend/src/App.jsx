import React, {useState} from 'react';
import Bar from "./components/AppBar";
import './App.css';

function App() {
  const [loggedIn, setLoggedIn] = useState(false);
  const [loading, setLoading] = useState(true);
  return (
    <>
      <Bar loggedIn={loggedIn} setLoggedIn={setLoggedIn} setLoading={setLoading} loading={loading}/>
      {
        loading ? "" :
        loggedIn ? "Oh we logged" : "Oop no log"
      }
    </>
  );
}

export default App;