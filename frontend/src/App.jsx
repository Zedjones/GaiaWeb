import React from 'react';
import Bar from "./components/AppBar";
import LoginAlert from "./components/LoginAlert";
import './App.css';

function App() {
  return (
    <>
      <Bar />
      <LoginAlert severity={"error"} message={"Test"}/>
    </>
  );
}

export default App;