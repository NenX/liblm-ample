var envs = [
  {
    APP_NAME: '广三',
    APP_KEY: 'gysy',
    API_PREFIX: '/',
    PUBLIC_PATH: '/',
    HOST_URL: 'http://192.168.124.53:3355/',
  }
];
module.exports = envs.find(_ => _.APP_NAME === '广三')

