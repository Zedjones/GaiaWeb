const prod = {
  API_URL_BASE: "",
  GOOGLE_CLIENT_URL: "gaia-web.report", //Placeholder
  GOOGLE_CLIENT_ID: "274234851238-vihli01kc93s0pg1v9biqj6njcft0t80.apps.googleusercontent.com" //Placeholder
};

const dev = {
  API_URL_BASE: "http://localhost:8080",
  GOOGLE_CLIENT_URL: "localhost:3000",
  GOOGLE_CLIENT_ID: "274234851238-vihli01kc93s0pg1v9biqj6njcft0t80.apps.googleusercontent.com"
};

const Settings = process.env.NODE_ENV === 'development' ? dev : prod;

export default Settings;