import React, {useState} from 'react';
import Bar from "./components/AppBar";
import { CircularProgress, Typography, Grid } from "@material-ui/core";
import './App.css';

function App() {
  const [loggedIn, setLoggedIn] = useState(false);
  const [loading, setLoading] = useState(true);

  const loadingContent = () => {
    return (
      <Grid
        container
        spacing={0}
        direction="column"
        alignItems="center"
        justify="center"
        style={{ minHeight: '100vh' }}
      >
        <Grid item xs={3}>
          <CircularProgress />
        </Grid>
      </Grid>
    )
  };

  return (
    <>
      <Bar loggedIn={loggedIn} setLoggedIn={setLoggedIn} setLoading={setLoading} loading={loading}/>
      {
        loading ? 
        loadingContent()
        :
        <Typography>
          {loggedIn ? "Oh we logged" : "Oop no log"}
        </Typography>
      }
    </>
  );
}

export default App;