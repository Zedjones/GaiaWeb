import React, { useState } from 'react';
import Bar from "./components/AppBar";
import { CircularProgress, Typography, Grid, Fab, makeStyles } from "@material-ui/core";
import AddIcon from "@material-ui/icons/Add";
import GaiaGrid from './components/GaiaGrid';
import './App.css';
import { ApolloProvider } from '@apollo/react-hooks';
import { client } from './index';

const useStyles = makeStyles((theme) => ({
  fab: {
    position: 'absolute',
    bottom: theme.spacing(2),
    right: theme.spacing(2),
  },
}));


export const LoadingContent = () => {
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
  const classes = useStyles();

  const LoggedIn = () => (
    <>
      <GaiaGrid email={email} />
      <Fab className={classes.fab}>
        <AddIcon />
      </Fab>
    </>
  );

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
          <LoadingContent />
          : loggedIn
            ? <LoggedIn />
            : <Typography> "Oop no log" </Typography>
      }
    </ApolloProvider>
  );
}

export default App;