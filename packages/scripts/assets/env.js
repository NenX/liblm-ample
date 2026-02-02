var envs = [
  {
    APP_NAME: '广三',
    APP_KEY: 'gysy',
    API_PREFIX: '/',
    PUBLIC_PATH: '/',
    HOST_URL: 'http://192.168.124.53:3373/',
    LM_Test:'test'
  },
  {
    APP_NAME: '华医',
    APP_KEY: 'hqyy',
    API_PREFIX: '/obis/',
    PUBLIC_PATH: '/obis/',
    HOST_URL: 'http://192.168.124.53:3354/',
  },

];
const target = process.env.VAR_NAME || '广三'
module.exports = envs.find(_ => _.APP_NAME === target)

