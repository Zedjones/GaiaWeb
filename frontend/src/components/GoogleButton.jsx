import React, {useState} from 'react';
import PropTypes from 'prop-types';
import LoginAlert from "./LoginAlert";
import Settings from '../Settings';
import GoogleLogin from "../react-google-login/google-login";
import { GoogleLogout } from 'react-google-login';

export default function GoogleButton(props) {
  const [accessToken, setAccessToken] = useState('');
  const [open, setOpen] = useState(false);
  const [message, setMessage] = useState('');

  const login = response => {
    setAccessToken(response.access_token);
    props.setLoggedIn(true);
  };

  const logout = _ => {
    props.setLoggedIn(false);
    setAccessToken('');
  };

  const handleLoginFailure = _ => {
    setMessage("Could not log in");
  };
  
  const handleLogoutFailure = _ => {
    setMessage("Could not log out");
  };

  return (
    <div>
      { props.loggedIn ?
        <GoogleLogout
          clientId={ Settings.GOOGLE_CLIENT_ID }
          buttonText='Logout'
          onLogoutSuccess={ logout }
          onFailure={ handleLogoutFailure }
        />
        : 
        <GoogleLogin
          clientId={ Settings.GOOGLE_CLIENT_ID }
          buttonText='Login'
          onSuccess={ login }
          onFailure={ handleLoginFailure }
          cookiePolicy={ 'single_host_origin' }
          responseType='code,token'
          isSignedIn={true}
          setLoading={props.setLoading}
        />
      }
      <LoginAlert severity="error" open={open} setOpen={setOpen} message={message}/>
    </div>
  )
}

GoogleButton.propTypes = {
  loggedIn: PropTypes.string,
  setLoggedIn: PropTypes.func,
  setLoading: PropTypes.func
}