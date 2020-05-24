import React, { useState, useEffect } from 'react';
import PropTypes from 'prop-types';
import LoginAlert from "./LoginAlert";
import { Button } from "@material-ui/core";
import Settings from '../Settings';
import { useGoogleLogin, useGoogleLogout } from 'react-google-login';

export default function GoogleButton(props) {
  const [accessToken, setAccessToken] = useState('');
  const [open, setOpen] = useState(false);
  const [message, setMessage] = useState('');

  const login = response => {
    props.setLoggedIn(true);
    props.setLoading(false);
    setAccessToken(response.access_token);
  };

  const logout = _ => {
    props.setLoggedIn(false);
    setAccessToken('');
  };

  const onRequest = _ => {
    props.setLoading(true);
  };

  const handleLoginFailure = _ => {
    setMessage("Could not log in");
  };
  
  const handleLogoutFailure = _ => {
    setMessage("Could not log out");
  };

  const { signIn, signInLoaded } = useGoogleLogin({
    onSuccess: login,
    clientId: Settings.GOOGLE_CLIENT_ID,
    buttonText: "Login",
    onFailure: handleLoginFailure,
    isSignedIn: true,
    responseType: 'code,token',
    onRequest: onRequest,
    cookiePolicy: 'single_host_origin'
  });

  const { signOut, signOutLoaded } = useGoogleLogout({
    clientId: Settings.GOOGLE_CLIENT_ID,
    buttonText: 'Logout',
    onLogoutSuccess: logout,
    onFailure: handleLogoutFailure
  });

  useEffect(() => {
    props.setLoading(!signInLoaded);
  });

  return (
    <div>
      { props.loggedIn ?
        <Button onClick={signOut} color='inherit'> Logout </Button>
        : 
        <Button onClick={signIn} color='inherit'> Login </Button>
      }
      <LoginAlert severity="error" open={open} setOpen={setOpen} message={message}/>
    </div>
  )
}

GoogleButton.propTypes = {
  loggedIn: PropTypes.bool,
  setLoggedIn: PropTypes.func,
  setLoading: PropTypes.func
}