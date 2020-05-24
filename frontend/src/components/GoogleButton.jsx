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
    setAccessToken(response.access_token);
    props.setLoggedIn(true);
  };

  const logout = _ => {
    setAccessToken('');
    props.setLoggedIn(false);
  };

  const handleLoginFailure = _ => {
    setMessage("Could not log in");
  };
  
  const handleLogoutFailure = _ => {
    setMessage("Could not log out");
  };

  const { signIn, loaded } = useGoogleLogin({
    onSuccess: login,
    clientId: Settings.GOOGLE_CLIENT_ID,
    onFailure: handleLoginFailure,
    isSignedIn: true,
    responseType: 'code,token',
    cookiePolicy: 'single_host_origin'
  });

  const { signOut } = useGoogleLogout({
    clientId: Settings.GOOGLE_CLIENT_ID,
    onLogoutSuccess: logout,
    onFailure: handleLogoutFailure
  });

  useEffect(() => {
    props.setLoading(!loaded);
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