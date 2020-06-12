import React, { useState } from 'react';
import Bar from "./components/AppBar";
import { CircularProgress, Typography, Grid } from "@material-ui/core";
import GaiaGrid from './components/GaiaGrid';
import './App.css';
import { ApolloProvider } from '@apollo/react-hooks';
import { client } from './index';


export const loadingContent = () => {
  return (
    <Grid
      container
      spacing={0}
      direction="column"
      alignItems="center"
      justify="center"
      style={{ minHeight: '90vh' }}
    >
      <Grid item xs={3}>
        <CircularProgress />
      </Grid>
    </Grid>
  )
};


function App() {
  const [loggedIn, setLoggedIn] = useState(false);
  const [loading, setLoading] = useState(true);
  const [email, setEmail] = useState('');

  return (
    <ApolloProvider client={client}>
      <Bar
        loggedIn={loggedIn}
        setLoggedIn={setLoggedIn}
        setLoading={setLoading}
        loading={loading}
        email={email}
        setEmail={setEmail}
      />
      {
        loading ?
          loadingContent()
          : loggedIn
            ? <GaiaGrid email={email} />
            : <Typography> "Oop no log" </Typography>
      }
    </ApolloProvider>
  );
}

export default App;