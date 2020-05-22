import React, {useState} from 'react';
import PropTypes from 'prop-types';
import LoginAlert from "./LoginAlert";
import Settings from '../Settings';
import { GoogleLogin, GoogleLogout } from 'react-google-login';

export default function GoogleButton(props) {
  const [accessToken, setAccessToken] = useState('');

  const login = response => {
    setAccessToken(response.Zi.access_token);
    props.setLoggedIn(true);
  };

  const logout = _ => {
    props.setLoggedIn(false);
    setAccessToken('');
  };

  const handleLoginFailure = _ => {
    <LoginAlert severity="error" message="Could not log in"/>
  };
  
  const handleLogoutFailure = _ => {
    <LoginAlert severity="error" message="Could not log out"/>
  };

  return (
    <div>
      { props.loggedIn ?
        <GoogleLogout
          clientId={ Settings.GOOGLE_CLIENT_ID }
          buttonText='Logout'
          onLogoutSuccess={ logout }
          onFailure={ handleLogoutFailure }
        >
        </GoogleLogout>: <GoogleLogin
          clientId={ Settings.GOOGLE_CLIENT_ID }
          buttonText='Login'
          onSuccess={ login }
          onFailure={ handleLoginFailure }
          cookiePolicy={ 'single_host_origin' }
          responseType='code,token'
        />
      }
    </div>
  )
}

GoogleButton.propTypes = {
  loggedIn: PropTypes.string,
  setLoggedIn: PropTypes.func
}