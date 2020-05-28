import React, {useState, useEffect} from 'react';
import PropTypes from 'prop-types';
import LoginAlert from "./LoginAlert";
import Settings from '../Settings';
import GoogleLogin from "../react-google-login/google-login";
import { GoogleLogout } from 'react-google-login';

export default function GoogleButton(props) {
  const [email, setEmail] = useState('');
  const [open, setOpen] = useState(false);
  const [message, setMessage] = useState('');

  const login = response => {
    setEmail(response.profileObj.email);
    props.setLoggedIn(true);
  };

  const logout = _ => {
    props.setLoggedIn(false);
    setEmail('');
  };

  const handleLoginFailure = _ => {
    setMessage("Could not log in");
  };
  
  const handleLogoutFailure = _ => {
    setMessage("Could not log out");
  };

  useEffect(() => {
    console.log(email);
  })

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
  loggedIn: PropTypes.bool,
  setLoggedIn: PropTypes.func,
  setLoading: PropTypes.func
}